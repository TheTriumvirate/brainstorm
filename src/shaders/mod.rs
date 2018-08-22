//! Contains the shaders for rendering.

use std::{
    str,
    ops::Drop,
    rc::Rc,
    cell::RefCell,
};
use window::{self, Window, AbstractWindow};


/// Vertex shader
pub const VERTEX_SHADER: &[u8] = include_bytes!("vertex.glslv");

/// Fragment shader
pub const FRAGMENT_SHADER: &[u8] = include_bytes!("fragment.glslf");

pub enum ShaderType {
    Vertex,
    Fragment,
}

pub struct OurShader {
    context: Rc<RefCell<Window>>,
    pub program: window::Program,
    vs: window::Shader,
    fs: window::Shader,
}

impl OurShader {
    pub fn new(
        context: Rc<RefCell<Window>>,
        vertex_shader: &str,
        fragment_shader: &str) -> Self {
        
        // Compile vertex shader
        let vs = {
            let shader_mut = context.borrow_mut();
            let vs = shader_mut
                .create_shader(ShaderType::Vertex)
                .expect("Failed to create vertex shader.");
            shader_mut.shader_source(&vs, vertex_shader);
            shader_mut.compile_shader(&vs);

            if let Some(log) = shader_mut.get_shader_info_log(&vs) {
                println!("vertex shader log: {}", log);
            }

            vs
        };

        // Compile fragment shader
        let fs = {
            let shader_mut = context.borrow_mut();

            let fs = shader_mut
                .create_shader(ShaderType::Fragment)
                .expect("Failed to create fragment shader.");
            shader_mut.shader_source(&fs, fragment_shader);
            shader_mut.compile_shader(&fs);

            if let Some(log) = shader_mut.get_shader_info_log(&fs) {
                println!("fragment shader log: {}", log);
            }

            fs
        };

        // Link program
        let program = {
            let shader_mut = context.borrow_mut();

            let program = shader_mut
                .create_program()
                .expect("Failed to create shader program.");
            shader_mut.attach_shader(&program, &vs);
            shader_mut.attach_shader(&program, &fs);
            shader_mut.link_program(&program);
            shader_mut.use_program(&program);

            program
        };

        OurShader {
            context,
            program,
            fs,
            vs,
        }
    }
}

impl Drop for OurShader {
    fn drop(&mut self) {
        let context = self.context.borrow_mut();
        context.delete_program(&self.program);
        context.delete_shader(&self.vs);
        context.delete_shader(&self.fs);
    }
}
