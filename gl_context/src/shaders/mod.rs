//! Contains the shaders for rendering.

use std::{
    str,
    ops::Drop,
};

use Shader;
use Program;
use AbstractContext;
use Context;
use context::{UniformLocation, GLUint};

/// Vertex shader for particles
pub const PARTICLES_VERTEX_SHADER: &[u8] = include_bytes!("particles_vertex.glslv");

/// Fragment shader for particles
pub const PARTICLES_FRAGMENT_SHADER: &[u8] = include_bytes!("particles_fragment.glslf");

/// Vertex shader for triangles
pub const TRIANGLES_VERTEX_SHADER: &[u8] = include_bytes!("triangles_vertex.glslv");

/// Fragment shader for triangles
pub const TRIANGLES_FRAGMENT_SHADER: &[u8] = include_bytes!("triangles_fragment.glslf");

pub enum ShaderType {
    Vertex,
    Fragment,
}

pub struct OurShader {
    pub program: Program,
    vs: Shader,
    fs: Shader,
    pos_attrib: GLUint,
    color_attrib: Option<GLUint>,
    shader_dimensions: i32,
}

impl OurShader {
    /// PREREQ: shader dimension isn't wrong.
    pub fn new(
        vertex_shader: &str,
        fragment_shader: &str,
        shader_dimensions: i32,
        has_color_attrib: bool) -> Self {

        let context = Context::get_context();
        // Compile vertex shader
        let vs = context
            .create_shader(ShaderType::Vertex)
            .expect("Failed to create vertex shader.");
        context.shader_source(&vs, vertex_shader);
        context.compile_shader(&vs);

        if let Some(log) = context.get_shader_info_log(&vs) {
            println!("vertex shader log: {}", log);
        }

        // Compile fragment shader
        let fs = context
            .create_shader(ShaderType::Fragment)
            .expect("Failed to create fragment shader.");
        context.shader_source(&fs, fragment_shader);
        context.compile_shader(&fs);

        if let Some(log) = context.get_shader_info_log(&fs) {
            println!("fragment shader log: {}", log);
        }

        // Link program
        let program = context
            .create_program()
            .expect("Failed to create shader program.");
        context.attach_shader(&program, &vs);
        context.attach_shader(&program, &fs);
        context.link_program(&program);

        // Enable the attribute arrays.
        context.use_program(&program);
        let pos_attrib = context.get_attrib_location(&program, "position");

        // And for colors
        let color_attrib = if has_color_attrib {
            Some(context.get_attrib_location(&program, "color"))
        } else {
            None
        };

        OurShader {
            program,
            fs,
            vs,
            pos_attrib,
            color_attrib,
            shader_dimensions,
        }
    }

    pub fn set_active(&self) {
        let context = Context::get_context();
        context.use_program(&self.program);

        let total_size = match self.color_attrib.is_some() {
            true => self.shader_dimensions + 3,
            false => self.shader_dimensions,
        };

        context.vertex_attrib_pointer(&self.pos_attrib, self.shader_dimensions, Context::FLOAT, false, total_size, 0);
        context.enable_vertex_attrib_array(&self.pos_attrib);
        
        if let Some(ref color_attrib) = self.color_attrib {
            context.vertex_attrib_pointer(color_attrib, 3, Context::FLOAT, false, total_size, self.shader_dimensions);
            context.enable_vertex_attrib_array(color_attrib);
        }
    }

    pub fn get_uniform_location(&self) -> UniformLocation {
        Context::get_context().get_uniform_location(&self.program, "MVP")
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
