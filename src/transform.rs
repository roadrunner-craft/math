use crate::matrix::Matrix4;
use crate::vector::Vector3;

pub struct Transform {
    position: Vector3,
    rotation: Vector3,
    scale: Vector3,
    m: Option<Matrix4>,
}

impl Transform {
    pub fn new_position(x: f32, y: f32, z: f32) -> Self {
        let mut t = Self {
            position: Vector3 { x, y, z },
            rotation: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            scale: Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            m: None,
        };

        t.generate_matrix();
        t
    }

    pub fn new(position: Vector3, rotation: Vector3, scale: Vector3) -> Self {
        let mut t = Self {
            position,
            rotation,
            scale,
            m: None,
        };

        t.generate_matrix();
        t
    }

    pub fn position(&self) -> Vector3 {
        self.position
    }

    pub fn set_position(&mut self, value: Vector3) {
        self.position = value;
        self.generate_matrix();
    }

    pub fn scale(&self) -> Vector3 {
        self.scale
    }

    pub fn set_scale(&mut self, value: Vector3) {
        self.scale = value;
        self.generate_matrix();
    }

    pub fn euler_angle(&self) -> Vector3 {
        self.rotation
    }

    pub fn set_euler_angles(&mut self, value: Vector3) {
        self.rotation = value;
        self.generate_matrix();
    }

    pub fn matrix(&self) -> &Matrix4 {
        &self.m.as_ref().unwrap()
    }

    fn generate_matrix(&mut self) {
        let ((cx, sx), (cy, sy), (cz, sz)) = (
            (
                self.rotation.x.to_radians().cos(),
                self.rotation.x.to_radians().sin(),
            ),
            (
                self.rotation.y.to_radians().cos(),
                self.rotation.y.to_radians().sin(),
            ),
            (
                self.rotation.z.to_radians().cos(),
                self.rotation.z.to_radians().sin(),
            ),
        );

        let m11 = cy * cz * self.scale.x;
        let m12 = cy * sz * self.scale.y;
        let m13 = -sy * self.scale.z;
        let m14 = cy * cz * self.position.x + sz * cy * self.position.y - sy * self.position.z;

        let m21 = (sx * sy * cz - sz * cx) * self.scale.x;
        let m22 = (cx * cz + sz * sx * sy) * self.scale.y;
        let m23 = sx * cy * self.scale.z;
        let m24 = (sy * sx * cz - sz * cx) * self.position.x
            + (cx * cz + sz * sy * sx) * self.position.y
            + cy * sx * self.position.z;

        let m31 = (cx * sy * cz + sz * sx) * self.scale.x;
        let m32 = (-sx * cz + sz * cx * sy) * self.scale.y;
        let m33 = (cx * cy) * self.scale.z;
        let m34 = (cx * sy * cz + sz * cx * sy) * self.position.x
            + (sz * cx * sy - sx * cz) * self.position.y
            + (cx * cy) * self.position.z;

        self.m = Some(Matrix4([
            [m11, m12, m13, m14],
            [m21, m22, m23, m24],
            [m31, m32, m33, m34],
            [0.0, 0.0, 0.0, 1.0],
        ]));
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new_position(0.0, 0.0, 0.0)
    }
}
