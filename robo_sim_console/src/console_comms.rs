use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};

use robo_sim_utils::messages::*;

use robo_sim_utils::comms;

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

pub struct ConsoleComms {
    pub sock: Option<UdpSocket>,
    pub addresses: HashMap<u32, SocketAddr>,

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
            let msg_result = {
            	self.sock.as_ref().unwrap().recv_from(&mut buf)
            };
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

    fn register_new_robot(&mut self, msg: &Box<dyn Message>, addr: SocketAddr) {
    	let alive_msg = downcast::<AliveMsg>(msg, "alive");
    	if self.addresses.contains_key(&alive_msg.id) {
    		panic!("ID {} is being re-used.", alive_msg.id);
    	}

    	self.addresses.insert(alive_msg.id, addr);
    	println!("registered address for ID {}", alive_msg.id);
    }

    pub fn unregister_robot(&mut self, robot_id: u32) {
    	self.addresses.remove(&robot_id);
    }
}
