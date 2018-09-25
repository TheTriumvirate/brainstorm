
#[cfg(not(target_arch = "wasm32"))]
use opengl as ContextImpl;
#[cfg(target_arch = "wasm32")]
use webgl as ContextImpl;

use std::slice;

use shaders::ShaderType;
use na::{Matrix4};
use Context;

pub enum GlPrimitiveArray<'a> {
    F32(&'a [f32]),
    U16(&'a [u16]),
}

pub trait GlPrimitive: Copy {
    fn into(data: &[Self]) -> GlPrimitiveArray;
}

// TODO: Auto-destruct Buffer, etc
pub type UniformLocation = ContextImpl::UniformLocation;
pub type GLEnum = ContextImpl::GLEnum;
pub type GLsizeiptr = ContextImpl::GLsizeiptr;
pub type GLintptr = ContextImpl::GLintptr;
pub type Buffer = ContextImpl::GLBuffer;
pub type GLUint = ContextImpl::GLUint;
pub type Shader = ContextImpl::GLShader;
pub type Program = ContextImpl::GLProgram;
pub type Texture = ContextImpl::GLTexture;

/// Represents the common interface of OpenGL and WebGL.
/// Check the OpenGL and WebGL documentation for details.
pub trait AbstractContext {
    const VERTEX_SHADER: u32;
    const FRAGMENT_SHADER: u32;
    const FLOAT: u32;
    const COLOR_BUFFER_BIT: u32;
    const ARRAY_BUFFER: u32;
    const ELEMENT_ARRAY_BUFFER: u32;
    const STATIC_DRAW: u32;
    const DYNAMIC_DRAW: u32;
    const COMPILE_STATUS: u32;
    const POINTS: u32;
    const LINE_STRIP: u32;
    const LINE_LOOP: u32;
    const LINES: u32;
    const TRIANGLE_STRIP: u32;
    const TRIANGLE_FAN: u32;
    const TRIANGLES: u32;
    const UNSIGNED_SHORT: u32;
    const TEXTURE_2D: u32;
    const UNSIGNED_BYTE: u32;
    const RGBA: u32;
    const TEXTURE0: u32;

    fn get_context() -> &'static Context;
    
    fn clear_color(&self, r: f32, g: f32, b: f32, a: f32);
    fn clear(&self, mask: u32);

    fn create_shader(&self, type_: ShaderType) -> Option<Shader>;
    fn shader_source(&self, shader: &Shader, source: &str);
    fn compile_shader(&self, shader: &Shader);
    fn delete_shader(&self, shader: &Shader);
    fn get_shader_parameter(&self, shader: &Shader, pname: GLEnum) -> Option<i32>;
    fn get_shader_info_log(&self, shader: &Shader) -> Option<String>;

    fn create_program(&self) -> Option<Program>;
    fn attach_shader(&self, program: &Program, shader: &Shader);
    fn link_program(&self, program: &Program);
    fn use_program(&self, program: &Program);
    fn delete_program(&self, program: &Program);

    fn create_buffer(&self) -> Option<Buffer>;
    fn bind_buffer(&self, target: GLEnum, buffer: &Buffer);
    fn buffer_data<T: GlPrimitive>(&self, target: GLEnum, data: &[T], usage: GLEnum);
    fn delete_buffer(&self, buffer: &Buffer);

    fn get_attrib_location(&self, program: &Program, name: &str) -> GLUint;
    fn vertex_attrib_pointer(
        &self,
        pointer: &GLUint,
        size: i32,
        type_: GLEnum,
        normalized: bool,
        stride: i32,
        offset: i32,
    );
    fn enable_vertex_attrib_array(&self, pointer: &GLUint);
    fn disable_vertex_attrib_array(&self, pointer: &GLUint);
    fn bind_attrib_location(&self, program: &Program, index: GLUint, name: &str);
    
    fn get_uniform_location(&self, program: &Program, name: &str) -> UniformLocation;
    fn uniform_matrix_4fv(&self, location: &UniformLocation, size: i32, transpose: bool, matrix: &Matrix4<f32>);
    fn uniform1i(&self, location: &UniformLocation, x: i32);

    fn create_texture(&self) -> Option<Texture>;
    fn bind_texture(&self, target: GLEnum, texture: &Texture);
    fn tex_image2d(&self, target: GLEnum, level: i32, internalformat: i32, width: i32, height: i32, border: i32, format: GLEnum, pixels: &[u8]);
    fn delete_texture(&self, texture: &Texture);
    fn active_texture(&self, _type: GLEnum);
    fn generate_mipmap(&self, target: GLEnum);

    fn draw_arrays(&self, type_: GLEnum, first: i32, count: i32);
    fn draw_elements(&self, mode: GLEnum, count: i32, type_: GLEnum, offset: GLintptr);

    fn viewport(&self, x: i32, y: i32, width: i32, height: i32);
}

impl GlPrimitive for f32 {
    fn into(data: &[f32]) -> GlPrimitiveArray {
        unsafe {
            let len = data.len();
            let ptr = data.as_ptr();

            GlPrimitiveArray::F32(slice::from_raw_parts(ptr as *const f32, len))
        }
    }
}

impl GlPrimitive for u16 {
    fn into(data: &[u16]) -> GlPrimitiveArray {
        unsafe {
            let len = data.len();
            let ptr = data.as_ptr();

            GlPrimitiveArray::U16(slice::from_raw_parts(ptr as *const u16, len))
        }
    }
}