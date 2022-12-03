use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use robo_sim_utils::vec3d::*;

use robo_sim_utils::robot_position::*;

use super::behavior::*;

pub struct GlobalToEgocentric {
    pub name: String,
    pub cycle: u64,
    pub robot_pos_input: Rc<RefCell<dyn Behavior>>,
    pub global_pos_input: Rc<RefCell<dyn Behavior>>,
    pub output: Vec3d<f32>,
}

impl GlobalToEgocentric {
    pub fn new(
        name: Option<&str>,
        robot_pos_input: Rc<RefCell<dyn Behavior>>,
        global_pos_input: Rc<RefCell<dyn Behavior>>,
    ) -> Self {
        Self {
            name: get_behavior_name(name),
            cycle: 0,
            robot_pos_input: robot_pos_input,
            global_pos_input: global_pos_input,
            output: Default::default(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_output(&mut self, cycle: u64) -> &dyn Any {
        if cycle == self.cycle {
            return &self.output;
        }
        self.cycle = cycle;

        let mut robot_pos_input_mut = self.robot_pos_input.borrow_mut();
        let mut global_pos_input_mut = self.global_pos_input.borrow_mut();

        let robot_pos = *downcast_input::<RobotPosition>(
            robot_pos_input_mut.get_output(cycle),
            "GlobalToEgocentric",
            "robot_pos_input",
        );

        let global_pos = *downcast_input::<Vec3d<f32>>(
            global_pos_input_mut.get_output(cycle),
            "GlobalToEgocentric",
            "global_pos_input",
        );

        self.output = global_pos - robot_pos.location;
        self.output = self.output.rotated_z(-robot_pos.heading_rad);
        &self.output
    }
}

impl Behavior for GlobalToEgocentric {
    fn get_name(&self) -> &str {
        GlobalToEgocentric::get_name(self)
    }

    fn get_output(&mut self, cycle: u64) -> &dyn Any {
        GlobalToEgocentric::get_output(self, cycle)
    }
}
