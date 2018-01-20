use cgmath::{Matrix4, One, Quaternion, Vector3};

#[derive(Copy, Clone, Debug)]
pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub scale: Vector3<f32>,
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Quaternion::one(),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

impl Into<Matrix4<f32>> for Transform {
    fn into(self) -> Matrix4<f32> {
        Matrix4::from_translation(self.position)
            * Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)
            * Matrix4::from(self.rotation)
    }
}
