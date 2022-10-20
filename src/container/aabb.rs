use crate::geometry::Box;
use crate::vector::Vector3;

/// Axis-aligned bounding box
pub struct AABB {
    bound: Box,
}

#[inline]
fn orzero(condition: bool, value: f32) -> f32 {
    if condition {
        value
    } else {
        0.0
    }
}

impl AABB {
    pub fn new(bound: Box) -> Self {
        Self { bound }
    }

    /// volume on the positive side of the plane
    pub fn vp(&self, normal: &Vector3) -> Vector3 {
        self.bound.origin
            + Vector3 {
                x: orzero(normal.x > 0.0, self.bound.size.x),
                y: orzero(normal.y > 0.0, self.bound.size.y),
                z: orzero(normal.z > 0.0, self.bound.size.z),
            }
    }

    /// volume on the negative side of the plane
    pub fn vn(&self, normal: &Vector3) -> Vector3 {
        self.bound.origin
            - Vector3 {
                x: orzero(normal.x < 0.0, self.bound.size.x),
                y: orzero(normal.y < 0.0, self.bound.size.y),
                z: orzero(normal.z < 0.0, self.bound.size.z),
            }
    }
}
