use std::mem;

use gl;
use gl::types::*;

#[derive(Debug)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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
