use std::any::Any;
use std::cell::RefCell;

use robo_sim_utils::vec3d::*;

use super::super::robot_interfaces::robot_interface::*;

use super::behavior::*;

pub struct TestGoto<'a> {
    pub name: String,
    pub cycle: u64,
    pub robot_interface: &'a dyn RobotInterface,
}

impl<'a> TestGoto<'a> {
    pub fn new(
        name: Option<&str>,
        robot_interface: &'a dyn RobotInterface,
    ) -> Self {
        Self {
            name: get_behavior_name(name),
            cycle: 0,
            robot_interface: robot_interface,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_output(&mut self, cycle: u64) -> &dyn Any {
        const DEFAULT_OUTPUT: i32 = 0;

        if cycle == self.cycle {
            return &DEFAULT_OUTPUT;
        }
        self.cycle = cycle;

        // let mut movement_input_mut = self.movement_input.borrow_mut();
        // let mut base_speed_input_mut = self.base_speed_input.borrow_mut();
        // let mut max_speed_input_mut = self.max_speed_input.borrow_mut();

        // let movement = *downcast_input::<Vec3d<f32>>(
        //     movement_input_mut.get_output(cycle),
        //     "TestGoto",
        //     "movement_input",
        // );

        // let base_speed = *downcast_input::<f32>(
        //     base_speed_input_mut.get_output(cycle),
        //     "MoveRobot",
        //     "base_speed_input",
        // );

        // let max_speed = *downcast_input::<f32>(
        //     max_speed_input_mut.get_output(cycle),
        //     "MoveRobot",
        //     "max_speed_input",
        // );

        // let mut move_cmd = movement * base_speed;
        // if move_cmd.len() > max_speed {
        //     move_cmd = move_cmd.to_unit() * max_speed;
        // }

        // self.robot_interface.cmd_move(move_cmd.x, move_cmd.y);

        &DEFAULT_OUTPUT
    }
}

impl Behavior for TestGoto<'_> {
    fn get_name(&self) -> &str {
        TestGoto::get_name(self)
    }

    fn get_output(&mut self, cycle: u64) -> &dyn Any {
        TestGoto::get_output(self, cycle)
    }
}
