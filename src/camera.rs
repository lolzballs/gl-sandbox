use cgmath::{Deg, Euler, Matrix4, Quaternion, Rotation, Rotation3, Vector3};

pub const UP: Vector3<f32> = Vector3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

pub struct Camera {
    pub position: Vector3<f32>,
    pub rotation: (f32, f32),
}

impl Camera {
    pub fn get_forward(&self) -> Vector3<f32> {
        self.get_rotation() * -Vector3::unit_z()
    }

    pub fn get_right(&self) -> Vector3<f32> {
        self.get_forward().cross(UP)
    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::from(self.get_rotation().invert()) * Matrix4::from_translation(-self.position)
    }

    fn get_rotation(&self) -> Quaternion<f32> {
        Quaternion::from_angle_y(Deg(self.rotation.1))
            * Quaternion::from_angle_x(Deg(self.rotation.0))
    }
}
