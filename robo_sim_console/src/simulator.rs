use std::collections::HashMap;

use std::f64::consts::PI;

use robo_sim_utils;
use robo_sim_utils::color::*;
use robo_sim_utils::ray::*;
use robo_sim_utils::robot_position::*;
use robo_sim_utils::vec3d;

use super::console_comms::ConsoleComms;
use super::environment::Environment;
use super::environment::Robot as EnvRobot;

#[derive(Clone, Copy)]
struct RobotInfo {
    pos: RobotPosition,
    max_vel: f32,
    max_angular_vel: f32,
    radius: f32,
}

impl RobotInfo {
    pub fn new(
        pos: RobotPosition,
        max_vel: f32,
        max_angular_vel: f32,
        radius: f32,
    ) -> Self {
        Self {
            pos: pos,
            max_vel: max_vel,
            max_angular_vel: max_angular_vel,
            radius: radius,
        }
    }
}

pub struct Simulator {
    robots: HashMap<u32, RobotInfo>,
    time_step: f32,
}

impl Simulator {
    const NUM_OF_SIM_RAYS: u32 = 16;

    pub fn new(time_step: f32) -> Self {
        Self {
            robots: HashMap::new(),
            time_step: time_step,
        }
    }

    pub fn register_robot(
        &mut self,
        id: u32,
        pos: RobotPosition,
        color: Color,
        max_vel: f32,
        max_angular_vel: f32,
        radius: f32,
        env: &mut Environment,
        comms: &ConsoleComms,
    ) {
        self.robots.insert(
            id,
            RobotInfo::new(pos, max_vel, max_angular_vel, radius),
        );
        env.add_robot(EnvRobot::new(id, pos, color));
        comms.send_start_msg(id);
    }

    pub fn update_robot_pos(&mut self, id: u32, pos: RobotPosition, env: &mut Environment) {
        match self.robots.get_mut(&id) {
            Some(robot) => {
                robot.pos = pos;
                env.update_robot_pos(id, pos);
            }
            None => println!("Error: tried to update an unregistered robot: {}", id),
        };
    }

    pub fn get_robot_pos(&self, id: u32) -> Option<RobotPosition> {
        match self.robots.get(&id) {
            Some(robot) => Some(robot.pos),
            None => {
                println!("Error: tried to update an unregistered robot: {}", id);
                None
            }
        }
    }

    fn global_to_egocentric(
        robot_pos: &RobotPosition,
        to_convert: vec3d::Vec3d<f32>,
    ) -> vec3d::Vec3d<f32> {
        let pos = to_convert - robot_pos.location;
        let pos = pos.rotated_z(-robot_pos.heading_rad);
        pos
    }

    fn egocentric_to_global(
        robot_pos: &RobotPosition,
        to_convert: vec3d::Vec3d<f32>,
    ) -> vec3d::Vec3d<f32> {
        let pos = to_convert.rotated_z(robot_pos.heading_rad);
        let pos = pos + robot_pos.location;
        pos
    }

    fn get_closest_reading(
        &self,
        robot_pos: &RobotPosition,
        ray_num: u32,
        env: &Environment,
    ) -> Option<vec3d::Vec3d<f32>> {
        // create the ray_num'th ray
        const FULL_CIRCLE_RAD: f32 = (2.0 * PI) as f32;
        let v = vec3d::Vec3d::<f32>::new(1.0, 0.0, 0.0);
        let rad_per_ray = FULL_CIRCLE_RAD / (Simulator::NUM_OF_SIM_RAYS as f32);
        let ray_angle = (ray_num as f32) * rad_per_ray;
        let v = v.rotated_z(robot_pos.heading_rad + ray_angle);
        let v = v + robot_pos.location;
        let ray = Ray::new(robot_pos.location, v);

        // look for the closest reading
        let obs_readings = env
            .obstacles
            .iter()
            .filter_map(|obs| obs.intersect_with_ray(&ray));
        let wall_readings = env
            .walls
            .iter()
            .filter_map(|wall| wall.intersect_with_ray(&ray));

        obs_readings
            .chain(wall_readings)
            .map(|reading| Simulator::global_to_egocentric(&robot_pos, reading))
            .reduce(|accum, reading| {
                if accum.len_sq() <= reading.len_sq() {
                    accum
                } else {
                    reading
                }
            })
    }

    pub fn get_obs_readings(&self, id: u32, env: &mut Environment) -> Vec<vec3d::Vec3d<f32>> {
        let robot = self.robots.get(&id);
        if robot.is_none() {
            return vec![];
        }

        let robot = robot.unwrap();
        let readings = (0..Simulator::NUM_OF_SIM_RAYS)
            .filter_map(|ray_num| self.get_closest_reading(&robot.pos, ray_num, env))
            .collect::<Vec<vec3d::Vec3d<f32>>>();

        let global_reading_positions = readings
            .iter()
            .map(|reading| Simulator::egocentric_to_global(&robot.pos, *reading))
            .collect::<Vec<vec3d::Vec3d<f32>>>();
        env.set_obstacle_readings(id, global_reading_positions);

        readings
    }

    pub fn robot_dying(&mut self, id: u32, env: &mut Environment) -> bool {
        env.remove_robot(id);
        self.robots.remove(&id);
        self.robots.is_empty()
    }

    fn constrain_by_robot(
        &self,
        requested: vec3d::Vec3d<f32>,
        max_vel: f32,
        max_angular_vel: f32,
    ) -> vec3d::Vec3d<f32> {
        let max_turn = max_angular_vel * self.time_step;
        let angle = requested.angle_rad();
        let angle_in_timestep = angle * self.time_step;
        let angle = if angle >= 0f32 {
            angle_in_timestep.min(max_turn)
        } else {
            angle_in_timestep.max(-max_turn)
        };
        let max_dist = max_vel * self.time_step;
        let requested_dist_in_timestep = requested.len() * self.time_step;
        let dist = requested_dist_in_timestep.min(max_dist);

        vec3d::Vec3d::<f32>::new(dist, 0.0, 0.0).rotated_z(angle)
    }

    fn constrain_by_environment(
        &self,
        from_vec: vec3d::Vec3d<f32>,
        to_vec: vec3d::Vec3d<f32>,
        radius: f32,
        env: &Environment,
    ) -> vec3d::Vec3d<f32> {
        let ray = Ray::new(from_vec, to_vec);
        let mut delta = to_vec - from_vec;
        let ray_len = delta.len();

        // look in the obstacles for the closest reading
        let obs_collisions = env
            .obstacles
            .iter()
            .filter_map(|obs| obs.intersect_with_ray(&ray));
        let wall_collisions = env
            .walls
            .iter()
            .filter_map(|wall| wall.intersect_with_ray(&ray));

        let collision_dist = obs_collisions
            .chain(wall_collisions)
            .map(|collision| (collision - from_vec).len())
            .filter(|collision_dist| *collision_dist < (ray_len + radius))
            .reduce(f32::min);

        if collision_dist.is_some() {
            delta = delta.to_unit() * (collision_dist.unwrap() - radius);
        }

        delta
    }

    pub fn move_robot(&mut self, id: u32, x: f32, y: f32, env: &mut Environment) {
        if !self.robots.contains_key(&id) {
            println!("Error: tried to move an unregistered robot: {}", id);
            return;
        }

        let requested = vec3d::Vec3d::<f32>::new(x, y, 0.0);

        let mut robot = *self.robots.get(&id).unwrap();

        // make sure the robot doesn't violate max velocity and
        // angular velocity constraints
        let v = self.constrain_by_robot(requested, robot.max_vel, robot.max_angular_vel);

        let v = v.rotated_z(robot.pos.heading_rad); // switch to real-world direction
        robot.pos.heading_rad = v.angle_rad(); // we've already got the new heading

        // make sure the robot doesn't violate any environmental constraints
        let v = self.constrain_by_environment(
            robot.pos.location,
            robot.pos.location + v,
            robot.radius,
            env,
        );

        // # update the robot's position and re-draw it
        robot.pos.location = robot.pos.location + v;
        {
            self.robots.get_mut(&id).unwrap().pos = robot.pos;
        }
        env.update_robot_pos(id, robot.pos);
    }

    pub fn spin_robot(&mut self, id: u32, _theta: f32) {
        println!("Error: spin_robot not implemented: {}", id);
    }
}
