use std::cell::RefCell;
use std::rc::Rc;

use robo_sim_utils::color::*;
use robo_sim_utils::robot_position::*;
use robo_sim_utils::vec3d::*;

use super::behaviors::behavior::Behavior;
use super::controller::Controller;
use super::robot_comm::RobotComm;
use super::robot_interfaces::robot_interface::*;
use super::robot_interfaces::sim_robot_interface::*;

pub struct Robot {
    comm: Rc<RefCell<RobotComm>>,
    controller: Rc<RefCell<Controller>>,
    robot_interface: Rc<RefCell<dyn RobotInterface>>,
}

impl Robot {
    pub fn new(
        id: u32,
        host: &str,
        robot_type: &str,
        x_pos: f32,
        y_pos: f32,
        z_pos: f32,
        heading_rad: f32,
        color: Color,
        max_vel: f32,
        max_angular_vel: f32,
        radius: f32,
    ) -> Self {
        let comm = Rc::new(RefCell::new(RobotComm::new(host, id)));

        let robot_interface = match robot_type {
            "simulation" => Rc::new(RefCell::new(SimRobotInterface::new(comm.clone()))),
            _ => panic!("Error: robot type '{}' not currently supported", robot_type),
        };

        let pos = RobotPosition::new(Vec3d::new(x_pos, y_pos, z_pos), heading_rad);

        if let Err(err) = comm.borrow_mut().open() {
            panic!("Could not open comms connection: {}", err);
        }

        comm.borrow_mut()
            .send_alive_confirmation(pos, color, max_vel, max_angular_vel, radius);

        let controller = Rc::new(RefCell::new(Controller::new()));

        Self {
            comm: comm,
            controller: controller,
            robot_interface: robot_interface,
        }
    }

    pub fn get_robot_interface(&self) -> Rc<RefCell<dyn RobotInterface>> {
        self.robot_interface.clone()
    }

    pub fn add_behavior(&mut self, behavior: Rc<RefCell<dyn Behavior>>) {
        self.controller.borrow_mut().add_behavior(behavior);
    }

    pub fn run(&mut self) {
        self.controller.borrow_mut().run(self.comm.clone());
    }
}
