use std::ops;

use crate::vector::Vector3;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Quaternion {
    w: f32,
    x: f32,
    y: f32,
    z: f32,
}

impl Quaternion {
    pub fn new(w: f32, x: f32, y: f32, z: f32) -> Self {
        Self { w, x, y, z }
    }

    pub fn identity() -> Self {
        Self::new(1.0, 0.0, 0.0, 0.0)
    }

    pub fn conjugate(&self) -> Self {
        Self::new(self.w, -self.x, -self.y, -self.z)
    }

    pub fn dot(a: Self, b: Self) -> f32 {
        a.w * b.w + a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn magnitude(self) -> f32 {
        Self::dot(self, self).sqrt()
    }

    pub fn normalize(&mut self) {
        *self = self.normalized();
    }

    pub fn normalized(&self) -> Self {
        let m = self.magnitude();

        Self::new(self.w / m, self.x / m, self.y / m, self.z / m)
    }

    pub fn inverse(v: Quaternion) -> Self {
        v.conjugate() * (1. / v.magnitude().powi(2))
    }

    pub fn lerp(a: Self, b: Self, t: f32) -> Self {
        let t = t.max(0.).min(1.);
        ((1. - t) * a + t * b).normalized()
    }

    pub fn slerp(a: Self, b: Self, t: f32) -> Self {
        let t = t.max(0.).min(1.);
        let theta = Quaternion::dot(a, b).acos();
        let sine = theta.sin();

        if sine == 0. {
            return Quaternion::lerp(a, b, t);
        }

        let value = (((1. - t) * theta).sin() / sine) * a + ((t * theta).sin() / sine) * b;
        value.normalized()
    }
}

impl ops::Add for Quaternion {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(
            self.w + other.w,
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        )
    }
}

impl ops::Sub for Quaternion {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(
            self.w - other.w,
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
        )
    }
}

impl ops::Mul<f32> for Quaternion {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Self::new(
            self.w * scalar,
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
        )
    }
}

impl ops::Mul<Quaternion> for f32 {
    type Output = Quaternion;

    fn mul(self, q: Quaternion) -> Self::Output {
        Self::Output::new(self * q.w, self * q.x, self * q.y, self * q.z)
    }
}

impl ops::Mul for Quaternion {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let u = Vector3::new(self.x, self.y, self.z);
        let v = Vector3::new(other.x, other.y, other.z);
        let w = v * self.w + u * other.w + Vector3::cross(u, v);

        Self::Output::new(self.w * other.w - Vector3::dot(u, v), w.x, w.y, w.z)
    }
}

impl Default for Quaternion {
    fn default() -> Self {
        Self::identity()
    }
}
