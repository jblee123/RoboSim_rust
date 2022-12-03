use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use robo_sim_utils::vec3d::*;

use super::behavior::*;

pub struct AvoidObs {
    pub name: String,
    pub cycle: u64,
    pub obs_list_input: Rc<RefCell<dyn Behavior>>,
    pub safety_margin_input: Rc<RefCell<dyn Behavior>>,
    pub sphere_of_influence_input: Rc<RefCell<dyn Behavior>>,
    pub output: Vec3d<f32>,
}

impl<'a> AvoidObs {
    pub fn new(
        name: Option<&str>,
        obs_list_input: Rc<RefCell<dyn Behavior>>,
        safety_margin_input: Rc<RefCell<dyn Behavior>>,
        sphere_of_influence_input: Rc<RefCell<dyn Behavior>>,
    ) -> Self {
        Self {
            name: get_behavior_name(name),
            cycle: 0,
            obs_list_input: obs_list_input,
            safety_margin_input: safety_margin_input,
            sphere_of_influence_input: sphere_of_influence_input,
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

        let mut obs_list_input_mut = self.obs_list_input.borrow_mut();
        let mut safety_margin_input_mut = self.safety_margin_input.borrow_mut();
        let mut sphere_of_influence_input_mut = self.sphere_of_influence_input.borrow_mut();

        let obstacles = downcast_input::<Vec<Vec3d<f32>>>(
            obs_list_input_mut.get_output(cycle),
            "AvoidObs",
            "obs_list_input",
        );

        let safety_margin = *downcast_input::<f32>(
            safety_margin_input_mut.get_output(cycle),
            "AvoidObs",
            "safety_margin_input",
        );

        let sphere_of_influence = *downcast_input::<f32>(
            sphere_of_influence_input_mut.get_output(cycle),
            "AvoidObs",
            "sphere_of_influence_input",
        );

        self.output = Vec3d::new(0.0, 0.0, 0.0);
        for obs in obstacles {
            let mut obs = Vec3d::new(obs.x, obs.y, 0.0);
            let length = obs.len();
            if length < sphere_of_influence {
                if length < safety_margin {
                    obs *= 100_000.0;
                }
                self.output += -obs;
            }
        }

        &self.output
    }
}

impl Behavior for AvoidObs {
    fn get_name(&self) -> &str {
        AvoidObs::get_name(self)
    }

    fn get_output(&mut self, cycle: u64) -> &dyn Any {
        AvoidObs::get_output(self, cycle)
    }
}
