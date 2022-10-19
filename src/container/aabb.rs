use crate::geometry::Box;
use crate::vector::Vector3;

/// Axis-aligned bounding box
pub struct AABB {
    volume: Box,
}

impl AABB {
    pub fn new(volume: Box) -> Self {
        Self { volume }
    }

    /// volume on the positive side of the plane
    pub fn vp(&self, normal: &Vector3) -> Vector3 {
        self.volume.origin
            + Vector3 {
                x: (normal.x.signum() * self.volume.size.x).max(0.0),
                y: (normal.y.signum() * self.volume.size.y).max(0.0),
                z: (normal.z.signum() * self.volume.size.z).max(0.0),
            }
    }

    /// volume on the negative side of the plane
    pub fn vn(&self, normal: &Vector3) -> Vector3 {
        self.volume.origin
            - Vector3 {
                x: (normal.x.signum() * self.volume.size.x).min(0.0),
                y: (normal.y.signum() * self.volume.size.y).min(0.0),
                z: (normal.z.signum() * self.volume.size.z).min(0.0),
            }
    }
}
