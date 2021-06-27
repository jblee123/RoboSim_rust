use std::error::Error;

use super::color::*;
use super::messages::*;
use super::robot_position::*;
use super::vec3d::*;

pub const CONSOLE_PORT: u16 = 50000;

pub fn parse_message(msg_buf: &[u8]) -> Result<Box<dyn Message>, Box<dyn Error>> {
    if msg_buf.len() < 1 {
        return Err("msg buf doesnt' have type byte")?;
    }

    if msg_buf[0] == MessageType::Alive as u8 {
        let msg = parse_alive_message(msg_buf)?;
        return Ok(Box::new(msg));
    }

    if msg_buf[0] == MessageType::Start as u8 {
        let msg = parse_start_message(msg_buf)?;
        return Ok(Box::new(msg));
    }

    if msg_buf[0] == MessageType::RequestPosition as u8 {
        let msg = parse_request_position_message(msg_buf)?;
        return Ok(Box::new(msg));
    }

    if msg_buf[0] == MessageType::Position as u8 {
        let msg = parse_position_message(msg_buf)?;
        return Ok(Box::new(msg));
    }

    if msg_buf[0] == MessageType::Kill as u8 {
        let msg = parse_kill_message(msg_buf)?;
        return Ok(Box::new(msg));
    }

    if msg_buf[0] == MessageType::RobotDying as u8 {
        let msg = parse_robot_dying_message(msg_buf)?;
        return Ok(Box::new(msg));
    }

    if msg_buf[0] == MessageType::GetObstacles as u8 {
        let msg = parse_get_obstacles_message(msg_buf)?;
        return Ok(Box::new(msg));
    }

    if msg_buf[0] == MessageType::ObsReadings as u8 {
        let msg = parse_obs_readings_message(msg_buf)?;
        return Ok(Box::new(msg));
    }

    if msg_buf[0] == MessageType::Pause as u8 {
        let msg = parse_pause_message(msg_buf)?;
        return Ok(Box::new(msg));
    }

    if msg_buf[0] == MessageType::Move as u8 {
        let msg = parse_move_message(msg_buf)?;
        return Ok(Box::new(msg));
    }

    if msg_buf[0] == MessageType::Spin as u8 {
        let msg = parse_spin_message(msg_buf)?;
        return Ok(Box::new(msg));
    }

    Err(format!("bad msg type byte: {}", msg_buf[0]))?
}

const ALIVE_MSG_LEN: usize = 1 + 4 + (4 * 4) + 4 + (4 * 3);
const START_MSG_LEN: usize = 1;
const REQUEST_POSITION_MSG_LEN: usize = 1 + 4;
const POSITION_MSG_LEN: usize = 1 + (4 * 4);
const KILL_MSG_LEN: usize = 1;
const ROBOT_DYING_MSG_LEN: usize = 1 + 4;
const GET_OBSTACLES_MSG_LEN: usize = 1 + 4;
const OBS_READING_SIZE: usize = 8;
const PAUSE_MSG_LEN: usize = 1;
const MOVE_MSG_LEN: usize = 1 + 4 + 4 + 4;
const SPIN_MSG_LEN: usize = 1 + 4 + 4;

fn to_arr_4(buf: &[u8], offset: usize) -> [u8; 4] {
    [
        buf[offset + 0],
        buf[offset + 1],
        buf[offset + 2],
        buf[offset + 3],
    ]
}

fn arr4_into_vec(dst: &mut Vec<u8>, offset: usize, src: [u8; 4]) {
    dst[offset + 0] = src[0];
    dst[offset + 1] = src[1];
    dst[offset + 2] = src[2];
    dst[offset + 3] = src[3];
}

fn check_msg_buf_len(
    msg_buf: &[u8],
    expected: usize,
    msg_label: &str,
) -> Result<(), Box<dyn Error>> {
    if msg_buf.len() != expected {
        return Err(format!(
            "{} msg wrong size. got {}, expected {} bytes",
            msg_label,
            msg_buf.len(),
            expected
        ))?;
    }

    Ok(())
}

fn check_msg_buf_expected_type(
    msg_buf: &[u8],
    expected: MessageType,
    msg_label: &str,
) -> Result<(), Box<dyn Error>> {
    if msg_buf[0] != expected as u8 {
        return Err(format!(
            "{} msg buf has wrong start byte: {}",
            msg_label, msg_buf[0]
        ))?;
    }

    Ok(())
}

fn parse_alive_message(msg_buf: &[u8]) -> Result<AliveMsg, Box<dyn Error>> {
    check_msg_buf_len(msg_buf, ALIVE_MSG_LEN, "ALIVE")?;
    check_msg_buf_expected_type(msg_buf, MessageType::Alive, "ALIVE")?;

    let id = u32::from_be_bytes(to_arr_4(msg_buf, 1));

    let loc = Vec3d::new(
        f32::from_be_bytes(to_arr_4(msg_buf, 5)),
        f32::from_be_bytes(to_arr_4(msg_buf, 9)),
        f32::from_be_bytes(to_arr_4(msg_buf, 13)),
    );
    let heading = f32::from_be_bytes(to_arr_4(msg_buf, 17));

    let color = Color::new(msg_buf[21], msg_buf[22], msg_buf[23], msg_buf[24]);

    let max_vel = f32::from_be_bytes(to_arr_4(msg_buf, 25));
    let max_angular_vel = f32::from_be_bytes(to_arr_4(msg_buf, 29));
    let radius = f32::from_be_bytes(to_arr_4(msg_buf, 33));

    Ok(AliveMsg::new(
        id,
        RobotPosition::new(loc, heading),
        color,
        max_vel,
        max_angular_vel,
        radius,
    ))
}

pub fn pack_alive_message(msg: AliveMsg) -> Vec<u8> {
    let mut buf = vec![0; ALIVE_MSG_LEN];

    buf[0] = msg.msg_type as u8;

    arr4_into_vec(&mut buf, 1, u32::to_be_bytes(msg.id));

    arr4_into_vec(&mut buf, 5, f32::to_be_bytes(msg.pos.location.x));
    arr4_into_vec(&mut buf, 9, f32::to_be_bytes(msg.pos.location.y));
    arr4_into_vec(&mut buf, 13, f32::to_be_bytes(msg.pos.location.z));
    arr4_into_vec(&mut buf, 17, f32::to_be_bytes(msg.pos.heading));

    buf[21] = msg.color.r;
    buf[22] = msg.color.g;
    buf[23] = msg.color.b;
    buf[24] = msg.color.a;

    arr4_into_vec(&mut buf, 25, f32::to_be_bytes(msg.max_vel));
    arr4_into_vec(&mut buf, 29, f32::to_be_bytes(msg.max_angular_vel));
    arr4_into_vec(&mut buf, 33, f32::to_be_bytes(msg.radius));

    buf
}

fn parse_start_message(msg_buf: &[u8]) -> Result<StartMsg, Box<dyn Error>> {
    check_msg_buf_len(msg_buf, START_MSG_LEN, "START")?;
    check_msg_buf_expected_type(msg_buf, MessageType::Start, "START")?;

    Ok(StartMsg::new())
}

pub fn pack_start_message(msg: StartMsg) -> Vec<u8> {
    let mut buf = vec![0; START_MSG_LEN];

    buf[0] = msg.msg_type as u8;

    buf
}

fn parse_request_position_message(msg_buf: &[u8]) -> Result<RequestPositionMsg, Box<dyn Error>> {
    check_msg_buf_len(msg_buf, REQUEST_POSITION_MSG_LEN, "REQUEST_POSITION")?;
    check_msg_buf_expected_type(msg_buf, MessageType::RequestPosition, "REQUEST_POSITION")?;

    let id = u32::from_be_bytes(to_arr_4(msg_buf, 1));

    Ok(RequestPositionMsg::new(id))
}

pub fn pack_request_position_message(msg: RequestPositionMsg) -> Vec<u8> {
    let mut buf = vec![0; REQUEST_POSITION_MSG_LEN];

    buf[0] = msg.msg_type as u8;

    arr4_into_vec(&mut buf, 1, u32::to_be_bytes(msg.id));

    buf
}

fn parse_position_message(msg_buf: &[u8]) -> Result<PositionMsg, Box<dyn Error>> {
    check_msg_buf_len(msg_buf, POSITION_MSG_LEN, "POSITION")?;
    check_msg_buf_expected_type(msg_buf, MessageType::Position, "POSITION")?;

    let loc = Vec3d::new(
        f32::from_be_bytes(to_arr_4(msg_buf, 1)),
        f32::from_be_bytes(to_arr_4(msg_buf, 5)),
        f32::from_be_bytes(to_arr_4(msg_buf, 9)),
    );
    let heading = f32::from_be_bytes(to_arr_4(msg_buf, 13));

    Ok(PositionMsg::new(RobotPosition::new(loc, heading)))
}

pub fn pack_position_message(msg: PositionMsg) -> Vec<u8> {
    let mut buf = vec![0; POSITION_MSG_LEN];

    buf[0] = msg.msg_type as u8;

    arr4_into_vec(&mut buf, 1, f32::to_be_bytes(msg.pos.location.x));
    arr4_into_vec(&mut buf, 5, f32::to_be_bytes(msg.pos.location.y));
    arr4_into_vec(&mut buf, 9, f32::to_be_bytes(msg.pos.location.z));
    arr4_into_vec(&mut buf, 13, f32::to_be_bytes(msg.pos.heading));

    buf
}

fn parse_kill_message(msg_buf: &[u8]) -> Result<KillMsg, Box<dyn Error>> {
    check_msg_buf_len(msg_buf, KILL_MSG_LEN, "KILL")?;
    check_msg_buf_expected_type(msg_buf, MessageType::Kill, "KILL")?;

    Ok(KillMsg::new())
}

pub fn pack_kill_message(msg: KillMsg) -> Vec<u8> {
    let mut buf = vec![0; KILL_MSG_LEN];

    buf[0] = msg.msg_type as u8;

    buf
}

fn parse_robot_dying_message(msg_buf: &[u8]) -> Result<RobotDyingMsg, Box<dyn Error>> {
    check_msg_buf_len(msg_buf, ROBOT_DYING_MSG_LEN, "ROBOT_DYING")?;
    check_msg_buf_expected_type(msg_buf, MessageType::RobotDying, "ROBOT_DYING")?;

    let id = u32::from_be_bytes(to_arr_4(msg_buf, 1));

    Ok(RobotDyingMsg::new(id))
}

pub fn pack_robot_dying_message(msg: RobotDyingMsg) -> Vec<u8> {
    let mut buf = vec![0; ROBOT_DYING_MSG_LEN];

    buf[0] = msg.msg_type as u8;

    arr4_into_vec(&mut buf, 1, u32::to_be_bytes(msg.id));

    buf
}

fn parse_get_obstacles_message(msg_buf: &[u8]) -> Result<GetObstaclesMsg, Box<dyn Error>> {
    check_msg_buf_len(msg_buf, GET_OBSTACLES_MSG_LEN, "GET_OBSTACLES")?;
    check_msg_buf_expected_type(msg_buf, MessageType::GetObstacles, "GET_OBSTACLES")?;

    let id = u32::from_be_bytes(to_arr_4(msg_buf, 1));

    Ok(GetObstaclesMsg::new(id))
}

pub fn pack_get_obstacles_message(msg: GetObstaclesMsg) -> Vec<u8> {
    let mut buf = vec![0; GET_OBSTACLES_MSG_LEN];

    buf[0] = msg.msg_type as u8;

    arr4_into_vec(&mut buf, 1, u32::to_be_bytes(msg.id));

    buf
}

fn parse_obs_readings_message(msg_buf: &[u8]) -> Result<ObsReadingsMsg, Box<dyn Error>> {
    if msg_buf.len() < 1 || ((msg_buf.len() - 1) % OBS_READING_SIZE) != 0 {
        return Err(format!(
            "{} msg wrong size. got {}, expected (1 + 8n) bytes",
            "OBS_READINGS",
            msg_buf.len()
        ))?;
    }

    check_msg_buf_expected_type(msg_buf, MessageType::ObsReadings, "OBS_READINGS")?;

    let num_readings = (msg_buf.len() - 1) / OBS_READING_SIZE;
    let mut readings = Vec::with_capacity(num_readings);

    let mut offset = 1;
    while offset < msg_buf.len() {
        let x = f32::from_be_bytes(to_arr_4(msg_buf, offset));
        offset += 4;
        let y = f32::from_be_bytes(to_arr_4(msg_buf, offset));
        offset += 4;

        readings.push((x, y));
    }

    Ok(ObsReadingsMsg::new(readings))
}

pub fn pack_obs_readings_message(msg: ObsReadingsMsg) -> Vec<u8> {
    let mut buf = vec![0; 1 + msg.readings.len() * OBS_READING_SIZE];

    buf[0] = msg.msg_type as u8;

    let mut offset = 1;
    for (x, y) in msg.readings {
        arr4_into_vec(&mut buf, offset, f32::to_be_bytes(x));
        offset += 4;
        arr4_into_vec(&mut buf, offset, f32::to_be_bytes(y));
        offset += 4;
    }

    buf
}

fn parse_pause_message(msg_buf: &[u8]) -> Result<PauseMsg, Box<dyn Error>> {
    check_msg_buf_len(msg_buf, PAUSE_MSG_LEN, "PAUSE")?;
    check_msg_buf_expected_type(msg_buf, MessageType::Pause, "PAUSE")?;

    Ok(PauseMsg::new())
}

pub fn pack_pause_message(msg: PauseMsg) -> Vec<u8> {
    let mut buf = vec![0; PAUSE_MSG_LEN];

    buf[0] = msg.msg_type as u8;

    buf
}

fn parse_move_message(msg_buf: &[u8]) -> Result<MoveMsg, Box<dyn Error>> {
    check_msg_buf_len(msg_buf, MOVE_MSG_LEN, "MOVE")?;
    check_msg_buf_expected_type(msg_buf, MessageType::Move, "MOVE")?;

    let id = u32::from_be_bytes(to_arr_4(msg_buf, 1));
    let x = f32::from_be_bytes(to_arr_4(msg_buf, 5));
    let y = f32::from_be_bytes(to_arr_4(msg_buf, 9));

    Ok(MoveMsg::new(id, x, y))
}

pub fn pack_move_message(msg: MoveMsg) -> Vec<u8> {
    let mut buf = vec![0; MOVE_MSG_LEN];

    buf[0] = msg.msg_type as u8;

    arr4_into_vec(&mut buf, 1, u32::to_be_bytes(msg.id));
    arr4_into_vec(&mut buf, 5, f32::to_be_bytes(msg.x));
    arr4_into_vec(&mut buf, 9, f32::to_be_bytes(msg.y));

    buf
}

fn parse_spin_message(msg_buf: &[u8]) -> Result<SpinMsg, Box<dyn Error>> {
    check_msg_buf_len(msg_buf, SPIN_MSG_LEN, "SPIN")?;
    check_msg_buf_expected_type(msg_buf, MessageType::Spin, "SPIN")?;

    let id = u32::from_be_bytes(to_arr_4(msg_buf, 1));
    let theta = f32::from_be_bytes(to_arr_4(msg_buf, 5));

    Ok(SpinMsg::new(id, theta))
}

pub fn pack_spin_message(msg: SpinMsg) -> Vec<u8> {
    let mut buf = vec![0; SPIN_MSG_LEN];

    buf[0] = msg.msg_type as u8;

    arr4_into_vec(&mut buf, 1, u32::to_be_bytes(msg.id));
    arr4_into_vec(&mut buf, 5, f32::to_be_bytes(msg.theta));

    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alive_message() {
        let msg = AliveMsg::new(
            3,
            RobotPosition::new(Vec3d::new(1.5, 2.5, 3.5), 4.5),
            Color::new(100, 150, 160, 1),
            5.0,
            10.0,
            20.0,
        );

        let buf = pack_alive_message(msg);
        let msg2 = parse_alive_message(buf.as_slice()).unwrap();

        assert_eq!(msg, msg2);
    }

    #[test]
    fn test_start_message() {
        let msg = StartMsg::new();

        let buf = pack_start_message(msg);
        let msg2 = parse_start_message(buf.as_slice()).unwrap();

        assert_eq!(msg, msg2);
    }

    #[test]
    fn test_request_position_message() {
        let msg = RequestPositionMsg::new(5);

        let buf = pack_request_position_message(msg);
        let msg2 = parse_request_position_message(buf.as_slice()).unwrap();

        assert_eq!(msg, msg2);
    }

    #[test]
    fn test_position_message() {
        let msg = PositionMsg::new(RobotPosition::new(Vec3d::new(1.5, 2.5, 3.5), 4.5));

        let buf = pack_position_message(msg);
        let msg2 = parse_position_message(buf.as_slice()).unwrap();

        assert_eq!(msg, msg2);
    }

    #[test]
    fn test_kill_message() {
        let msg = KillMsg::new();

        let buf = pack_kill_message(msg);
        let msg2 = parse_kill_message(buf.as_slice()).unwrap();

        assert_eq!(msg, msg2);
    }

    #[test]
    fn test_robot_dying_message() {
        let msg = RobotDyingMsg::new(5);

        let buf = pack_robot_dying_message(msg);
        let msg2 = parse_robot_dying_message(buf.as_slice()).unwrap();

        assert_eq!(msg, msg2);
    }

    #[test]
    fn test_get_obstacles_message() {
        let msg = GetObstaclesMsg::new(5);

        let buf = pack_get_obstacles_message(msg);
        let msg2 = parse_get_obstacles_message(buf.as_slice()).unwrap();

        assert_eq!(msg, msg2);
    }

    #[test]
    fn test_obs_readings_message() {
        let msg = ObsReadingsMsg::new(vec![(1.0, 2.0), (3.0, 4.0), (-1.0, -2.5)]);

        let buf = pack_obs_readings_message(msg.clone());
        let msg2 = parse_obs_readings_message(buf.as_slice()).unwrap();

        assert_eq!(msg, msg2);
    }

    #[test]
    fn test_pause_message() {
        let msg = PauseMsg::new();

        let buf = pack_pause_message(msg);
        let msg2 = parse_pause_message(buf.as_slice()).unwrap();

        assert_eq!(msg, msg2);
    }

    #[test]
    fn test_move_message() {
        let msg = MoveMsg::new(5, 1.5, 2.5);

        let buf = pack_move_message(msg);
        let msg2 = parse_move_message(buf.as_slice()).unwrap();

        assert_eq!(msg, msg2);
    }

    #[test]
    fn test_spin_message() {
        let msg = SpinMsg::new(5, 1.5);

        let buf = pack_spin_message(msg);
        let msg2 = parse_spin_message(buf.as_slice()).unwrap();

        assert_eq!(msg, msg2);
    }
}
