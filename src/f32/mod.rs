#![allow(dead_code)]

use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::f32;
use std::fmt;
use std::ops::*;

#[derive(Clone, Copy, Debug)]
#[repr(C, align(16))]
pub struct Vec3(f32, f32, f32);

#[inline]
pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3(x, y, z)
}

impl Vec3 {
    #[inline]
    pub fn zero() -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }

    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3(x, y, z)
    }

    #[inline]
    pub fn splat(v: f32) -> Vec3 {
        Vec3(v, v, v)
    }

    #[inline]
    pub fn get_x(self) -> f32 {
        self.0
    }

    #[inline]
    pub fn get_y(self) -> f32 {
        self.1
    }

    #[inline]
    pub fn get_z(self) -> f32 {
        self.2
    }

    #[inline]
    pub fn dot(self, rhs: Vec3) -> f32 {
        (self.0 * rhs.0) + (self.1 * rhs.1) + (self.2 * rhs.2)
    }

    #[inline]
    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3(
            self.1 * rhs.2 - rhs.1 * self.2,
            self.2 * rhs.0 - rhs.2 * self.0,
            self.0 * rhs.1 - rhs.0 * self.1,
        )
    }

    #[inline]
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    #[inline]
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    #[inline]
    pub fn normalize(self) -> Vec3 {
        let inv_length = 1.0 / self.dot(self).sqrt();
        self * inv_length
    }

    #[inline]
    pub fn min(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0.min(rhs.0), self.1.min(rhs.1), self.2.min(rhs.2))
    }

    #[inline]
    pub fn max(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0.max(rhs.0), self.1.max(rhs.1), self.2.max(rhs.2))
    }

    #[inline]
    pub fn hmin(self) -> f32 {
        self.0.min(self.1.min(self.2))
    }

    #[inline]
    pub fn hmax(self) -> f32 {
        self.0.max(self.1.max(self.2))
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.0, self.1, self.2)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: f32) -> Vec3 {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        *self = Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl MulAssign<Vec3> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: f32) -> Vec3 {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        *self = Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Vec3) {
        *self = Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    #[inline]
    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, rhs: &Vec3) -> bool {
        self.0 == rhs.0 && self.1 == rhs.1 && self.2 == rhs.2
    }
    fn ne(&self, rhs: &Vec3) -> bool {
        self.0 != rhs.0 || self.1 != rhs.1 || self.2 != rhs.2
    }
}

impl PartialOrd for Vec3 {
    fn partial_cmp(&self, rhs: &Vec3) -> Option<Ordering> {
        match (
            self.0.partial_cmp(&rhs.0),
            self.1.partial_cmp(&rhs.1),
            self.2.partial_cmp(&rhs.2),
        ) {
            (Some(Ordering::Greater), Some(Ordering::Greater), Some(Ordering::Greater)) => {
                Some(Ordering::Greater)
            }
            (Some(Ordering::Less), Some(Ordering::Less), Some(Ordering::Less)) => {
                Some(Ordering::Less)
            }
            (Some(Ordering::Equal), Some(Ordering::Equal), Some(Ordering::Equal)) => {
                Some(Ordering::Equal)
            }
            _ => None,
        }
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from(t: (f32, f32, f32)) -> Self {
        Vec3::new(t.0, t.1, t.2)
    }
}

impl From<Vec3> for (f32, f32, f32) {
    fn from(v: Vec3) -> Self {
        (v.get_x(), v.get_y(), v.get_z())
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(a: [f32; 3]) -> Self {
        Vec3::new(a[0], a[1], a[2])
    }
}

impl From<Vec3> for [f32; 3] {
    fn from(v: Vec3) -> Self {
        [v.get_x(), v.get_y(), v.get_z()]
    }
}