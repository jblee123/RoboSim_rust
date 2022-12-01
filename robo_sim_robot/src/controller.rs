use std::cell::RefCell;
use std::rc::Rc;

use robo_sim_utils::messages::*;

use super::behaviors::behavior::Behavior;
use super::robot_comm::RobotComm;

pub struct Controller {
    behaviors: Vec<Rc<RefCell<dyn Behavior>>>,
    paused: bool,
    cycle: u64,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            behaviors: Vec::new(),
            paused: true,
            cycle: 0,
        }
    }

    pub fn add_behavior(&mut self, behavior: Rc<RefCell<dyn Behavior>>) {
        self.behaviors.push(behavior);
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    pub fn run(&mut self, comm: Rc<RefCell<RobotComm>>) {
        loop {
            let got_kill = self.handle_msgs(&mut comm.borrow_mut());
            if got_kill {
                break;
            }

            if self.paused {
                continue;
            }

            // run all the top-level behaviors
            for behavior in &self.behaviors {
                behavior.borrow_mut().get_output(self.cycle);
            }

            self.cycle += 1;
        }
    }

    fn handle_msgs(&mut self, comm: &mut RobotComm) -> bool {
        let mut got_kill = false;
        let msgs = comm.check_msgs(None);
        for msg in msgs {
            match msg.get_type() {
                MessageType::Start => self.handle_start(),
                MessageType::Kill => {
                    self.handle_kill(comm);
                    got_kill = true;
                    break;
                }
                MessageType::Pause => self.handle_pause(),
                _ => println!("unhandled message type: {}", msg.get_type() as i32),
            }
        }

        got_kill
    }

    fn handle_start(&mut self) {
        self.paused = false;
    }

    fn handle_kill(&mut self, comm: &mut RobotComm) {
        comm.send_death_msg();
    }

    fn handle_pause(&mut self) {
        self.paused = !self.paused;
    }
}
