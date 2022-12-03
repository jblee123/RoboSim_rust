use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};

use robo_sim_utils::comms;
use robo_sim_utils::messages::*;
use robo_sim_utils::robot_position::*;
use robo_sim_utils::vec3d::*;

pub struct ConsoleComms {
    sock: Option<UdpSocket>,
    addresses: HashMap<u32, SocketAddr>,
}

impl ConsoleComms {
    pub fn new() -> Self {
        Self {
            sock: None,
            addresses: HashMap::new(),
        }
    }

    pub fn open(&mut self) -> std::io::Result<()> {
        let sock_addr = SocketAddr::from(([0, 0, 0, 0], comms::CONSOLE_PORT));
        let sock = UdpSocket::bind(sock_addr)?;
        sock.set_nonblocking(true)?;
        self.sock = Some(sock);

        Ok(())
    }

    pub fn check_for_msgs(&mut self) -> Vec<Box<dyn Message>> {
        let mut msgs: Vec<Box<dyn Message>> = vec![];

        // make sure we the socket has been opened
        if self.sock.is_none() {
            return msgs;
        }

        // keep going while there are messages waiting
        loop {
            let mut buf = [0; 1024];
            let msg_result = { self.sock.as_ref().unwrap().recv_from(&mut buf) };
            match msg_result {
                Ok((received, addr)) => {
                    let msg = comms::parse_message(&buf[..received]).unwrap();
                    if msg.get_type() == MessageType::Alive {
                        self.register_new_robot(&msg, addr);
                    }
                    msgs.push(msg);
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

    fn send_msg(&self, id: u32, msg_buf: &[u8]) {
        let sock = self.sock.as_ref().unwrap();
        let addr_result = self.addresses.get(&id);
        if addr_result.is_none() {
            panic!("uknown robot ID: {}", id);
        }

        if let Err(err) = sock.send_to(msg_buf, addr_result.unwrap()) {
            panic!("send function failed: {:?}", err);
        }
    }

    fn register_new_robot(&mut self, msg: &Box<dyn Message>, addr: SocketAddr) {
        let alive_msg = downcast::<AliveMsg>(msg, "alive");
        if self.addresses.contains_key(&alive_msg.id) {
            panic!("ID {} is being re-used.", alive_msg.id);
        }

        self.addresses.insert(alive_msg.id, addr);
        println!("registered address for ID {}", alive_msg.id);
    }

    pub fn unregister_robot(&mut self, id: u32) {
        self.addresses.remove(&id);
    }

    pub fn send_start_msg(&self, id: u32) {
        let msg = StartMsg::new();
        let msg_buf = comms::pack_start_message(msg);
        self.send_msg(id, &msg_buf);
    }

    pub fn send_pause_msg(&self) {
        let msg = PauseMsg::new();
        let msg_buf = comms::pack_pause_message(msg);
        for id in self.addresses.keys() {
            self.send_msg(*id, &msg_buf);
        }
    }

    pub fn send_position_msg(&self, id: u32, pos: RobotPosition) {
        let msg = PositionMsg::new(id, pos);
        let msg_buf = comms::pack_position_message(msg);
        self.send_msg(id, &msg_buf);
    }

    pub fn send_obs_readings_msg(&self, id: u32, obs_readings: Vec<Vec3d<f32>>) {
        let msg = ObsReadingsMsg::new(obs_readings);
        let msg_buf = comms::pack_obs_readings_message(msg);
        self.send_msg(id, &msg_buf);
    }
}
