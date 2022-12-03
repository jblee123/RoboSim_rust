extern crate piston_window;

pub mod console_comms;
pub mod display;
pub mod environment;
pub mod simulator;

use std::env;
use std::process::Command;

use robo_sim_utils::color::*;
use robo_sim_utils::messages::*;

use environment as environ;
use piston_window as pw;

fn start_robot(id: u32) {
    let exec_name = match env::consts::OS {
        "windows" => "robo_sim_robot.exe",
        _ => panic!("Unsupported OS: {}", env::consts::OS),
    };

    Command::new(&exec_name)
        .args(&[
            "-i", &id.to_string(), "-x", "1", "-y", "1", "-c", "blue", "-v", "1", "-a", "20",
        ])
        .spawn()
        .expect("failed to execute process");
}

fn handle_msgs(
    msgs: &Vec<Box<dyn Message>>,
    simulator: &mut simulator::Simulator,
    env: &mut environ::Environment,
    comms: &console_comms::ConsoleComms,
) {
    for msg in msgs {
        match msg.get_type() {
            MessageType::Alive => {
                handle_alive_msg(msg, simulator, env, comms);
            }
            MessageType::Position => {
                handle_position_msg(msg, simulator, env);
            }
            MessageType::RequestPosition => {
                handle_request_position_msg(msg, simulator, comms);
            }
            MessageType::GetObstacles => {
                handle_get_obstacles_msg(msg, simulator, env, comms);
            }
            MessageType::RobotDying => {
                println!("got msg type: {:?}", msg.get_type());
            }
            MessageType::Move => {
                handle_move_msg(msg, simulator, env);
            }
            MessageType::Spin => {
                println!("got msg type: {:?}", msg.get_type());
            }
            _ => println!("got unsupported msg type: {:?}", msg.get_type()),
        }
    }
}

fn handle_alive_msg(
    msg: &Box<dyn Message>,
    simulator: &mut simulator::Simulator,
    env: &mut environ::Environment,
    comms: &console_comms::ConsoleComms,
) {
    let msg = downcast::<AliveMsg>(msg, "alive");
    println!("alive msg: {:?}", msg);
    simulator.register_robot(
        msg.id,
        msg.pos,
        msg.color,
        msg.max_vel,
        msg.max_angular_vel,
        msg.radius,
        env,
        comms,
    );
}

fn handle_position_msg(
    msg: &Box<dyn Message>,
    simulator: &mut simulator::Simulator,
    env: &mut environ::Environment,
) {
    let msg = downcast::<PositionMsg>(msg, "position");
    simulator.update_robot_pos(msg.id, msg.pos, env);
}

fn handle_request_position_msg(
    msg: &Box<dyn Message>,
    simulator: &mut simulator::Simulator,
    comms: &console_comms::ConsoleComms,
) {
    let msg = downcast::<RequestPositionMsg>(msg, "request_position");
    let pos = simulator.get_robot_pos(msg.id);
    if pos.is_none() {
        println!("no pos for requesting ID: {}", msg.id);
        return;
    }

    comms.send_position_msg(msg.id, pos.unwrap());
}

fn handle_get_obstacles_msg(
    msg: &Box<dyn Message>,
    simulator: &mut simulator::Simulator,
    env: &mut environ::Environment,
    comms: &console_comms::ConsoleComms,
) {
    let msg = downcast::<GetObstaclesMsg>(msg, "get_obstacles");
    let obs_readings = simulator.get_obs_readings(msg.id, env);
    comms.send_obs_readings_msg(msg.id, obs_readings);
}

fn handle_move_msg(
    msg: &Box<dyn Message>,
    simulator: &mut simulator::Simulator,
    env: &mut environ::Environment,
) {
    let msg = downcast::<MoveMsg>(msg, "move");
    simulator.move_robot(msg.id, msg.x, msg.y, env);
}

fn main() {
    let mut simulator = simulator::Simulator::new(0.2f32);
    let mut comms = console_comms::ConsoleComms::new();
    comms.open().expect("could not open comms");

    const ENV_WIDTH_M: f32 = 50.0;
    const ENV_HEIGHT_M: f32 = 50.0;
    const ENV_SCALE_PIX_PER_METER: f32 = 20.0;

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

    start_robot(1);

    let mut window: pw::PistonWindow = pw::WindowSettings::new("RoboSim", [1200, 1000])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {

        if let pw::Event::Input(pw::Input::Button(btn_args), _) = event {
            if btn_args.state == pw::ButtonState::Press {
                if let pw::Button::Keyboard(key) = btn_args.button {
                    match key {
                        pw::Key::P => { comms.send_pause_msg(); },
                        _ => {},
                    }
                }
            }
        }

        window.draw_2d(&event, |context, graphics, _device| {
            let msgs = comms.check_for_msgs();
            handle_msgs(&msgs, &mut simulator, &mut env, &mut comms);

            display::draw_env(&env, &scale, context, graphics);
        });
    }
}
