use std::any::Any;
use std::cell::RefCell;
use std::f32::consts::PI;

use rand;

use robo_sim_utils::vec3d::*;

use super::behavior::*;

pub struct Wander<'a> {
    pub name: String,
    pub cycle: u64,
    pub persistence_input: &'a RefCell<dyn Behavior>,
    pub same_direction_count: u32,
    pub output: Vec3d<f32>,
}

impl<'a> Wander<'a> {
    pub fn new(name: Option<&str>, persistence_input: &'a RefCell<dyn Behavior>) -> Self {
        Self {
            name: get_behavior_name(name),
            cycle: 0,
            persistence_input: persistence_input,
            same_direction_count: 0,
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

        let mut persistence_input_mut = self.persistence_input.borrow_mut();
        let persistence = *downcast_input::<f32>(
            persistence_input_mut.get_output(cycle),
            "Wander",
            "persistence_input",
        ) as u32;

        if self.same_direction_count >= persistence {
            self.same_direction_count = 0
        }

        if self.same_direction_count == 0 {
            let theta = rand::random::<f32>() * 2.0 * PI;
            self.output = Vec3d::new(1.0, 0.0, 0.0).rotated_z(theta);
        }

        self.same_direction_count += 1;

        &self.output
    }
}

impl Behavior for Wander<'_> {
    fn get_name(&self) -> &str {
        Wander::get_name(self)
    }

    fn get_output(&mut self, cycle: u64) -> &dyn Any {
        Wander::get_output(self, cycle)
    }
}
