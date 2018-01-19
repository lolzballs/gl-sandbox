extern crate cgmath;
extern crate gl;
extern crate glutin;

mod mesh;
mod shader;
mod vertex;

use mesh::{Buffer, BufferType, VertexArray, VertexAttrib};
use shader::{Program, Shader, ShaderStage};
use vertex::Vertex;

use glutin::{ContextBuilder, Event, EventsLoop, GlContext, GlWindow, WindowBuilder, WindowEvent};

// Vertex data

// Shader sources
static VS_SRC: &'static str = include_str!("triangle.vs");
static FS_SRC: &'static str = include_str!("triangle.fs");

fn main() {
    let mut events_loop = EventsLoop::new();
    let window = WindowBuilder::new().with_title("GL Sandbox");
    let context = ContextBuilder::new().with_vsync(true);
    let gl_window = GlWindow::new(window, context, &events_loop).expect("failed to create window");

    unsafe {
        gl_window.make_current().unwrap();
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    let vbo = {
        let verticies: [Vertex; 3] = [
            Vertex {
                position: [0.0, 0.5, 0.0].into(),
                color: [0.0, 1.0, 0.0, 1.0].into(),
                ..Default::default()
            },
            Vertex {
                position: [0.5, -0.5, 0.0].into(),
                color: [1.0, 1.0, 0.0, 1.0].into(),
                ..Default::default()
            },
            Vertex {
                position: [-0.5, -0.5, 0.0].into(),
                color: [0.0, 1.0, 1.0, 1.0].into(),
                ..Default::default()
            },
        ];

        let vbo = Buffer::new(BufferType::Vertex);
        vbo.buffer_verticies(&verticies);
        vbo
    };

    let indicies = [0, 1, 2];
    let ibo = {
        let ibo = Buffer::new(BufferType::Index);
        ibo.buffer_indicies(&indicies);
        ibo
    };

    let program = Program::from_shaders(&[
        Shader::from_source(ShaderStage::Vertex, VS_SRC),
        Shader::from_source(ShaderStage::Fragment, FS_SRC),
    ]);

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
        ],
    );

    let mut running = true;
    while running {
        events_loop.poll_events(|event| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Closed => running = false,
                WindowEvent::Resized(w, h) => gl_window.resize(w, h),
                _ => (),
            },
            _ => (),
        });

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        vao.draw(&program, gl::TRIANGLES, 0, indicies.len());
        gl_window.swap_buffers().unwrap();
    }
}
