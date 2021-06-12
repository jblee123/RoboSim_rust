use piston_window as pw;

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
}

pub fn draw_obstacle(
    obs: &environ::Obstacle,
    scale: &environ::EnvironmentScale,
    context: pw::Context,
    graphics: &mut pw::G2d<'_>,
) {
    let (x, y) = scale.coords_meters_to_pixels(obs.x, obs.y);
    let r = scale.dist_meters_to_pixels(obs.radius) as f64;
    pw::ellipse(
        pw::color::BLACK,
        [(x as f64 - r), (y as f64 - r), r, r],
        context.transform,
        graphics,
    );
}

pub fn draw_wall(
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

pub fn draw_object(
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
        [(x as f64 - r), (y as f64 - r), r, r],
        context.transform,
        graphics,
    );
}
