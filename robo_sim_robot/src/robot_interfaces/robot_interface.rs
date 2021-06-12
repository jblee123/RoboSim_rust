use robo_sim_utils::robot_position::*;
use robo_sim_utils::vec3d::*;

pub trait RobotInterface {
    fn get_position(&self) -> RobotPosition;
    fn cmd_move(&self, movement: Vec3d<f32>);
    fn get_obs_readings(&self) -> Vec<Vec3d<f32>>;
}
