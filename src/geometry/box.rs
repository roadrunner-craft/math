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

    pub fn contains(self, point: Vector3) -> bool {
        Line::new(self.origin.x, self.size.x).contains(point.x)
            && Line::new(self.origin.y, self.size.y).contains(point.y)
            && Line::new(self.origin.z, self.size.z).contains(point.z)
    }

    pub fn intersects(&self, other: &Box) -> bool {
        Line::new(self.origin.x, self.size.x).intersects(Line::new(other.origin.x, other.size.x))
            && Line::new(self.origin.y, self.size.y)
                .intersects(Line::new(other.origin.y, other.size.y))
            && Line::new(self.origin.z, self.size.z)
                .intersects(Line::new(other.origin.z, other.size.z))
    }
}
