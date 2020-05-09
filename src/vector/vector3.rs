use std::fmt;
use std::ops;

#[derive(Default, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self::default()
    }

    pub fn forward() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    pub fn backward() -> Self {
        Self::new(0.0, 0.0, -1.0)
    }

    pub fn up() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    pub fn down() -> Self {
        Self::new(0.0, -1.0, 0.0)
    }

    pub fn right() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    pub fn left() -> Self {
        Self::new(-1.0, 0.0, 0.0)
    }

    pub fn identity() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn dot(a: Self, b: Self) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: Self, b: Self) -> Self {
        Self::new(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        )
    }

    pub fn project(a: Self, b: Self) -> Self {
        (Vector3::dot(a, b) / Vector3::dot(b, b)) * b
    }

    pub fn magnitude(self) -> f32 {
        Self::dot(self, self).sqrt()
    }

    pub fn normalize(&mut self) {
        *self = self.normalized();
    }

    pub fn normalized(self) -> Self {
        let m = self.magnitude();

        Self::new(self.x / m, self.y / m, self.z / m)
    }
}

impl ops::Add for Vector3 {
    type Output = Self;

    fn add(self, other: Vector3) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl ops::AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self::new(self.x + other.x, self.y + other.y, self.z + other.z);
    }
}

impl ops::Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Vector3) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl ops::SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self::new(self.x - other.x, self.y - other.y, self.z - other.z);
    }
}

impl ops::Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, v: Vector3) -> Self::Output {
        Self::Output::new(self * v.x, self * v.y, self * v.z)
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl ops::MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, scalar: f32) {
        *self = Self::new(self.x * scalar, self.y * scalar, self.z * scalar);
    }
}

impl ops::Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl fmt::Debug for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Vector3 {{ x: {}, y: {}, z: {} }}",
            self.x, self.y, self.z
        )
    }
}
