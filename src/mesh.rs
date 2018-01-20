use std::mem;

use shader::ActiveProgram;

use gl;
use gl::types::*;

pub struct VertexAttrib {
    pub location: GLuint,
    pub size: GLint,
    pub stride: GLsizei,
    pub start: usize,
}

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

    pub fn draw(&self, _: &ActiveProgram, prim: GLenum, start: GLint, len: usize) {
        unsafe {
            match self.vao.ibo {
                Some(_) => gl::DrawElements(
                    prim,
                    len as GLsizei,
                    gl::UNSIGNED_SHORT,
                    start as *const i32 as *const _,
                ),
                None => gl::DrawArrays(prim, start, len as GLsizei),
            }
        }
    }
}

impl<'a> Drop for ActiveVAO<'a> {
    fn drop(&mut self) {
        unsafe { gl::BindVertexArray(0) }
    }
}

pub struct Buffer {
    ty: BufferType,
    id: GLuint,
}

impl Buffer {
    pub fn new(ty: BufferType) -> Self {
        unsafe {
            let mut id = 0;
            gl::GenBuffers(1, &mut id);
            Buffer { ty, id }
        }
    }

    pub fn bind(&self) -> ActiveBuffer {
        ActiveBuffer::new(self)
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

pub struct ActiveBuffer<'a> {
    buffer: &'a Buffer,
}

impl<'a> ActiveBuffer<'a> {
    fn new(buffer: &'a Buffer) -> Self {
        unsafe { gl::BindBuffer(buffer.ty.into(), buffer.id) }
        ActiveBuffer { buffer }
    }

    pub fn buffer(&self, data: &[u8]) {
        unsafe {
            gl::BufferData(
                self.buffer.ty.into(),
                mem::size_of_val(data) as GLsizeiptr,
                data.as_ptr() as *const _,
                gl::STATIC_DRAW,
            )
        }
    }
}

impl<'a> Drop for ActiveBuffer<'a> {
    fn drop(&mut self) {
        unsafe { gl::BindBuffer(self.buffer.ty.into(), 0) }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum BufferType {
    Vertex,
    Index,
}

impl From<BufferType> for GLenum {
    fn from(buffer: BufferType) -> Self {
        match buffer {
            BufferType::Vertex => gl::ARRAY_BUFFER,
            BufferType::Index => gl::ELEMENT_ARRAY_BUFFER,
        }
    }
}
