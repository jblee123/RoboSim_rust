pub mod ray;
pub mod robot_position;
pub mod vec3d;

use std::f64::consts::PI;

use num_traits::Float;

pub fn wrap_in_interval<T: Float>(value: T, interval_start: T, interval_end: T) -> T {
    if interval_start == interval_end {
        return interval_start;
    }

    let mut interval_start = interval_start;
    let mut interval_end = interval_end;

    if interval_end < interval_start {
        std::mem::swap(&mut interval_start, &mut interval_end);
    }

    if (value < interval_start) || (value >= interval_end) {
        let length = interval_end - interval_start;
        return value - ((value - interval_start) / length).floor() * length;
    }

    value
}

pub fn normalize_angle_pi<T: Float>(ang_rad: T) -> T {
    let my_pi = T::from(PI).unwrap();
    wrap_in_interval(ang_rad, -my_pi, my_pi)
}

pub fn normalize_angle_2pi<T: Float>(ang_rad: T) -> T {
    let my_zero = T::from(0.0).unwrap();
    let my_2pi = T::from(2.0 * PI).unwrap();
    wrap_in_interval(ang_rad, my_zero, my_2pi)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_in_interval() {
        assert_eq!(wrap_in_interval(1.0, 0.0, 2.0), 1.0);
        assert_eq!(wrap_in_interval(-1.0, 0.0, 2.0), 1.0);
        assert_eq!(wrap_in_interval(-1.5, 0.0, 2.0), 0.5);
        assert_eq!(wrap_in_interval(2.5, 0.0, 2.0), 0.5);
        assert_eq!(wrap_in_interval(6.5, 0.0, 2.0), 0.5);
    }

    #[test]
    fn test_normalize_angle_pi() {
        const HALF_PI: f64 = PI / 2.0;

        assert_eq!(normalize_angle_pi(0.0), 0.0);
        assert_eq!(normalize_angle_pi(HALF_PI), HALF_PI);
        assert_eq!(normalize_angle_pi(1.5 * PI), -HALF_PI);
        assert_eq!(normalize_angle_pi(2.0 * PI), 0.0);
        assert_eq!(normalize_angle_pi(3.0 * PI), -PI);
        assert_eq!(normalize_angle_pi(4.0 * PI), 0.0);
        assert_eq!(normalize_angle_pi(-HALF_PI), -HALF_PI);
        assert_eq!(normalize_angle_pi(-2.0 * PI), 0.0);
    }

    #[test]
    fn test_normalize_angle_2pi() {
        const HALF_PI: f64 = PI / 2.0;

        assert_eq!(normalize_angle_2pi(0.0), 0.0);
        assert_eq!(normalize_angle_2pi(HALF_PI), HALF_PI);
        assert_eq!(normalize_angle_2pi(1.5 * PI), 1.5 * PI);
        assert_eq!(normalize_angle_2pi(2.0 * PI), 0.0);
        assert_eq!(normalize_angle_2pi(3.0 * PI), PI);
        assert_eq!(normalize_angle_2pi(4.0 * PI), 0.0);
        assert_eq!(normalize_angle_2pi(-HALF_PI), 1.5 * PI);
        assert_eq!(normalize_angle_2pi(-2.0 * PI), 0.0);
    }
}
