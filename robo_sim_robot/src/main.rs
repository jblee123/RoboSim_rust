pub mod robot_comm;

mod robot_interfaces {
    pub mod robot_interface;
    pub mod sim_robot_interface;
}

mod behaviors {
    pub mod avoid_obs;
    pub mod behavior;
    pub mod get_obs;
    pub mod get_position;
    pub mod global_to_egocentric;
    pub mod literal;
    pub mod move_robot;
    pub mod move_to;
    pub mod sum_vectors;
    pub mod test_goto;
    pub mod wander;
}

fn main() {
    let l1 = behaviors::literal::LiteralF32::new(Some("a"), 1.0);
    let l2 = behaviors::literal::LiteralF32::new(None, 1.0);
    let l3 = behaviors::literal::LiteralF32::new(None, 1.0);
    println!("l1: name='{}'", l1.get_name());
    println!("l2: name='{}'", l2.get_name());
    println!("l3: name='{}'", l3.get_name());

    let c1 = std::rc::Rc::new(std::cell::RefCell::new(1));
    let c2 = c1.clone();

    println!("c1 val: {}", *c1.borrow());
    println!("c2 val: {}", *c2.borrow());

    {
        *c2.borrow_mut() = 2;
    }

    println!("c1 val: {}", *c1.borrow());
    println!("c2 val: {}", *c2.borrow());
}
