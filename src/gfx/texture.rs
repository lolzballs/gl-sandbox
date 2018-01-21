use gl;
use gl::types::*;
use image::{ColorType, DecodingResult, ImageDecoder};

pub struct Texture {
    id: GLuint,
}

impl Texture {
    pub fn new<I>(image: I) -> Self
    where
        I: ImageDecoder,
    {
        let texture = unsafe {
            let mut id = 0;
            gl::GenTextures(1, &mut id);
            Texture { id }
        };
        texture.bind().write(image);
        texture
    }

    pub fn bind(&self) -> ActiveTexture {
        ActiveTexture::new(self)
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id) }
    }
}

pub struct ActiveTexture<'a> {
    texture: &'a Texture,
}

impl<'a> ActiveTexture<'a> {
    fn new(texture: &'a Texture) -> Self {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, texture.id) }
        ActiveTexture { texture }
    }

    pub fn set_wrap_function(&self, wrap: (WrapFunction, WrapFunction)) {
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap.0.into());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap.1.into());
        }
    }

    pub fn set_minify_filter(&self, filter: MinifyFilter) {
        unsafe { gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, filter.into()) }
    }

    pub fn set_magnify_filter(&self, filter: MagnifyFilter) {
        unsafe { gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filter.into()) }
    }

    fn write<I>(&self, mut image: I)
    where
        I: ImageDecoder,
    {
        let (width, height) = image.dimensions().unwrap();
        let (int_format, format) = match image.colortype().unwrap() {
            ColorType::RGB(_) => (gl::RGB8, gl::RGB),
            ColorType::RGBA(_) => (gl::RGBA8, gl::RGBA),
            _ => unimplemented!(),
        };
        unsafe {
            match image.read_image().unwrap() {
                DecodingResult::U8(v) => {
                    gl::TexImage2D(
                        gl::TEXTURE_2D,
                        0,
                        int_format as i32,
                        width as i32,
                        height as i32,
                        0,
                        format,
                        gl::UNSIGNED_BYTE,
                        v.as_ptr() as *const _,
                    );
                }
                _ => unimplemented!(),
            }
        }
    }
}

impl<'a> Drop for ActiveTexture<'a> {
    fn drop(&mut self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, 0) }
    }
}

pub enum MinifyFilter {
    Nearest,
    Linear,
    NearestMipmapNearest,
    LinearMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapLinear,
}

impl Into<GLint> for MinifyFilter {
    fn into(self) -> GLint {
        match self {
            MinifyFilter::Nearest => gl::NEAREST as i32,
            MinifyFilter::Linear => gl::LINEAR as i32,
            MinifyFilter::NearestMipmapNearest => gl::NEAREST_MIPMAP_NEAREST as i32,
            MinifyFilter::LinearMipmapNearest => gl::LINEAR_MIPMAP_NEAREST as i32,
            MinifyFilter::NearestMipmapLinear => gl::NEAREST_MIPMAP_LINEAR as i32,
            MinifyFilter::LinearMipmapLinear => gl::LINEAR_MIPMAP_LINEAR as i32,
        }
    }
}

pub enum MagnifyFilter {
    Nearest,
    Linear,
}

impl Into<GLint> for MagnifyFilter {
    fn into(self) -> GLint {
        match self {
            MagnifyFilter::Nearest => gl::NEAREST as i32,
            MagnifyFilter::Linear => gl::LINEAR as i32,
        }
    }
}

pub enum WrapFunction {
    Repeat,
    Mirror,
    Clamp,
    MirrorClamp,
}

impl Into<GLint> for WrapFunction {
    fn into(self) -> GLint {
        match self {
            WrapFunction::Repeat => gl::REPEAT as i32,
            WrapFunction::Mirror => gl::MIRRORED_REPEAT as i32,
            WrapFunction::Clamp => gl::CLAMP_TO_EDGE as i32,
            WrapFunction::MirrorClamp => gl::MIRROR_CLAMP_TO_EDGE as i32,
        }
    }
}
