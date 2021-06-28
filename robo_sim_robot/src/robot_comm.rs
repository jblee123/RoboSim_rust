use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

use robo_sim_utils::color::*;
use robo_sim_utils::comms;
use robo_sim_utils::messages::*;
use robo_sim_utils::robot_position::*;

fn downcast<'a, T: 'static>(msg: &'a Box<dyn Message>, fn_name: &str) -> &'a T {
    match msg.as_any().downcast_ref::<T>() {
        Some(m) => m,
        None => panic!(
            "Downcast from Message to {} failed in {}().",
            std::any::type_name::<T>(),
            fn_name
        ),
    }
}

fn downcast_mut<'a, T: 'static>(msg: &'a mut Box<dyn Message>, fn_name: &str) -> &'a mut T {
    match msg.as_any_mut().downcast_mut::<T>() {
        Some(m) => m,
        None => panic!(
            "Downcast from Message to {} failed in {}().",
            std::any::type_name::<T>(),
            fn_name
        ),
    }
}

//#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct RobotComm {
    pub host: String,
    pub id: u32,
    pub queued_msgs: Vec<Box<dyn Message>>,
    pub sock: Option<UdpSocket>,
    pub console_addr: Option<SocketAddr>,
}

impl RobotComm {
    pub fn new(host: &str, id: u32) -> Self {
        Self {
            host: host.to_string(),
            id: id,
            queued_msgs: Vec::new(),
            sock: None,
            console_addr: None,
        }
    }

    pub fn open(&mut self) -> std::io::Result<()> {
        let my_port = comms::CONSOLE_PORT + self.id as u16;
        let sock_addr = SocketAddr::from(([0, 0, 0, 0], my_port));
        let sock = UdpSocket::bind(sock_addr)?;
        sock.set_nonblocking(true)?;
        self.sock = Some(sock);

        let console_addr_str = format!("{}:{}", self.host, comms::CONSOLE_PORT);
        let console_addr = console_addr_str.to_socket_addrs()?.next().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Could not convert '{}' to sockaddr.", console_addr_str),
            )
        })?;
        self.console_addr = Some(console_addr);

        Ok(())
    }

    pub fn check_msgs(&mut self, wait_for: Option<MessageType>) -> Vec<Box<dyn Message>> {
        let mut msgs: Vec<Box<dyn Message>> = vec![];

        // make sure we the socket has been opened
        if self.sock.is_none() {
            return msgs;
        }

        let sock = self.sock.as_ref().unwrap();

        // return any queued messages up-front
        if wait_for.is_none() && !self.queued_msgs.is_empty() {
            msgs = std::mem::take(&mut self.queued_msgs);
        }

        // keep going while there's messages waiting
        loop {
            let mut buf = [0; 1024];
            match sock.recv_from(&mut buf) {
                Ok((received, _)) => {
                    let msg = comms::parse_message(&buf[..received]).unwrap();
                    if wait_for == Some(msg.get_type()) {
                        msgs.push(msg);
                        return msgs;
                    } else if wait_for.is_some() {
                        self.queued_msgs.push(msg);
                    } else {
                        msgs.push(msg);
                    }
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::WouldBlock {
                        break;
                    } else {
                        panic!("recv function failed: {:?}", e);
                    }
                }
            }
        }

        msgs
    }

    pub fn wait_for_msg(&mut self, wait_for: MessageType) -> Box<dyn Message> {
        let sleep_time = std::time::Duration::from_millis(10);
        loop {
            let mut msgs = self.check_msgs(Some(wait_for));
            if !msgs.is_empty() {
                return msgs.pop().unwrap();
            }
            std::thread::sleep(sleep_time);
        }
    }

    fn send_msg(&self, msg_buf: &[u8]) {
        let sock = self.sock.as_ref().unwrap();
        let console_addr = self.console_addr.as_ref().unwrap();
        match sock.send_to(msg_buf, console_addr) {
            Err(e) => {
                panic!("send function failed: {:?}", e);
            }
            _ => {}
        }
    }

    pub fn send_alive_confirmation(
        &self,
        pos: RobotPosition,
        color: Color,
        max_vel: f32,
        max_angular_vel: f32,
        radius: f32,
    ) {
        if self.console_addr.is_none() {
            return;
        }

        let msg = AliveMsg::new(self.id, pos, color, max_vel, max_angular_vel, radius);
        let msg_buf = comms::pack_alive_message(msg);
        self.send_msg(msg_buf.as_slice());
    }

    pub fn send_position_update(&self, pos: RobotPosition) {
        let msg = PositionMsg::new(pos);
        let msg_buf = comms::pack_position_message(msg);
        self.send_msg(msg_buf.as_slice());
    }

    pub fn get_position(&mut self) -> RobotPosition {
        let req_msg = RequestPositionMsg::new(self.id);
        let req_msg_buf = comms::pack_request_position_message(req_msg);
        self.send_msg(req_msg_buf.as_slice());
        let reply_msg = self.wait_for_msg(MessageType::Position);
        let pos_msg = downcast::<PositionMsg>(&reply_msg, "get_position");
        pos_msg.pos
    }

    pub fn send_death_msg(&self) {
        let msg = RobotDyingMsg::new(self.id);
        let msg_buf = comms::pack_robot_dying_message(msg);
        self.send_msg(msg_buf.as_slice());
    }

    pub fn get_obs(&mut self) -> Vec<(f32, f32)> {
        let req_msg = GetObstaclesMsg::new(self.id);
        let req_msg_buf = comms::pack_get_obstacles_message(req_msg);
        self.send_msg(req_msg_buf.as_slice());
        let mut reply_msg = self.wait_for_msg(MessageType::ObsReadings);
        let obs_msg = downcast_mut::<ObsReadingsMsg>(&mut reply_msg, "get_obs");

        std::mem::take(&mut obs_msg.readings)
    }

    pub fn sim_move(&self, x: f32, y: f32) {
        let msg = MoveMsg::new(self.id, x, y);
        let msg_buf = comms::pack_move_message(msg);
        self.send_msg(msg_buf.as_slice());
    }

    pub fn sim_spin(&self, theta: f32) {
        let msg = SpinMsg::new(self.id, theta);
        let msg_buf = comms::pack_spin_message(msg);
        self.send_msg(msg_buf.as_slice());
    }
}
