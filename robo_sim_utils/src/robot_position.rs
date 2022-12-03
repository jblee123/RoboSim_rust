use super::vec3d;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct RobotPosition {
    pub location: vec3d::Vec3d<f32>,
    pub heading_rad: f32,
}

impl RobotPosition {
    pub fn new(location: vec3d::Vec3d<f32>, heading_rad: f32) -> Self {
        Self {
            location: location,
            heading_rad: heading_rad,
        }
    }
}
