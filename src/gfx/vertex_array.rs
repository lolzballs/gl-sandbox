use std::mem;

use gfx::shader::ActiveProgram;
use gfx::buffer::Buffer;

use gl;
use gl::types::*;

pub struct VertexAttrib {
    pub location: GLuint,
    pub size: GLint,
    pub stride: GLsizei,
    pub start: usize,
}

#[derive(Debug)]
pub struct VertexArray {
    id: GLuint,
    vbo: Buffer,
    ibo: Option<Buffer>,
}

impl VertexArray {
    pub fn new(vbo: Buffer, ibo: Option<Buffer>, attribs: &[VertexAttrib]) -> Self {
        let vao = unsafe {
            let mut id = 0;
            gl::GenVertexArrays(1, &mut id);

            VertexArray { id, vbo, ibo }
        };

        {
            let active = vao.bind();

            // Don't unbind in the vao setup
            mem::forget(vao.vbo.bind());
            if let Some(ref ibo) = vao.ibo {
                mem::forget(ibo.bind());
            }
            for a in attribs {
                active.vertex_attrib_pointer(a.location, a.size, a.stride, a.start);
            }
        }

        vao
    }

    pub fn bind(&self) -> ActiveVAO {
        ActiveVAO::new(self)
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.id) }
    }
}

pub struct ActiveVAO<'a> {
    vao: &'a VertexArray,
}

impl<'a> ActiveVAO<'a> {
    fn new(vao: &'a VertexArray) -> Self {
        unsafe { gl::BindVertexArray(vao.id) }
        ActiveVAO { vao }
    }

    fn vertex_attrib_pointer(&self, location: GLuint, size: GLint, stride: GLsizei, start: usize) {
        unsafe {
            gl::EnableVertexAttribArray(location);
            gl::VertexAttribPointer(
                location,
                size,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                stride,
                start as *const GLvoid,
            );
        }
    }

    pub fn draw(&self, _: &ActiveProgram, prim: GLenum, start: GLint, len: GLsizei) {
        unsafe {
            match self.vao.ibo {
                Some(_) => gl::DrawElements(
                    prim,
                    len as GLsizei,
                    gl::UNSIGNED_SHORT,
                    start as *const i32 as *const _,
                ),
                None => gl::DrawArrays(prim, start, len),
            }
        }
    }
}

impl<'a> Drop for ActiveVAO<'a> {
    fn drop(&mut self) {
        unsafe { gl::BindVertexArray(0) }
    }
}
