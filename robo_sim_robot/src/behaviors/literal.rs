use std::any::Any;

use super::behavior::*;

pub struct LiteralF32 {
    pub name: String,
    pub value: f32,
}

impl LiteralF32 {
    pub fn new(name: Option<&str>, value: f32) -> Self {
        Self {
            name: get_behavior_name(name),
            value: value,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_output(&mut self, _cycle: u64) -> &dyn Any {
        &self.value
    }
}

impl Behavior for LiteralF32 {
    fn get_name(&self) -> &str {
        LiteralF32::get_name(self)
    }

    fn get_output(&mut self, cycle: u64) -> &dyn Any {
        LiteralF32::get_output(self, cycle)
    }
}
