use crate::geometry::Box;
use crate::utils::ternary;
use crate::vector::Vector3;

/// Axis-aligned bounding box
pub struct AABB {
    bounds: Box,
}

impl AABB {
    pub fn new(bounds: Box) -> Self {
        Self { bounds }
    }

    /// volume on the positive side of the plane
    pub fn vp(&self, normal: &Vector3) -> Vector3 {
        self.bounds.origin
            + Vector3 {
                x: ternary(normal.x > 0.0, self.bounds.size.x, 0.0),
                y: ternary(normal.y > 0.0, self.bounds.size.y, 0.0),
                z: ternary(normal.z > 0.0, self.bounds.size.z, 0.0),
            }
    }

    /// volume on the negative side of the plane
    pub fn vn(&self, normal: &Vector3) -> Vector3 {
        self.bounds.origin
            - Vector3 {
                x: ternary(normal.x < 0.0, self.bounds.size.x, 0.0),
                y: ternary(normal.y < 0.0, self.bounds.size.y, 0.0),
                z: ternary(normal.z < 0.0, self.bounds.size.z, 0.0),
            }
    }
}
