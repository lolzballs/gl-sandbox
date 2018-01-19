use std::mem;

use shader::Program;
use vertex::Vertex;

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
}

impl VertexArray {
    pub fn new(vbo: Buffer, attribs: &[VertexAttrib]) -> Self {
        let vao = unsafe {
            let mut id = 0;
            gl::GenVertexArrays(1, &mut id);

            VertexArray { id, vbo }
        };

        vao.bind();
        vao.vbo.bind();
        for a in attribs {
            vao.vertex_attrib_pointer(a.location, a.size, a.stride, a.start);
        }
        vao.unbind();

        vao
    }

    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
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

    pub fn draw_arrays(&self, program: &Program, prim: GLenum, start: GLint, len: GLsizei) {
        self.bind();
        program.bind();
        unsafe {
            gl::DrawArrays(prim, start, len);
        }
        program.unbind();
        self.unbind();
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
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

    fn bind(&self) {
        unsafe { gl::BindBuffer(self.ty.into(), self.id) }
    }

    fn unbind(&self) {
        unsafe { gl::BindBuffer(self.ty.into(), 0) }
    }

    pub fn buffer_verticies(&self, verticies: &[Vertex]) {
        assert!(self.ty == BufferType::Vertex);

        self.bind();
        unsafe {
            gl::BufferData(
                self.ty.into(),
                mem::size_of_val(verticies) as GLsizeiptr,
                Vertex::into_bytes(verticies).as_ptr() as *const _,
                gl::STATIC_DRAW,
            )
        }
        self.unbind();
    }

    pub fn buffer_indicies(&self, indicies: &[usize]) {
        assert!(self.ty == BufferType::Index);

        self.bind();
        unsafe {
            gl::BufferData(
                self.ty.into(),
                mem::size_of_val(indicies) as GLsizeiptr,
                mem::transmute(&indicies[0]),
                gl::STATIC_DRAW,
            )
        }
        self.unbind();
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
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
