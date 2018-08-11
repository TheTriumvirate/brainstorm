/*
 * Very inspired from kiss3d's implementation of window and context
 * link: https://github.com/sebcrozet/kiss3d
 */
extern crate gl;
extern crate glutin;

use std::ffi::CString;
use std::iter;
use std::mem;
use std::ptr;

use window::abstract_window::*;

use self::glutin::dpi::*;
use self::glutin::Api::OpenGl;
use self::glutin::{GlContext, GlRequest};

// allow dead code until events are implemented
#[allow(dead_code)]
pub struct GLWindow {
    window: glutin::GlWindow,
    events: glutin::EventsLoop,
}

impl AbstractWindow for GLWindow {
    const FLOAT: u32 = gl::FLOAT;
    const COLOR_BUFFER_BIT: u32 = gl::COLOR_BUFFER_BIT;
    const VERTEX_SHADER: u32 = gl::VERTEX_SHADER;
    const FRAGMENT_SHADER: u32 = gl::FRAGMENT_SHADER;
    const ARRAY_BUFFER: u32 = gl::ARRAY_BUFFER;
    const STATIC_DRAW: u32 = gl::STATIC_DRAW;
    const DYNAMIC_DRAW: u32 = gl::DYNAMIC_DRAW;
    const COMPILE_STATUS: u32 = gl::COMPILE_STATUS;
    const POINTS: u32 = gl::POINTS;
    const LINE_STRIP: u32 = gl::LINE_STRIP;
    const LINE_LOOP: u32 = gl::LINE_LOOP;
    const LINES: u32 = gl::LINES;
    const TRIANGLE_STRIP: u32 = gl::TRIANGLE_STRIP;
    const TRIANGLE_FAN: u32 = gl::TRIANGLE_FAN;
    const TRIANGLES: u32 = gl::TRIANGLES;

    type GLEnum = u32;
    type GLsizeiptr = gl::types::GLsizeiptr;
    type GLintptr = gl::types::GLintptr;
    type GLBuffer = u32;
    type GLShader = u32;
    type GLProgram = u32;
    type GLVertexArray = u32;
    type GLUint = u32;

    fn new(title: &str, width: u32, height: u32) -> Self {
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(LogicalSize::new(width as f64, height as f64));
        let context = glutin::ContextBuilder::new()
            .with_gl(GlRequest::Specific(OpenGl, (3, 2)))
            .with_vsync(false);
        let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

        unsafe {
            gl_window.make_current().unwrap();
        }

        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

        GLWindow {
            window: gl_window,
            events: events_loop,
        }
    }

    fn run_loop(mut callback: impl FnMut(f64) -> bool + 'static) {
        while callback(0.0) {
            /*self.events.poll_events(|event| {
                match event {
                    glutin::Event::WindowEvent{ event, .. } => match event {
                        glutin::WindowEvent::CloseRequested => running = false,
                        _ => ()
                    },
                    _ => ()
                }
            });*/

            unsafe {
                gl::PointSize(4.0);
                gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                gl::Enable(gl::BLEND);
            }
        }
    }

    fn swap_buffers(&mut self) {
        self.window.swap_buffers().unwrap();
    }

    fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
    }

    fn clear(&self, mask: u32) {
        unsafe {
            gl::Clear(mask);
        }
    }

    fn create_shader(&self, type_: ShaderType) -> Option<Shader> {
        unsafe {
            match type_ {
                ShaderType::Vertex => Some(gl::CreateShader(Self::VERTEX_SHADER)),
                ShaderType::Fragment => Some(gl::CreateShader(Self::FRAGMENT_SHADER)),
            }
        }
    }

    fn shader_source(&self, shader: &Shader, source: &str) {
        unsafe {
            let src = CString::new(source).unwrap();
            gl::ShaderSource(*shader, 1, &src.as_ptr(), ptr::null());
        }
    }

    fn compile_shader(&self, shader: &Shader) {
        unsafe {
            gl::CompileShader(*shader);
        }
    }

    fn delete_shader(&self, shader: &Shader) {
        unsafe {
            gl::DeleteShader(*shader);
        }
    }

    fn get_shader_parameter(&self, shader: &Shader, pname: GLEnum) -> Option<i32> {
        let mut result = 0;
        unsafe {
            gl::GetShaderiv(*shader, pname, &mut result);
        }
        Some(result)
    }

    fn get_shader_info_log(&self, shader: &Shader) -> Option<String> {
        let info_length = self
            .get_shader_parameter(shader, gl::INFO_LOG_LENGTH)
            .unwrap();
        if (info_length > 0) {
            let mut written_length = 0;
            let buffer: String = iter::repeat(' ').take(info_length as usize).collect();

            let buffer_string = CString::new(buffer.as_bytes()).unwrap();
            unsafe {
                gl::GetShaderInfoLog(
                    *shader,
                    info_length,
                    &mut written_length,
                    buffer_string.as_ptr() as *mut i8,
                )
            };
            let bytes = buffer_string.as_bytes();
            let bytes = &bytes[..bytes.len() - 1];
            String::from_utf8(bytes.to_vec()).ok()
        } else {
            None
        }
    }

    fn create_program(&self) -> Option<Program> {
        unsafe { Some(gl::CreateProgram()) }
    }

    fn attach_shader(&self, program: &Program, shader: &Shader) {
        unsafe {
            gl::AttachShader(*program, *shader);
        }
    }

    fn link_program(&self, program: &Program) {
        unsafe {
            gl::LinkProgram(*program);
        }
    }

    fn use_program(&self, program: &Program) {
        unsafe {
            gl::UseProgram(*program);
        }
    }

    fn delete_program(&self, program: &Program) {
        unsafe {
            gl::DeleteProgram(*program);
        }
    }

    fn create_buffer(&self) -> Option<Buffer> {
        let mut buffer = 0;
        unsafe {
            gl::GenBuffers(1, &mut buffer);
        }
        Some(buffer)
    }

    fn bind_buffer(&self, target: GLEnum, buffer: &Buffer) {
        unsafe {
            gl::BindBuffer(target, *buffer);
        }
    }

    fn buffer_data(&self, target: GLEnum, data: &[f32], usage: GLEnum) {
        unsafe {
            gl::BufferData(
                target,
                (data.len() * mem::size_of::<f32>()) as GLsizeiptr,
                mem::transmute(&data[0]),
                usage,
            )
        }
    }

    fn delete_buffer(&self, buffer: &Buffer) {
        unsafe {
            gl::DeleteBuffers(1, buffer);
        }
    }

    fn create_vertex_array(&self) -> Option<VertexArray> {
        let mut vertex_array = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vertex_array);
        }
        Some(vertex_array)
    }

    fn bind_vertex_array(&self, vbo: &VertexArray) {
        unsafe {
            gl::BindVertexArray(*vbo);
        }
    }

    fn delete_vertex_array(&self, vbo: &VertexArray) {
        unsafe {
            gl::DeleteVertexArrays(1, vbo);
        }
    }

    fn get_attrib_location(&self, program: &Program, name: &str) -> GLUint {
        unsafe {
            let src = CString::new(name).unwrap();
            gl::GetAttribLocation(*program, src.as_ptr()) as GLUint
        }
    }

    fn vertex_attrib_pointer(
        &self,
        pointer: &GLUint,
        size: i32,
        type_: GLEnum,
        normalized: bool,
        stride: i32,
        offset: i32,
    ) {
        unsafe {
            gl::VertexAttribPointer(
                *pointer,
                size,
                type_,
                normalized as u8,
                stride * mem::size_of::<f32>() as gl::types::GLsizei,
                (offset * mem::size_of::<f32>() as i32) as *const () as *const _,
            ); // black magic
        }
    }

    fn enable_vertex_attrib_array(&self, pointer: &GLUint) {
        unsafe {
            gl::EnableVertexAttribArray(*pointer);
        }
    }

    fn draw_arrays(&self, type_: GLEnum, first: i32, count: i32) {
        unsafe {
            gl::DrawArrays(type_, first, count);
        }
    }
}
