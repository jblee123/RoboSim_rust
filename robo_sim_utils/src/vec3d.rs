use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

use num_traits::Float;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Vec3d<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Vec3d<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x: x, y: y, z: z }
    }

    pub fn dot(&self, rhs: Self) -> T {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    pub fn len_sq(&self) -> T {
        self.dot(*self)
    }

    pub fn len(&self) -> T {
        self.dot(*self).sqrt()
    }

    pub fn angle_rad(&self) -> T {
        normalize_angle_pi(self.y.atan2(self.x))
    }

    pub fn to_unit(&self) -> Self {
        let len = self.len();
        *self / len
    }

    pub fn rotated_z(&self, angle_rad: T) -> Self {
        let c = angle_rad.cos();
        let s = angle_rad.sin();
        let x = self.x;
        let y = self.y;
        Self {
            x: x * c - y * s,
            y: x * s + y * c,
            z: self.z,
        }
    }
}

impl<T> Neg for Vec3d<T>
where
    T: Neg<Output = T> + Float,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> Add for Vec3d<T>
where
    T: Add<Output = T> + Float,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> AddAssign for Vec3d<T>
where
    T: Add<Output = T> + Float,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Sub for Vec3d<T>
where
    T: Sub<Output = T> + Float,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> SubAssign for Vec3d<T>
where
    T: Sub<Output = T> + Float,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> Mul<T> for Vec3d<T>
where
    T: Mul<Output = T> + Float,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> MulAssign<T> for Vec3d<T>
where
    T: Mul<Output = T> + Float,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> Div<T> for Vec3d<T>
where
    T: Div<Output = T> + Float,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T> DivAssign<T> for Vec3d<T>
where
    T: Div<Output = T> + Float,
{
    fn div_assign(&mut self, rhs: T) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_vec() {
        assert_eq!(
            Vec3d::new(5.0, 7.0, 9.0),
            Vec3d {
                x: 5.0,
                y: 7.0,
                z: 9.0
            }
        );
    }

    #[test]
    fn neg() {
        assert_eq!(-(Vec3d::new(2.0, 4.0, 6.0)), Vec3d::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn add() {
        assert_eq!(
            Vec3d::new(2.0, 4.0, 6.0) + Vec3d::new(7.0, 10.0, 13.0),
            Vec3d::new(9.0, 14.0, 19.0)
        );
    }

    #[test]
    fn add_assign() {
        let mut p1 = Vec3d::new(2.0, 4.0, 6.0);
        p1 += Vec3d::new(7.0, 10.0, 13.0);
        assert_eq!(p1, Vec3d::new(9.0, 14.0, 19.0));
    }

    #[test]
    fn sub() {
        assert_eq!(
            Vec3d::new(2.0, 4.0, 6.0) - Vec3d::new(7.0, 10.0, 13.0),
            Vec3d::new(-5.0, -6.0, -7.0)
        );
    }

    #[test]
    fn sub_assign() {
        let mut p1 = Vec3d::new(2.0, 4.0, 6.0);
        p1 -= Vec3d::new(7.0, 10.0, 13.0);
        assert_eq!(p1, Vec3d::new(-5.0, -6.0, -7.0));
    }

    #[test]
    fn mul() {
        assert_eq!(Vec3d::new(2.0, 4.0, 6.0) * 3.0, Vec3d::new(6.0, 12.0, 18.0));
    }

    #[test]
    fn mul_assign() {
        let mut p1 = Vec3d::new(2.0, 4.0, 6.0);
        p1 *= 3.0;
        assert_eq!(p1, Vec3d::new(6.0, 12.0, 18.0));
    }

    #[test]
    fn div() {
        assert_eq!(Vec3d::new(2.0, 4.0, 6.0) / 2.0, Vec3d::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn div_assign() {
        let mut p1 = Vec3d::new(2.0, 4.0, 6.0);
        p1 /= 2.0;
        assert_eq!(p1, Vec3d::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn dot() {
        let p1 = Vec3d::new(1.0, 2.0, 3.0);
        let p2 = Vec3d::new(4.0, 5.0, 6.0);
        assert_eq!(p1.dot(p2), 32.0);
    }

    #[test]
    fn len_sq() {
        let p1 = Vec3d::new(1.0, 2.0, 3.0);
        assert_eq!(p1.len_sq(), 14.0);
    }

    #[test]
    fn len() {
        let p1 = Vec3d::new(1.0, 2.0, 3.0);
        assert_eq!(p1.len(), 14.0.sqrt());
    }

    #[test]
    fn rotated_z() {
        const EPS: f32 = 0.00001;

        let p1 = Vec3d::new(1.0, 0.0, 6.0);
        let p2 = p1.rotated_z(90.0.to_radians());
        let diff = (Vec3d::new(0.0, 1.0, 6.0) - p2).len();
        assert!(diff < EPS);

        let p1 = Vec3d::new(0.0, 1.0, 6.0);
        let p2 = p1.rotated_z(-90.0.to_radians());
        let diff = (Vec3d::new(1.0, 0.0, 6.0) - p2).len();
        assert!(diff < EPS);
    }
}
