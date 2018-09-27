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

const DEFAULT_VERTEX_SHADER: &[u8] = include_bytes!("default.vert");
const DEFAULT_FRAGMENT_SHADER: &[u8] = include_bytes!("default.frag");

lazy_static! {
    static ref DEFAULT: OurShader = OurShader::new(
        str::from_utf8(DEFAULT_VERTEX_SHADER).expect("Failed to read vertex shader"), 
        str::from_utf8(DEFAULT_FRAGMENT_SHADER).expect("Failed to read fragment shader"), 
        &[
            ShaderAttribute{name: "a_position".to_string(), size: 2},
            ShaderAttribute{name: "a_color".to_string(), size: 3},
            ShaderAttribute{name: "a_texture".to_string(), size: 2},
        ]
    );
}

pub enum ShaderType {
    Vertex,
    Fragment,
}

#[derive(Clone)]
pub struct ShaderAttribute {
    pub name: String,
    pub size: usize,
}

pub struct OurShader {
    pub program: Program,
    vs: Shader,
    fs: Shader,
    attributes: Vec<ShaderAttribute>,
    attribute_locations: Vec<GLUint>,
    attribute_size: usize,
}

impl OurShader {
    pub fn new(
        vertex_shader: &str,
        fragment_shader: &str,
        attributes: &[ShaderAttribute]) -> Self {

        let context = Context::get_context();

        // Compile vertex shader
        let vs = context
            .create_shader(ShaderType::Vertex)
            .expect("Failed to create vertex shader.");
        context.shader_source(&vs, vertex_shader);
        context.compile_shader(&vs);

        let compiles = context.get_shader_parameter(&vs, Context::COMPILE_STATUS);
        if compiles == Some(0) {
            if let Some(log) = context.get_shader_info_log(&vs) {
                println!("vertex shader log: {}", log);
            } else {
                println!("Some error occured while compiling vertex shader");
            }
        }
        // Compile fragment shader
        let fs = context
            .create_shader(ShaderType::Fragment)
            .expect("Failed to create fragment shader.");
        context.shader_source(&fs, fragment_shader);
        context.compile_shader(&fs);

        let compiles = context.get_shader_parameter(&fs, Context::COMPILE_STATUS);
        if compiles == Some(0) {
            if let Some(log) = context.get_shader_info_log(&fs) {
                println!("fragment shader log: {}", log);
            } else {
                println!("Some error occured while compiling fragment shader");
            }
        }

        // Link program
        let program = context
            .create_program()
            .expect("Failed to create shader program.");
        context.attach_shader(&program, &vs);
        context.attach_shader(&program, &fs);
        context.link_program(&program);

        let mut attribute_locations : Vec<GLUint> = Vec::new();
        context.use_program(&program);
        let mut attribute_size = 0;
        for (index, attrib) in attributes.iter().enumerate() {
            context.bind_attrib_location(&program, index as u32, &attrib.name);
            //let attrib_loc : i32 = context.get_attrib_location(&program, &attrib.name) as i32;
            let attrib_loc: i32 = index as i32;
            attribute_locations.push(attrib_loc as GLUint);
            attribute_size += attrib.size;
        }

        OurShader {
            program,
            fs,
            vs,
            attributes: attributes.to_vec(),
            attribute_locations,
            attribute_size,
        }
    }

    pub fn default() -> &'static Self {
        &DEFAULT
    }

    pub fn use_program(&self) {
        let context = Context::get_context();
        context.use_program(&self.program);

    }

    pub fn bind_attribs(&self) {
        let context = Context::get_context();

        let mut offset : usize = 0;
        for i in 0..self.attributes.len() {
            let attrib = &self.attributes[i];
            let attrib_pos = self.attribute_locations[i];
            let off = offset as i32;
            context.vertex_attrib_pointer(&attrib_pos, attrib.size as i32, Context::FLOAT, false, self.attribute_size as i32, off);
            context.enable_vertex_attrib_array(&attrib_pos);
            offset += attrib.size;
        }
    }
    
    pub fn unbind_attribs(&self) {
        let context = Context::get_context();

        let mut offset : usize = 0;
        for i in 0..self.attributes.len() {
            let attrib = &self.attributes[i];
            let attrib_pos = self.attribute_locations[i];
            let off = offset as i32;
            context.vertex_attrib_pointer(&attrib_pos, attrib.size as i32, Context::FLOAT, false, self.attribute_size as i32, off);
            context.disable_vertex_attrib_array(&attrib_pos);
            offset += attrib.size;
        }
    }

    pub fn uniform1i(&self, name: &str, value: i32) {
        let location = Context::get_context().get_uniform_location(&self.program, name);
        Context::get_context().uniform1i(&location, value);
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
