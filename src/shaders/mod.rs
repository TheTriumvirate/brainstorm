//! Contains the shaders for rendering.

use std::{
    str,
    ops::Drop,
};

// TODO: proper imports
use context::*;

/// Vertex shader
pub const VERTEX_SHADER: &[u8] = include_bytes!("vertex.glslv");

/// Fragment shader
pub const FRAGMENT_SHADER: &[u8] = include_bytes!("fragment.glslf");

pub enum ShaderType {
    Vertex,
    Fragment,
}

pub struct OurShader {
    pub program: Program,
    vs: Shader,
    fs: Shader,
}

impl OurShader {
    pub fn new(
        vertex_shader: &str,
        fragment_shader: &str) -> Self {
        
        let context = Context::get_context();
        // Compile vertex shader
        let vs = {
            let vs = context
                .create_shader(ShaderType::Vertex)
                .expect("Failed to create vertex shader.");
            context.shader_source(&vs, vertex_shader);
            context.compile_shader(&vs);

            if let Some(log) = context.get_shader_info_log(&vs) {
                println!("vertex shader log: {}", log);
            }

            vs
        };

        // Compile fragment shader
        let fs = {
            let fs = context
                .create_shader(ShaderType::Fragment)
                .expect("Failed to create fragment shader.");
            context.shader_source(&fs, fragment_shader);
            context.compile_shader(&fs);

            if let Some(log) = context.get_shader_info_log(&fs) {
                println!("fragment shader log: {}", log);
            }

            fs
        };

        // Link program
        let program = {
            let program = context
                .create_program()
                .expect("Failed to create shader program.");
            context.attach_shader(&program, &vs);
            context.attach_shader(&program, &fs);
            context.link_program(&program);
            context.use_program(&program);

            program
        };

        OurShader {
            program,
            fs,
            vs,
        }
    }
}

impl Drop for OurShader {
    fn drop(&mut self) {
        let context = Context::get_context();
        context.delete_program(&self.program);
        context.delete_shader(&self.vs);
        context.delete_shader(&self.fs);
    }
}
