use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use robo_sim_utils::vec3d::*;

use super::super::robot_interfaces::robot_interface::*;

use super::avoid_obs::*;
use super::behavior::*;
use super::get_obs::*;
use super::get_position::*;
use super::global_to_egocentric::*;
use super::literal::*;
use super::move_robot::*;
use super::move_to::*;
use super::sum_vectors::*;
use super::wander::*;

pub struct TestGoto {
    pub name: String,
    pub cycle: u64,
    pub robot_interface: Rc<RefCell<dyn RobotInterface>>,

    move_robot: MoveRobot,
}

impl TestGoto {
    pub fn new(name: Option<&str>, robot_interface: Rc<RefCell<dyn RobotInterface>>) -> Self {
        let get_pos = Rc::new(RefCell::new(GetPosition::new(
            None,
            robot_interface.clone(),
        )));

        let global_target_pos = Rc::new(RefCell::new(LiteralVec3dF32::new(
            None,
            Vec3d::<f32>::new(49.0, 49.0, 0.0),
        )));

        let move_to = Rc::new(RefCell::new(MoveTo::new(
            None,
            Rc::new(RefCell::new(GlobalToEgocentric::new(
                None,
                get_pos,
                global_target_pos,
            ))),
        )));

        let get_obs = Rc::new(RefCell::new(GetObs::new(None, robot_interface.clone())));

        let avoid_obs = Rc::new(RefCell::new(AvoidObs::new(
            None,
            get_obs,
            Rc::new(RefCell::new(LiteralF32::new(None, 1.5f32))),
            Rc::new(RefCell::new(LiteralF32::new(None, 5f32))),
        )));

        let wander = Rc::new(RefCell::new(Wander::new(
            None,
            Rc::new(RefCell::new(LiteralF32::new(None, 10f32))),
        )));

        Self {
            name: get_behavior_name(name),
            cycle: 0,
            robot_interface: robot_interface.clone(),

            move_robot: MoveRobot::new(
                None,
                robot_interface.clone(),
                Rc::new(RefCell::new(SumVectors::new(
                    None,
                    vec![move_to, avoid_obs, wander],
                    vec![
                        Rc::new(RefCell::new(LiteralF32::new(None, 1f32))),
                        Rc::new(RefCell::new(LiteralF32::new(None, 1f32))),
                        Rc::new(RefCell::new(LiteralF32::new(None, 0.3f32))),
                    ],
                ))),
                Rc::new(RefCell::new(LiteralF32::new(None, 1f32))),
                Rc::new(RefCell::new(LiteralF32::new(None, 1f32))),
            ),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_output(&mut self, cycle: u64) -> &dyn Any {
        self.move_robot.get_output(cycle)
    }
}

impl Behavior for TestGoto {
    fn get_name(&self) -> &str {
        TestGoto::get_name(self)
    }

    fn get_output(&mut self, cycle: u64) -> &dyn Any {
        TestGoto::get_output(self, cycle)
    }
}
