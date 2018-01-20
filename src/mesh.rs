use std::ops::Range;

use gfx::shader::ActiveProgram;
use gfx::vertex_array::VertexArray;
use transform::Transform;

use gl;
use gl::types::GLenum;

#[derive(Debug)]
pub struct Mesh {
    pub transform: Transform,
    pub vao: VertexArray,
    pub draw_mode: DrawMode,
    pub range: Range<usize>,
}

impl Mesh {
    pub fn draw(&self, program: &ActiveProgram) {
        self.vao.bind().draw(
            program,
            self.draw_mode.into(),
            self.range.start as i32,
            self.range.end as i32,
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub enum DrawMode {
    Triangles,
}

impl Into<GLenum> for DrawMode {
    fn into(self) -> GLenum {
        match self {
            DrawMode::Triangles => gl::TRIANGLES,
        }
    }
}
