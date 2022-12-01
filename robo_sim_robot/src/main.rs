extern crate getopts;

use std::cell::RefCell;
use std::env;
use std::rc::Rc;

use getopts::Options;

use robo_sim_utils::color::Color;

use behaviors::test_goto::TestGoto;
use robot::Robot;

pub mod controller;
pub mod robot;
pub mod robot_comm;

pub mod robot_interfaces {
    pub mod robot_interface;
    pub mod sim_robot_interface;
}

pub mod behaviors {
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

// fn print_usage(program: &str, opts: Options) {
//     let brief = format!("Usage: {} [options]", program);
//     print!("{}", opts.usage(&brief));
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    // let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("i", "", "set robot ID", "ID");
    opts.optopt("h", "", "console host", "HOST");
    opts.optopt("T", "", "robot type", "TYPE");
    opts.optopt("x", "", "x-pos", "Y_POS");
    opts.optopt("y", "", "y-pos", "X_POS");
    opts.optopt("d", "", "direction", "DIRECTION");
    opts.optopt("c", "", "color", "COLOR");
    opts.optopt("v", "", "max velocity", "MAX VEL");
    opts.optopt("a", "", "max angular velocity", "MAX ANGULAR VEL");
    opts.optopt("r", "", "radius", "RADIUS");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    let mut id = 1u32;
    let mut host = String::from("localhost");
    let mut robot_type = String::from("simulation");
    let mut x_pos = 0f32;
    let mut y_pos = 0f32;
    let mut heading = 0f32;
    let mut color = Color::BLACK;
    let mut max_vel = 0f32;
    let mut max_angular_vel = 0f32;
    let mut radius = 0f32;

    if let Some(id_opt) = matches.opt_str("i") {
        id = id_opt.parse::<u32>().unwrap(); // ok to panic
    }

    if let Some(host_opt) = matches.opt_str("h") {
        host = host_opt;
    }

    if let Some(robot_type_opt) = matches.opt_str("T") {
        robot_type = robot_type_opt;
    }

    if let Some(x_pos_opt) = matches.opt_str("T") {
        x_pos = x_pos_opt.parse::<f32>().unwrap(); // ok to panic
    }

    if let Some(y_pos_opt) = matches.opt_str("T") {
        y_pos = y_pos_opt.parse::<f32>().unwrap(); // ok to panic
    }

    if let Some(heading_opt) = matches.opt_str("T") {
        heading = heading_opt.parse::<f32>().unwrap(); // ok to panic
    }

    if let Some(color_opt) = matches.opt_str("c") {
        color = Color::from_str(&color_opt);
    }

    if let Some(max_vel_opt) = matches.opt_str("v") {
        max_vel = max_vel_opt.parse::<f32>().unwrap(); // ok to panic
    }

    if let Some(max_angular_vel_opt) = matches.opt_str("a") {
        max_angular_vel = max_angular_vel_opt.parse::<f32>().unwrap(); // ok to panic
    }

    if let Some(radius_opt) = matches.opt_str("r") {
        radius = radius_opt.parse::<f32>().unwrap(); // ok to panic
    }

    let mut r = Robot::new(
        id,
        &host,
        &robot_type,
        x_pos,
        y_pos,
        0f32,
        heading,
        color,
        max_vel,
        max_angular_vel,
        radius,
    );

    let test_goto = Rc::new(RefCell::new(TestGoto::new(
        Some("TestGoto"),
        r.get_robot_interface(),
    )));

    r.add_behavior(test_goto);
    r.run();

    // let l1 = behaviors::literal::LiteralF32::new(Some("a"), 1.0);
    // let l2 = behaviors::literal::LiteralF32::new(None, 1.0);
    // let l3 = behaviors::literal::LiteralF32::new(None, 1.0);
    // println!("l1: name='{}'", l1.get_name());
    // println!("l2: name='{}'", l2.get_name());
    // println!("l3: name='{}'", l3.get_name());

    // let c1 = std::rc::Rc::new(std::cell::RefCell::new(1));
    // let c2 = c1.clone();

    // println!("c1 val: {}", *c1.borrow());
    // println!("c2 val: {}", *c2.borrow());

    // {
    //     *c2.borrow_mut() = 2;
    // }

    // println!("c1 val: {}", *c1.borrow());
    // println!("c2 val: {}", *c2.borrow());
}
