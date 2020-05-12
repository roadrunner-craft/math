use crate::container::Volume;
use crate::vector::Vector3;

/// Axis-aligned bounding box
pub struct AABB {
    volume: Volume,
}

impl AABB {
    pub fn new(volume: Volume) -> Self {
        Self { volume }
    }

    pub fn get_vn(&self, normal: &Vector3) -> Vector3 {
        Vector3 {
            x: self.volume.x as f32
                + if normal.x < 0.0 {
                    self.volume.width as f32
                } else {
                    0.0
                },
            y: self.volume.y as f32
                + if normal.y < 0.0 {
                    self.volume.height as f32
                } else {
                    0.0
                },
            z: self.volume.z as f32
                + if normal.z < 0.0 {
                    self.volume.depth as f32
                } else {
                    0.0
                },
        }
    }

    pub fn get_vp(&self, normal: &Vector3) -> Vector3 {
        Vector3 {
            x: self.volume.x as f32
                + if normal.x > 0.0 {
                    self.volume.width as f32
                } else {
                    0.0
                },
            y: self.volume.y as f32
                + if normal.y > 0.0 {
                    self.volume.height as f32
                } else {
                    0.0
                },
            z: self.volume.z as f32
                + if normal.z > 0.0 {
                    self.volume.depth as f32
                } else {
                    0.0
                },
        }
    }
}
