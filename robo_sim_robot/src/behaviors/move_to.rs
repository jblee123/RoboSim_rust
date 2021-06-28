use std::any::Any;
use std::cell::RefCell;

use robo_sim_utils::vec3d::*;

use super::behavior::*;

pub struct MoveTo<'a> {
    pub name: String,
    pub cycle: u64,
    pub target_input: &'a RefCell<dyn Behavior>,
    pub output: Vec3d<f32>,
}

impl<'a> MoveTo<'a> {
    pub fn new(name: Option<&str>, target_input: &'a RefCell<dyn Behavior>) -> Self {
        Self {
            name: get_behavior_name(name),
            cycle: 0,
            target_input: target_input,
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

        let mut target_input_mut = self.target_input.borrow_mut();

        self.output = *downcast_input::<Vec3d<f32>>(
            target_input_mut.get_output(cycle),
            "MoveTo",
            "target_input",
        );

        self.output = self.output.to_unit();

        &self.output
    }
}

impl Behavior for MoveTo<'_> {
    fn get_name(&self) -> &str {
        MoveTo::get_name(self)
    }

    fn get_output(&mut self, cycle: u64) -> &dyn Any {
        MoveTo::get_output(self, cycle)
    }
}
