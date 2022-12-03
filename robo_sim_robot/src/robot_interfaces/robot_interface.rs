use robo_sim_utils::robot_position::RobotPosition;
use robo_sim_utils::vec3d::Vec3d;

pub trait RobotInterface {
    fn get_position(&self) -> RobotPosition;
    fn cmd_move(&self, x: f32, y: f32);
    fn get_obs_readings(&self) -> Vec<Vec3d<f32>>;
}
