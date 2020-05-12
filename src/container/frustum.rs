use crate::container::AABB;
use crate::matrix::Matrix4;
use crate::vector::Vector3;

enum FrustumPlaneType {
    Near,
    Far,
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Clone, Default, Debug)]
struct FrustumPlane {
    normal: Vector3,
    origin_distance: f32,
}

impl FrustumPlane {
    fn normalize(&mut self) {
        let length = self.normal.magnitude();
        self.normal = self.normal * (1.0 / length);
        self.origin_distance = self.origin_distance / length;
    }
}

#[derive(Clone, Default, Debug)]
pub struct Frustum {
    planes: [FrustumPlane; 6],
}

impl Frustum {
    pub fn new(projection_view: &Matrix4) -> Self {
        let mut f = Self::default();
        f.update(projection_view);
        f
    }

    /// m: projection * view matrix
    pub fn update(&mut self, m: &Matrix4) {
        // near
        self.planes[FrustumPlaneType::Near as usize] = FrustumPlane {
            normal: Vector3 {
                x: m[3][0] + m[2][0],
                y: m[3][1] + m[2][1],
                z: m[3][2] + m[2][2],
            },
            origin_distance: m[3][3] + m[2][3],
        };

        self.planes[FrustumPlaneType::Far as usize] = FrustumPlane {
            normal: Vector3 {
                x: m[3][0] - m[2][0],
                y: m[3][1] - m[2][1],
                z: m[3][2] - m[2][2],
            },
            origin_distance: m[3][3] - m[2][3],
        };

        self.planes[FrustumPlaneType::Left as usize] = FrustumPlane {
            normal: Vector3 {
                x: m[3][0] + m[0][0],
                y: m[3][1] + m[0][1],
                z: m[3][2] + m[0][2],
            },
            origin_distance: m[3][3] + m[0][3],
        };

        self.planes[FrustumPlaneType::Right as usize] = FrustumPlane {
            normal: Vector3 {
                x: m[3][0] - m[0][0],
                y: m[3][1] - m[0][1],
                z: m[3][2] - m[0][2],
            },
            origin_distance: m[3][3] - m[0][3],
        };

        self.planes[FrustumPlaneType::Top as usize] = FrustumPlane {
            normal: Vector3 {
                x: m[3][0] - m[1][0],
                y: m[3][1] - m[1][1],
                z: m[3][2] - m[1][2],
            },
            origin_distance: m[3][3] - m[1][3],
        };

        self.planes[FrustumPlaneType::Bottom as usize] = FrustumPlane {
            normal: Vector3 {
                x: m[3][0] + m[1][0],
                y: m[3][1] + m[1][1],
                z: m[3][2] + m[1][2],
            },
            origin_distance: m[3][3] + m[1][3],
        };

        for plane in self.planes.iter_mut() {
            plane.normalize();
        }
    }

    pub fn contains(&self, aabb: &AABB) -> bool {
        let mut result = true;

        for plane in self.planes.iter() {
            let vp = aabb.get_vp(&plane.normal);

            if Vector3::dot(vp, plane.normal) + plane.origin_distance < 0.0 {
                return false;
            }

            let vn = aabb.get_vp(&plane.normal);

            if Vector3::dot(vn, plane.normal) + plane.origin_distance < 0.0 {
                result = true;
            }
        }

        result
    }
}
