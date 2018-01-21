extern crate cgmath;
extern crate gl;
extern crate glutin;
extern crate image;

#[macro_use]
mod macros;

mod camera;
mod gfx;
mod input;
mod transform;
mod mesh;
mod vertex;

use std::io::{Cursor, Read};
use std::mem;

use camera::Camera;
use input::{KeyState, MouseState};
use gfx::buffer::{Buffer, BufferType};
use gfx::shader::{Program, Shader, ShaderStage, UniformValue};
use gfx::texture::{MagnifyFilter, MinifyFilter, Texture, WrapFunction};
use gfx::vertex_array::{VertexArray, VertexAttrib};
use transform::Transform;
use mesh::{DrawMode, Mesh};
use vertex::Vertex;

use cgmath::{Deg, Matrix4, Vector3};
use glutin::{ContextBuilder, CursorState, DeviceEvent, ElementState, Event, EventsLoop, GlContext,
             GlProfile, GlWindow, MouseButton, MouseCursor, MouseScrollDelta, VirtualKeyCode,
             WindowBuilder, WindowEvent};
use image::{DecodingResult, ImageDecoder};
use image::png::PNGDecoder;

// Shader sources
static VS_SRC: &'static str = include_res_str!("triangle.vs");
static FS_SRC: &'static str = include_res_str!("triangle.fs");

static TEST_PNG: &'static [u8] = include_res!("test.png");

fn update_perspective(w: u32, h: u32) -> Matrix4<f32> {
    cgmath::perspective(Deg(70.0), w as f32 / h as f32, 0.001, 1000.0)
}

fn main() {
    let mut events_loop = EventsLoop::new();
    let window = WindowBuilder::new().with_title("GL Sandbox");
    let context = ContextBuilder::new()
        .with_gl_profile(GlProfile::Core)
        .with_vsync(true);
    let gl_window = GlWindow::new(window, context, &events_loop).expect("failed to create window");

    let (mut width, mut height) = gl_window.window().get_inner_size().unwrap();
    gl_window.window().set_cursor(MouseCursor::NoneCursor);
    gl_window
        .window()
        .set_cursor_state(CursorState::Grab)
        .expect("could not grab cursor");

    unsafe {
        gl_window.make_current().unwrap();
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        gl::Enable(gl::CULL_FACE);
        gl::FrontFace(gl::CW);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    let texture = { Texture::new(PNGDecoder::new(Cursor::new(TEST_PNG))) };

    let mesh = {
        let vbo = {
            let verticies = [
                Vertex {
                    position: [-0.5, 0.5, 0.0].into(),
                    color: [1.0, 0.0, 0.0, 1.0].into(),
                    tex_coord: [0.0, 0.0].into(),
                    ..Default::default()
                },
                Vertex {
                    position: [0.5, 0.5, 0.0].into(),
                    color: [0.0, 1.0, 0.0, 1.0].into(),
                    tex_coord: [1.0, 0.0].into(),
                    ..Default::default()
                },
                Vertex {
                    position: [-0.5, -0.5, 0.0].into(),
                    color: [0.0, 0.0, 1.0, 1.0].into(),
                    tex_coord: [0.0, 1.0].into(),
                    ..Default::default()
                },
                Vertex {
                    position: [0.5, -0.5, 0.0].into(),
                    color: [1.0, 1.0, 1.0, 1.0].into(),
                    tex_coord: [1.0, 1.0].into(),
                    ..Default::default()
                },
            ];

            let vbo = Buffer::new(BufferType::Vertex);
            vbo.bind().buffer(&Vertex::into_bytes(&verticies));
            vbo
        };

        let indicies = [0u16, 1, 2, 2, 1, 3];
        let ibo = {
            let ibo = Buffer::new(BufferType::Index);
            unsafe {
                ibo.bind().buffer(std::slice::from_raw_parts(
                    indicies.as_ptr() as *const u8,
                    mem::size_of_val(&indicies),
                ))
            }
            ibo
        };
        let vao = VertexArray::new(
            vbo,
            Some(ibo),
            &[
                VertexAttrib {
                    location: 0,
                    size: 3,
                    stride: vertex::consts::SIZE as i32,
                    start: vertex::consts::POSITION_START,
                },
                VertexAttrib {
                    location: 1,
                    size: 4,
                    stride: vertex::consts::SIZE as i32,
                    start: vertex::consts::COLOR_START,
                },
                VertexAttrib {
                    location: 2,
                    size: 2,
                    stride: vertex::consts::SIZE as i32,
                    start: vertex::consts::TEXCOORD_START,
                },
                VertexAttrib {
                    location: 3,
                    size: 3,
                    stride: vertex::consts::SIZE as i32,
                    start: vertex::consts::NORMAL_START,
                },
            ],
        );
        let transform = Transform {
            position: Vector3::new(0.0, 0.0, -1.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
            ..Default::default()
        };
        Mesh {
            transform,
            vao,
            draw_mode: DrawMode::Triangles,
            range: 0..indicies.len(),
        }
    };

    let mut grabbed = true;
    let mut key_state = KeyState::new();
    let mut mouse_state = MouseState::new();

    let mut camera = Camera {
        position: Vector3::new(0.0, 0.0, 5.0),
        rotation: (0.0, 0.0),
    };
    let mut perspective = update_perspective(width, height);

    let program = Program::from_shaders(&[
        Shader::from_source(ShaderStage::Vertex, VS_SRC),
        Shader::from_source(ShaderStage::Fragment, FS_SRC),
    ]);
    let u_mvp = program.get_uniform_location("mvp");
    let u_tex = program.get_uniform_location("tex");

    let active_program = program.bind();
    active_program.uniform(u_tex, UniformValue::I1(0));
    let active_tex = texture.bind();
    active_tex.set_minify_filter(MinifyFilter::Linear);
    active_tex.set_magnify_filter(MagnifyFilter::Linear);
    active_tex.set_wrap_function((WrapFunction::Repeat, WrapFunction::Repeat));

    let mut running = true;
    while running {
        key_state = KeyState::from_last_frame(key_state);
        mouse_state = MouseState::from_last_frame(mouse_state);

        events_loop.poll_events(|event| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Closed => running = false,
                WindowEvent::Resized(w, h) => {
                    width = w;
                    height = h;
                    perspective = update_perspective(w, h);
                    unsafe {
                        gl::Viewport(0, 0, w as i32, h as i32);
                    }
                    gl_window.resize(w, h)
                }
                WindowEvent::KeyboardInput {
                    input:
                        glutin::KeyboardInput {
                            state,
                            virtual_keycode: Some(vk),
                            ..
                        },
                    ..
                } => match state {
                    ElementState::Pressed => key_state.pressed(vk),
                    ElementState::Released => key_state.released(vk),
                },
                WindowEvent::MouseInput { state, button, .. } => match state {
                    ElementState::Pressed => mouse_state.pressed(button),
                    ElementState::Released => mouse_state.released(button),
                },
                _ => (),
            },
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta: (x, y) } => {
                    if grabbed {
                        mouse_state.position = (-x as i32, -y as i32);
                    }
                }
                DeviceEvent::MouseWheel {
                    delta: MouseScrollDelta::LineDelta(_, y),
                } => mouse_state.mouse_wheel_delta = (y * 100.0) as i32,
                _ => (),
            },
            _ => (),
        });

        camera.rotation.0 += mouse_state.position.1 as f32 / 3.0;
        camera.rotation.1 += mouse_state.position.0 as f32 / 3.0;
        if mouse_state.pressed.contains(&MouseButton::Left) {
            grabbed = true;

            gl_window.window().set_cursor(MouseCursor::NoneCursor);
            gl_window
                .window()
                .set_cursor_state(CursorState::Grab)
                .unwrap();
        }
        if key_state.pressed.contains(&VirtualKeyCode::Escape) {
            grabbed = false;

            gl_window.window().set_cursor(MouseCursor::Default);
            gl_window
                .window()
                .set_cursor_state(CursorState::Normal)
                .unwrap();
        }
        if key_state.down.contains(&VirtualKeyCode::W) {
            camera.position += camera.get_forward() * 0.1;
        }
        if key_state.down.contains(&VirtualKeyCode::A) {
            camera.position -= camera.get_right() * 0.1;
        }
        if key_state.down.contains(&VirtualKeyCode::S) {
            camera.position -= camera.get_forward() * 0.1;
        }
        if key_state.down.contains(&VirtualKeyCode::D) {
            camera.position += camera.get_right() * 0.1;
        }
        if key_state.down.contains(&VirtualKeyCode::Space) {
            camera.position += camera::UP * 0.1;
        }
        if key_state.down.contains(&VirtualKeyCode::LShift) {
            camera.position -= camera::UP * 0.1;
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let transform: Matrix4<f32> = (&mesh.transform).into();
        let mvp = perspective * camera.get_view_matrix() * transform;
        active_program.uniform(u_mvp, UniformValue::Matrix4(mvp));
        mesh.draw(&active_program);

        gl_window.swap_buffers().unwrap();
    }
}
