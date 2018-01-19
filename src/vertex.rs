use cgmath::{Vector2, Vector3, Vector4};

pub mod consts {
    pub const SIZE_F32: usize = 12;
    pub const SIZE: usize = SIZE_F32 * 4;
}

pub struct Vertex {
    pub position: Vector3<f32>,
    pub color: Vector4<f32>,
    pub tex_coord: Vector2<f32>,
    pub normal: Vector3<f32>,
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: Vector3::new(0.0, 0.0, 0.0),
            color: Vector4::new(1.0, 1.0, 1.0, 1.0),
            tex_coord: Vector2::new(0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}

impl Vertex {
    pub fn into_bytes(verticies: &[Vertex]) -> Vec<u8> {
        let mut vec = Vec::with_capacity(consts::SIZE * verticies.len());
        for v in verticies.iter() {
            let t: [u8; consts::SIZE] = v.into();
            vec.extend(t.iter());
        }
        vec
    }
}

impl<'a> Into<[u8; consts::SIZE]> for &'a Vertex {
    fn into(self) -> [u8; consts::SIZE] {
        unsafe {
            ::std::mem::transmute([
                self.position.x,
                self.position.y,
                self.position.z,
                self.color.x,
                self.color.y,
                self.color.z,
                self.color.w,
                self.tex_coord.x,
                self.tex_coord.y,
                self.normal.x,
                self.normal.y,
                self.normal.z,
            ])
        }
    }
}
