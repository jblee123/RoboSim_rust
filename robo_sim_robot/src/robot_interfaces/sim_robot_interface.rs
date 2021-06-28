use std::cell::RefCell;
use std::rc::Rc;

use robo_sim_utils::robot_position::*;

use super::super::robot_comm::*;
use super::robot_interface::*;

pub struct SimRobotInterface {
    pub comm: Rc<RefCell<RobotComm>>,
}

impl SimRobotInterface {
    pub fn new(comm: Rc<RefCell<RobotComm>>) -> Self {
        Self { comm: comm }
    }

    pub fn get_position(&self) -> RobotPosition {
        self.comm.borrow_mut().get_position()
    }

    pub fn cmd_move(&self, x: f32, y: f32) {
        self.comm.borrow().sim_move(x, y)
    }

    pub fn get_obs_readings(&self) -> Vec<(f32, f32)> {
        self.comm.borrow_mut().get_obs()
    }
}

impl RobotInterface for SimRobotInterface {
    fn get_position(&self) -> RobotPosition {
        SimRobotInterface::get_position(self)
    }

    fn cmd_move(&self, x: f32, y: f32) {
        SimRobotInterface::cmd_move(self, x, y)
    }

    fn get_obs_readings(&self) -> Vec<(f32, f32)> {
        SimRobotInterface::get_obs_readings(self)
    }
}
