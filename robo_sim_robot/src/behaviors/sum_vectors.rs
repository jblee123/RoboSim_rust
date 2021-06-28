use std::any::Any;
use std::cell::RefCell;

use robo_sim_utils::vec3d::*;

use super::behavior::*;

pub struct SumVectors<'a> {
    pub name: String,
    pub cycle: u64,
    pub vectors_input: Vec<&'a RefCell<dyn Behavior>>,
    pub weights_input: Vec<&'a RefCell<dyn Behavior>>,
    pub output: Vec3d<f32>,
}

impl<'a> SumVectors<'a> {
    pub fn new(
        name: Option<&str>,
        vectors_input: Vec<&'a RefCell<dyn Behavior>>,
        weights_input: Vec<&'a RefCell<dyn Behavior>>,
    ) -> Self {
        Self {
            name: get_behavior_name(name),
            cycle: 0,
            vectors_input: vectors_input,
            weights_input: weights_input,
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

        self.output = Default::default();
        let vector_weight_pairs = self.vectors_input.iter().zip(self.weights_input.iter());
        for (vector_input, weight_input) in vector_weight_pairs {
            let mut vector_input_mut = vector_input.borrow_mut();
            let mut weight_input_mut = weight_input.borrow_mut();

            let vector = *downcast_input::<Vec3d<f32>>(
                vector_input_mut.get_output(cycle),
                "SumVectors",
                "vector_input",
            );
            let weight = *downcast_input::<f32>(
                weight_input_mut.get_output(cycle),
                "SumVectors",
                "weight_input",
            );
            self.output += vector * weight;
        }

        &self.output
    }
}

impl Behavior for SumVectors<'_> {
    fn get_name(&self) -> &str {
        SumVectors::get_name(self)
    }

    fn get_output(&mut self, cycle: u64) -> &dyn Any {
        SumVectors::get_output(self, cycle)
    }
}
