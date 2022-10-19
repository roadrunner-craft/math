use std::fmt;
use std::ops;

#[derive(Default, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::default()
    }

    pub fn up() -> Self {
        Self::new(0.0, 1.0)
    }

    pub fn down() -> Self {
        Self::new(0.0, -1.0)
    }

    pub fn right() -> Self {
        Self::new(1.0, 0.0)
    }

    pub fn left() -> Self {
        Self::new(-1.0, 0.0)
    }

    pub fn identity() -> Self {
        Self::new(1.0, 1.0)
    }

    pub fn dot(a: Self, b: Self) -> f32 {
        a.x * b.x + a.y * b.y
    }

    pub fn project(a: Self, b: Self) -> Self {
        (Vector2::dot(a, b) / Vector2::dot(b, b)) * b
    }

    pub fn magnitude(self) -> f32 {
        Self::dot(self, self).sqrt()
    }

    pub fn normalize(&mut self) {
        *self = self.normalized();
    }

    pub fn normalized(self) -> Self {
        let m = self.magnitude();

        Self::new(self.x / m, self.y / m)
    }

    pub fn x(self) -> Self {
        Self { x: self.x, y: 0.0 }
    }

    pub fn y(self) -> Self {
        Self { x: 0.0, y: self.y }
    }
}

impl ops::Add for Vector2 {
    type Output = Self;

    fn add(self, other: Vector2) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl ops::AddAssign for Vector2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self::new(self.x + other.x, self.y + other.y);
    }
}

impl ops::Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Vector2) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl ops::SubAssign for Vector2 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self::new(self.x - other.x, self.y - other.y);
    }
}

impl ops::Mul<Vector2> for f32 {
    type Output = Vector2;

    fn mul(self, v: Vector2) -> Self::Output {
        Self::Output::new(self * v.x, self * v.y)
    }
}

impl ops::Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Self::new(self.x * scalar, self.y * scalar)
    }
}

impl ops::MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, scalar: f32) {
        *self = Self::new(self.x * scalar, self.y * scalar);
    }
}

impl ops::Neg for Vector2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl fmt::Debug for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vector2 {{ x: {}, y: {} }}", self.x, self.y)
    }
}
