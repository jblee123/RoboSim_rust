use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

use robo_sim_utils::comms::*;
use robo_sim_utils::messages::*;

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
            //console_addr: format!("{}:{}", host, CONSOLE_PORT),
            host: host.to_string(),
            id: id,
            queued_msgs: Vec::new(),
            sock: None,
            console_addr: None,
        }
    }

    pub fn open(&mut self) -> std::io::Result<()> {
        let my_port = CONSOLE_PORT + self.id as u16;
        let sock_addr = SocketAddr::from(([0, 0, 0, 0], my_port));
        let sock = UdpSocket::bind(sock_addr)?;
        sock.set_nonblocking(true)?;
        self.sock = Some(sock);

        let console_addr_str = format!("{}:{}", self.host, CONSOLE_PORT);
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

        let sock: &UdpSocket = self.sock.as_ref().unwrap();

        // return any queued messages up-front
        if wait_for.is_none() && !self.queued_msgs.is_empty() {
            std::mem::swap(&mut msgs, &mut self.queued_msgs);
        }

        // keep going while there's messages waiting
        loop {
            let mut buf = [0; 1024];
            match sock.recv_from(&mut buf) {
                Ok((received, _)) => println!("received {} bytes {:?}", received, &buf[..received]),
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
}
