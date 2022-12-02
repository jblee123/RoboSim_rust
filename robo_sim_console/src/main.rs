extern crate piston_window;

pub mod console_comms;
pub mod display;
pub mod environment;

use robo_sim_utils::color::*;

use environment as environ;
use piston_window as pw;

fn main() {
    const ENV_WIDTH_M: f32 = 50.0;
    const ENV_HEIGHT_M: f32 = 50.0;
    const ENV_SCALE_PIX_PER_METER: f32 = 15.0;

    let mut env = environ::Environment::new(ENV_WIDTH_M, ENV_HEIGHT_M);
    env.add_obstacle(environ::Obstacle::new(10.0, 10.0, 2.0));
    env.add_obstacle(environ::Obstacle::new(15.0, 15.0, 3.0));
    env.add_wall(environ::Wall::new(15.0, 35.0, 25.0, 35.0));
    env.add_wall(environ::Wall::new(25.0, 35.0, 35.0, 25.0));
    env.add_wall(environ::Wall::new(35.0, 25.0, 35.0, 15.0));
    env.add_object(environ::Object::new(49.0, 49.0, 1.0, Color::RED));

    // env.add_wall(environ::Wall::new(15.0, 15.0, 15.0, 35.0));
    // env.add_wall(environ::Wall::new(15.0, 35.0, 35.0, 35.0));
    // env.add_wall(environ::Wall::new(35.0, 35.0, 35.0, 15.0));
    // env.add_wall(environ::Wall::new(35.0, 15.0, 15.0, 15.0));

    let scale = environ::EnvironmentScale::new(ENV_SCALE_PIX_PER_METER, env.width_m, env.height_m);

    let mut window: pw::PistonWindow = pw::WindowSettings::new("RoboSim", [1200, 1000])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            display::draw_env(&env, &scale, context, graphics);
        });
    }
}
