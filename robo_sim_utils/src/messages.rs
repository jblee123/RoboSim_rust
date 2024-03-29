use std::any::Any;

use super::color::Color;
use super::robot_position::RobotPosition;
use super::vec3d::Vec3d;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MessageType {
    Alive = 1,
    Start = 2,
    RequestPosition = 3,
    Position = 4,
    Kill = 5,
    RobotDying = 6,
    GetObstacles = 7,
    ObsReadings = 8,
    Pause = 9,
    Move = 10,
    Spin = 11,
}

pub trait Message {
    fn get_type(&self) -> MessageType;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

////////////////////////////////////////////////////////////////////////////////

pub fn downcast<'a, T: 'static>(msg: &'a Box<dyn Message>, fn_name: &str) -> &'a T {
    match msg.as_any().downcast_ref::<T>() {
        Some(m) => m,
        None => panic!(
            "Downcast from Message to {} failed in {}().",
            std::any::type_name::<T>(),
            fn_name
        ),
    }
}

pub fn downcast_mut<'a, T: 'static>(msg: &'a mut Box<dyn Message>, fn_name: &str) -> &'a mut T {
    match msg.as_any_mut().downcast_mut::<T>() {
        Some(m) => m,
        None => panic!(
            "Downcast from Message to {} failed in {}().",
            std::any::type_name::<T>(),
            fn_name
        ),
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct AliveMsg {
    pub msg_type: MessageType,
    pub id: u32,
    pub pos: RobotPosition,
    pub color: Color,
    pub max_vel: f32,
    pub max_angular_vel: f32,
    pub radius: f32,
}

impl AliveMsg {
    pub fn new(
        id: u32,
        pos: RobotPosition,
        color: Color,
        max_vel: f32,
        max_angular_vel: f32,
        radius: f32,
    ) -> Self {
        Self {
            msg_type: MessageType::Alive,
            id: id,
            pos: pos,
            color: color,
            max_vel: max_vel,
            max_angular_vel: max_angular_vel,
            radius: radius,
        }
    }
}

impl Message for AliveMsg {
    fn get_type(&self) -> MessageType {
        self.msg_type
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct StartMsg {
    pub msg_type: MessageType,
}

impl StartMsg {
    pub fn new() -> Self {
        Self {
            msg_type: MessageType::Start,
        }
    }
}

impl Message for StartMsg {
    fn get_type(&self) -> MessageType {
        self.msg_type
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RequestPositionMsg {
    pub msg_type: MessageType,
    pub id: u32,
}

impl RequestPositionMsg {
    pub fn new(id: u32) -> Self {
        Self {
            msg_type: MessageType::RequestPosition,
            id: id,
        }
    }
}

impl Message for RequestPositionMsg {
    fn get_type(&self) -> MessageType {
        self.msg_type
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PositionMsg {
    pub msg_type: MessageType,
    pub id: u32,
    pub pos: RobotPosition,
}

impl PositionMsg {
    pub fn new(id: u32, pos: RobotPosition) -> Self {
        Self {
            msg_type: MessageType::Position,
            id: id,
            pos: pos,
        }
    }
}

impl Message for PositionMsg {
    fn get_type(&self) -> MessageType {
        self.msg_type
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct KillMsg {
    pub msg_type: MessageType,
}

impl KillMsg {
    pub fn new() -> Self {
        Self {
            msg_type: MessageType::Kill,
        }
    }
}

impl Message for KillMsg {
    fn get_type(&self) -> MessageType {
        self.msg_type
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RobotDyingMsg {
    pub msg_type: MessageType,
    pub id: u32,
}

impl RobotDyingMsg {
    pub fn new(id: u32) -> Self {
        Self {
            msg_type: MessageType::RobotDying,
            id: id,
        }
    }
}

impl Message for RobotDyingMsg {
    fn get_type(&self) -> MessageType {
        self.msg_type
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct GetObstaclesMsg {
    pub msg_type: MessageType,
    pub id: u32,
}

impl GetObstaclesMsg {
    pub fn new(id: u32) -> Self {
        Self {
            msg_type: MessageType::GetObstacles,
            id: id,
        }
    }
}

impl Message for GetObstaclesMsg {
    fn get_type(&self) -> MessageType {
        self.msg_type
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone)]
pub struct ObsReadingsMsg {
    pub msg_type: MessageType,
    pub readings: Vec<Vec3d<f32>>,
}

impl ObsReadingsMsg {
    pub fn new(readings: Vec<Vec3d<f32>>) -> Self {
        Self {
            msg_type: MessageType::ObsReadings,
            readings: readings,
        }
    }
}

impl Message for ObsReadingsMsg {
    fn get_type(&self) -> MessageType {
        self.msg_type
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PauseMsg {
    pub msg_type: MessageType,
}

impl PauseMsg {
    pub fn new() -> Self {
        Self {
            msg_type: MessageType::Pause,
        }
    }
}

impl Message for PauseMsg {
    fn get_type(&self) -> MessageType {
        self.msg_type
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MoveMsg {
    pub msg_type: MessageType,
    pub id: u32,
    pub x: f32,
    pub y: f32,
}

impl MoveMsg {
    pub fn new(id: u32, x: f32, y: f32) -> Self {
        Self {
            msg_type: MessageType::Move,
            id: id,
            x: x,
            y: y,
        }
    }
}

impl Message for MoveMsg {
    fn get_type(&self) -> MessageType {
        self.msg_type
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SpinMsg {
    pub msg_type: MessageType,
    pub id: u32,
    pub theta: f32,
}

impl SpinMsg {
    pub fn new(id: u32, theta: f32) -> Self {
        Self {
            msg_type: MessageType::Spin,
            id: id,
            theta: theta,
        }
    }
}

impl Message for SpinMsg {
    fn get_type(&self) -> MessageType {
        self.msg_type
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
