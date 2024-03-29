use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use robo_sim_utils::vec3d::Vec3d;

use super::super::robot_interfaces::robot_interface::*;

use super::behavior::*;

pub struct GetObs {
    pub name: String,
    pub cycle: u64,
    pub robot_interface: Rc<RefCell<dyn RobotInterface>>,
    pub readings: Vec<Vec3d<f32>>,
}

impl GetObs {
    pub fn new(name: Option<&str>, robot_interface: Rc<RefCell<dyn RobotInterface>>) -> Self {
        Self {
            name: get_behavior_name(name),
            cycle: 0,
            robot_interface: robot_interface,
            readings: vec![],
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_output(&mut self, cycle: u64) -> &dyn Any {
        if cycle != self.cycle {
            self.readings = self.robot_interface.borrow().get_obs_readings();
            self.cycle = cycle;
        }

        &self.readings
    }
}

impl Behavior for GetObs {
    fn get_name(&self) -> &str {
        GetObs::get_name(self)
    }

    fn get_output(&mut self, cycle: u64) -> &dyn Any {
        GetObs::get_output(self, cycle)
    }
}
