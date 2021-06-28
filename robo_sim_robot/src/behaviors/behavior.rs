use std::any::Any;

pub fn downcast_input<'a, T: 'static>(
    any: &'a dyn std::any::Any,
    beh_name: &str,
    input_beh_name: &str,
) -> &'a T {
    match any.downcast_ref::<T>() {
        Some(m) => m,
        None => panic!(
            "Downcast {}'s output from Any to {} failed from {}.",
            input_beh_name,
            std::any::type_name::<T>(),
            beh_name,
        ),
    }
}

fn get_next_anon_name() -> String {
    static mut NEXT_ID: u32 = 0;
    let id;
    unsafe {
        id = NEXT_ID;
        NEXT_ID += 1;
    }
    format!("AN_{}", id)
}

pub fn get_behavior_name(name: Option<&str>) -> String {
    match name {
        Some(name) => name.to_string(),
        None => get_next_anon_name(),
    }
}

pub trait Behavior {
    fn get_name(&self) -> &str;
    fn get_output(&mut self, cycle: u64) -> &dyn Any;
}
