use crate::geometry::Line;
use crate::vector::Vector3;

#[derive(Copy, Clone)]
pub struct Box {
    pub origin: Vector3,
    pub size: Vector3,
}

impl Box {
    pub fn new(origin: Vector3, width: f32, height: f32, depth: f32) -> Self {
        Self {
            origin,
            size: Vector3::new(width, height, depth),
        }
    }

    pub fn cube(origin: Vector3, size: f32) -> Self {
        Self::new(origin, size, size, size)
    }

    pub fn x(&self) -> Line {
        Line::new(self.origin.x, self.size.x)
    }

    pub fn y(&self) -> Line {
        Line::new(self.origin.y, self.size.y)
    }

    pub fn z(&self) -> Line {
        Line::new(self.origin.z, self.size.z)
    }

    pub fn contains(self, point: Vector3) -> bool {
        self.x().contains(point.x) && self.y().contains(point.y) && self.z().contains(point.z)
    }

    pub fn intersects(&self, other: &Box) -> bool {
        self.x().intersects(other.x())
            && self.y().intersects(other.y())
            && self.z().intersects(other.z())
    }
}
