use std::ffi::CString;
use std::ptr;
use std::str;

use cgmath::{Matrix, Matrix4};
use gl;
use gl::types::*;

pub enum UniformValue {
    Matrix4(Matrix4<f32>),
    I1(i32),
}

pub struct Program {
    pub id: GLuint,
}

impl Program {
    pub fn from_shaders<'a, I>(shaders: I) -> Self
    where
        I: IntoIterator<Item = &'a Shader>,
    {
        let program = unsafe {
            let program = gl::CreateProgram();
            for shader in shaders {
                gl::AttachShader(program, shader.id);
            }
            gl::LinkProgram(program);

            // Get the link status
            let mut status = gl::FALSE as GLint;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

            // Fail on error
            if status != (gl::TRUE as GLint) {
                let mut len: GLint = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
                gl::GetProgramInfoLog(
                    program,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );
                panic!(
                    "{}",
                    str::from_utf8(&buf)
                        .ok()
                        .expect("ProgramInfoLog not valid utf8",)
                );
            }
            program
        };

        Program { id: program }
    }

    pub fn bind(&self) -> ActiveProgram {
        ActiveProgram::new(self)
    }

    pub fn get_uniform_location(&self, name: &str) -> GLint {
        unsafe {
            let c_str = CString::new(name.as_bytes()).unwrap();
            gl::GetUniformLocation(self.id, c_str.as_ptr())
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

pub struct ActiveProgram<'a> {
    program: &'a Program,
}

impl<'a> ActiveProgram<'a> {
    fn new(program: &'a Program) -> Self {
        unsafe { gl::UseProgram(program.id) }
        ActiveProgram { program }
    }

    pub fn uniform(&self, location: GLint, val: UniformValue) {
        unsafe {
            match val {
                UniformValue::Matrix4(mat4) => {
                    gl::UniformMatrix4fv(location, 1, gl::FALSE, mat4.as_ptr())
                }
                UniformValue::I1(i) => gl::Uniform1i(location, i),
            }
        }
    }
}

impl<'a> Drop for ActiveProgram<'a> {
    fn drop(&mut self) {
        unsafe { gl::UseProgram(0) }
    }
}

pub struct Shader {
    stage: ShaderStage,
    pub id: GLuint,
}

impl Shader {
    pub fn from_source(stage: ShaderStage, source: &str) -> Self {
        Self::compile_source(ShaderSource::new(stage, source))
    }

    pub fn compile_source<'a>(source: ShaderSource<'a>) -> Self {
        let id = unsafe {
            let shader = gl::CreateShader(source.stage.into());
            let c_str = CString::new(source.src.as_bytes()).unwrap();
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

            // Fail on error
            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
                gl::GetShaderInfoLog(
                    shader,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );
                panic!(
                    "{}",
                    str::from_utf8(&buf)
                        .ok()
                        .expect("ShaderInfoLog not valid utf8",)
                );
            }
            shader
        };
        Shader {
            stage: source.stage,
            id: id,
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

pub struct ShaderSource<'a> {
    stage: ShaderStage,
    src: &'a str,
}

impl<'a> ShaderSource<'a> {
    pub fn new(stage: ShaderStage, src: &'a str) -> Self {
        ShaderSource {
            stage: stage,
            src: src,
        }
    }
}

#[derive(Clone, Copy)]
pub enum ShaderStage {
    Vertex,
    Fragment,
}

impl From<ShaderStage> for GLenum {
    fn from(stage: ShaderStage) -> Self {
        match stage {
            ShaderStage::Vertex => gl::VERTEX_SHADER,
            ShaderStage::Fragment => gl::FRAGMENT_SHADER,
        }
    }
}
