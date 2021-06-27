use robo_sim_utils::color::*;
use robo_sim_utils::ray::*;
use robo_sim_utils::vec3d::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Obstacle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

impl Obstacle {
    pub fn new(x: f32, y: f32, radius: f32) -> Self {
        Self {
            x: x,
            y: y,
            radius: radius,
        }
    }

    pub fn intersect_with_ray(&self, ray: &Ray<f32>) -> Option<Vec3d<f32>> {
        ray.intersect_with_circle_2d(self.x, self.y, self.radius)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Object {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub color: Color,
}

impl Object {
    pub fn new(x: f32, y: f32, radius: f32, color: Color) -> Self {
        Self {
            x: x,
            y: y,
            radius: radius,
            color: color,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Wall {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

impl Wall {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self {
            x1: x1,
            y1: y1,
            x2: x2,
            y2: y2,
        }
    }

    pub fn intersect_with_ray(&self, ray: &Ray<f32>) -> Option<Vec3d<f32>> {
        ray.intersect_with_segment_2d(self.x1, self.y1, self.x2, self.y2)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Environment {
    pub width_m: f32,
    pub height_m: f32,
    pub obstacles: Vec<Obstacle>,
    pub walls: Vec<Wall>,
    pub objects: Vec<Object>,
}

impl Environment {
    pub fn new(width_m: f32, height_m: f32) -> Self {
        Self {
            width_m: width_m,
            height_m: height_m,
            obstacles: vec![],
            walls: vec![],
            objects: vec![],
        }
    }

    pub fn add_obstacle(&mut self, obstacle: Obstacle) {
        self.obstacles.push(obstacle);
    }

    pub fn add_wall(&mut self, wall: Wall) {
        self.walls.push(wall);
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct EnvironmentScale {
    pub pix_per_meter: f32,
    pub env_width_m: f32,
    pub env_height_m: f32,
}

impl EnvironmentScale {
    pub fn new(pix_per_meter: f32, env_width_m: f32, env_height_m: f32) -> Self {
        Self {
            pix_per_meter: pix_per_meter,
            env_width_m: env_width_m,
            env_height_m: env_height_m,
        }
    }

    pub fn from_env_size(
        width_pix: f32,
        height_pix: f32,
        env_width_m: f32,
        env_height_m: f32,
    ) -> Self {
        let pix_per_meter_width = width_pix / env_width_m;
        let pix_per_meter_height = height_pix / env_height_m;
        Self {
            pix_per_meter: pix_per_meter_width.min(pix_per_meter_height),
            env_width_m: env_width_m,
            env_height_m: env_height_m,
        }
    }

    pub fn dist_meters_to_pixels(&self, d: f32) -> f32 {
        d * self.pix_per_meter
    }

    pub fn coords_meters_to_pixels(&self, x: f32, y: f32) -> (f32, f32) {
        let x = self.dist_meters_to_pixels(x);
        let y = self.dist_meters_to_pixels(self.env_height_m) - self.dist_meters_to_pixels(y);
        (x, y)
    }

    pub fn env_size_in_pixels(&self) -> (f32, f32) {
        let x = self.pix_per_meter * self.env_width_m;
        let y = self.pix_per_meter * self.env_height_m;
        (x, y)
    }

    pub fn zoom_in(&mut self) {
        self.pix_per_meter *= 2.0;
    }

    pub fn zoom_out(&mut self) {
        self.pix_per_meter /= 2.0;
    }
}
