use piston_window as pw;

use robo_sim_utils::vec3d::Vec3d;

use super::environment as environ;

pub fn draw_env(
    env: &environ::Environment,
    scale: &environ::EnvironmentScale,
    context: pw::Context,
    graphics: &mut pw::G2d<'_>,
) {
    pw::clear([1.0; 4], graphics);

    for obs in &env.obstacles {
        draw_obstacle(obs, scale, context, graphics);
    }

    for wall in &env.walls {
        draw_wall(wall, scale, context, graphics);
    }

    for object in &env.objects {
        draw_object(object, scale, context, graphics);
    }

    for robot in env.robots.values() {
        draw_robot(robot, scale, context, graphics);
    }

    for single_robot_readings in env.obstacle_readings.values() {
        for reading in single_robot_readings.iter() {
            draw_obstacle_reading(reading, scale, context, graphics);
        }
    }
}

fn draw_obstacle(
    obs: &environ::Obstacle,
    scale: &environ::EnvironmentScale,
    context: pw::Context,
    graphics: &mut pw::G2d<'_>,
) {
    let (x, y) = scale.coords_meters_to_pixels(obs.x, obs.y);
    let r = scale.dist_meters_to_pixels(obs.radius) as f64;
    pw::ellipse(
        pw::color::BLACK,
        [(x as f64 - r), (y as f64 - r), r * 2.0, r * 2.0],
        context.transform,
        graphics,
    );
}

fn draw_wall(
    wall: &environ::Wall,
    scale: &environ::EnvironmentScale,
    context: pw::Context,
    graphics: &mut pw::G2d<'_>,
) {
    let (x1, y1) = scale.coords_meters_to_pixels(wall.x1, wall.y1);
    let (x2, y2) = scale.coords_meters_to_pixels(wall.x2, wall.y2);
    pw::line(
        pw::color::BLACK,
        0.5,
        [x1 as f64, y1 as f64, x2 as f64, y2 as f64],
        context.transform,
        graphics,
    );
}

fn draw_object(
    object: &environ::Object,
    scale: &environ::EnvironmentScale,
    context: pw::Context,
    graphics: &mut pw::G2d<'_>,
) {
    let (x, y) = scale.coords_meters_to_pixels(object.x, object.y);
    let r = scale.dist_meters_to_pixels(object.radius) as f64;
    let color = [
        object.color.r as f32,
        object.color.g as f32,
        object.color.b as f32,
        object.color.a as f32 / 255.0,
    ];
    pw::ellipse(
        color,
        [(x as f64 - r), (y as f64 - r), r * 2.0, r * 2.0],
        context.transform,
        graphics,
    );
}

fn draw_robot(
    robot: &environ::Robot,
    scale: &environ::EnvironmentScale,
    context: pw::Context,
    graphics: &mut pw::G2d<'_>,
) {
    let p1 = Vec3d::<f32>::new(-0.5, 0.5, 0.0);
    let p2 = Vec3d::<f32>::new(0.5, 0.5, 0.0);
    let p3 = Vec3d::<f32>::new(1.0, 0.0, 0.0);
    let p4 = Vec3d::<f32>::new(0.5, -0.5, 0.0);
    let p5 = Vec3d::<f32>::new(-0.5, -0.5, 0.0);

    let p1 = p1.rotated_z(robot.pos.heading_rad) + robot.pos.location;
    let p2 = p2.rotated_z(robot.pos.heading_rad) + robot.pos.location;
    let p3 = p3.rotated_z(robot.pos.heading_rad) + robot.pos.location;
    let p4 = p4.rotated_z(robot.pos.heading_rad) + robot.pos.location;
    let p5 = p5.rotated_z(robot.pos.heading_rad) + robot.pos.location;

    let (x1, y1) = scale.coords_meters_to_pixels(p1.x, p1.y);
    let (x2, y2) = scale.coords_meters_to_pixels(p2.x, p2.y);
    let (x3, y3) = scale.coords_meters_to_pixels(p3.x, p3.y);
    let (x4, y4) = scale.coords_meters_to_pixels(p4.x, p4.y);
    let (x5, y5) = scale.coords_meters_to_pixels(p5.x, p5.y);

    let coords = [
        [x1 as f64, y1 as f64],
        [x2 as f64, y2 as f64],
        [x3 as f64, y3 as f64],
        [x4 as f64, y4 as f64],
        [x5 as f64, y5 as f64],
    ];

    let color = [
        robot.color.r as f32 / 255.0,
        robot.color.g as f32 / 255.0,
        robot.color.b as f32 / 255.0,
        robot.color.a as f32 / 255.0,
    ];

    pw::polygon(color, &coords, context.transform, graphics);
}

fn draw_obstacle_reading(
    reading: &Vec3d<f32>,
    scale: &environ::EnvironmentScale,
    context: pw::Context,
    graphics: &mut pw::G2d<'_>,
) {
    let (x, y) = scale.coords_meters_to_pixels(reading.x, reading.y);
    const RAD: f64 = 2.0;
    pw::ellipse(
        pw::color::RED,
        [(x as f64 - RAD), (y as f64 - RAD), RAD * 2.0, RAD * 2.0],
        context.transform,
        graphics,
    );
}
