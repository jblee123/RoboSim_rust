use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use robo_sim_utils::robot_position::*;

use super::super::robot_interfaces::robot_interface::*;

use super::behavior::*;

pub struct GetPosition {
    pub name: String,
    pub cycle: u64,
    pub robot_interface: Rc<RefCell<dyn RobotInterface>>,
    pub pos: RobotPosition,
}

impl GetPosition {
    pub fn new(name: Option<&str>, robot_interface: Rc<RefCell<dyn RobotInterface>>) -> Self {
        Self {
            name: get_behavior_name(name),
            cycle: 0,
            robot_interface: robot_interface,
            pos: Default::default(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_output(&mut self, cycle: u64) -> &dyn Any {
        if cycle != self.cycle {
            self.pos = self.robot_interface.borrow().get_position();
            self.cycle = cycle;
        }

        &self.pos
    }
}

impl Behavior for GetPosition {
    fn get_name(&self) -> &str {
        GetPosition::get_name(self)
    }

    fn get_output(&mut self, cycle: u64) -> &dyn Any {
        GetPosition::get_output(self, cycle)
    }
}
