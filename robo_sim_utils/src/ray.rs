use num_traits::Float;

use super::vec3d::Vec3d;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Ray<T: Float> {
    pub from_loc: Vec3d<T>,
    pub to_loc: Vec3d<T>,
}

impl<T: Float> Ray<T> {
    pub fn new(from_loc: Vec3d<T>, to_loc: Vec3d<T>) -> Self {
        Self {
            from_loc: from_loc,
            to_loc: to_loc,
        }
    }

    pub fn intersect_with_circle_2d(&self, xc: T, yc: T, radius: T) -> Option<Vec3d<T>> {
        let zero = T::zero();
        let two = T::from(2.0).unwrap();
        let four = T::from(4.0).unwrap();

        let (x0, y0) = (self.from_loc.x, self.from_loc.y);
        let (dx, dy) = (self.to_loc.x - x0, self.to_loc.y - y0);
        let a = dx * dx + dy * dy;
        let b = two * (x0 * dx - xc * dx + y0 * dy - yc * dy);
        let c = (x0 * x0) + (xc * xc) - two * x0 * xc + (y0 * y0) + (yc * yc)
            - two * y0 * yc
            - (radius * radius);
        let quot = (b * b) - four * a * c;

        if quot < zero {
            return None;
        }

        let sqrt_quot = quot.sqrt();
        let t1 = (-b + sqrt_quot) / (two * a);
        let t2 = (-b - sqrt_quot) / (two * a);

        let v1 = {
            if t1 > zero {
                Some(Vec3d::<T>::new(x0 + t1 * dx, y0 + t1 * dy, zero))
            } else {
                None
            }
        };

        let v2 = {
            if t2 > zero {
                Some(Vec3d::<T>::new(x0 + t2 * dx, y0 + t2 * dy, zero))
            } else {
                None
            }
        };

        if let (Some(v1), Some(v2)) = (v1, v2) {
            let dist_to_v1 = (v1 - self.from_loc).len();
            let dist_to_v2 = (v2 - self.from_loc).len();
            if dist_to_v1 < dist_to_v2 {
                return Some(v1);
            } else {
                return Some(v2);
            }
        } else if let Some(v1) = v1 {
            return Some(v1);
        } else {
            return v2;
        }
    }

    pub fn intersect_with_segment_2d(&self, xs0: T, ys0: T, xs1: T, ys1: T) -> Option<Vec3d<T>> {
        let zero = T::zero();
        let one = T::from(1.0).unwrap();

        let (x0, y0) = (self.from_loc.x, self.from_loc.y);
        let (dxr, dyr) = (self.to_loc.x - x0, self.to_loc.y - y0);
        let (dxs, dys) = (xs1 - xs0, ys1 - ys0);

        let intersection_exists =
            ((dxs * dyr - dys * dxr) != zero) && ((dxr != zero) || (dyr != zero));

        if !intersection_exists {
            return None;
        }

        let ts = dxr * (ys0 - y0) + dyr * (x0 - xs0);
        let ts = ts / (dxs * dyr - dys * dxr);
        let tr = {
            if dxr != zero {
                (xs0 + ts * dxs - x0) / dxr
            } else {
                (ys0 + ts * dys - y0) / dyr
            }
        };

        let intersection = {
            if (ts >= zero) && (ts <= one) && (tr >= zero) {
                Some(Vec3d::<T>::new(xs0 + ts * dxs, ys0 + ts * dys, zero))
            } else {
                None
            }
        };

        intersection
    }
}
