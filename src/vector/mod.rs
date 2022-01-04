mod vector2;
mod vector3;

pub use self::vector2::Vector2;
pub use self::vector3::Vector3;

pub trait Vector {}

impl Vector for Vector2 {}

impl Vector for Vector3 {}
