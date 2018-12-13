
#[cfg(not(target_arch = "wasm32"))]
use crate::opengl as ContextImpl;
#[cfg(target_arch = "wasm32")]
use crate::webgl as ContextImpl;

use std::slice;
use na::{Matrix4};

use crate::shaders::ShaderType;
use crate::Context;

pub enum GlPrimitiveArray<'a> {
    F32(&'a [f32]),
    U16(&'a [u16]),
    U32(&'a [u32]),
}

pub trait GlPrimitive: Copy {
    fn into(data: &[Self]) -> GlPrimitiveArray;
    fn primitive() -> u32;
}

// TODO: Auto-destruct Buffer, etc
pub type UniformLocation = ContextImpl::UniformLocation;
pub type GLEnum = ContextImpl::GLEnum;
pub type GLintptr = ContextImpl::GLintptr;
pub type Buffer = ContextImpl::GLBuffer;
pub type GLUint = ContextImpl::GLUint;
pub type Shader = ContextImpl::GLShader;
pub type Program = ContextImpl::GLProgram;
pub type Texture = ContextImpl::GLTexture;
pub type FrameBuffer = ContextImpl::GLFrameBuffer;
pub type VertexArray = ContextImpl::GLVertexArray;

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
    const UNSIGNED_INT: u32;
    const TEXTURE_2D: u32;
    const TEXTURE_3D: u32;
    const TEXTURE_2D_ARRAY: u32;
    const UNSIGNED_BYTE: u32;
    const RGBA: u32;
    const RGBA32F: u32;
    const RGBA8: u32;
    const LUMINANCE: u32;
    const TEXTURE0: u32;
    const TEXTURE_WRAP_S: u32;
    const TEXTURE_WRAP_T: u32;
    const CLAMP_TO_EDGE: u32;
    const TEXTURE_MIN_FILTER: u32;
    const TEXTURE_MAG_FILTER: u32;
    const NEAREST: u32;
    const LINEAR: u32;
    const LINEAR_MIPMAP_LINEAR: u32;
    const UNPACK_ALIGNMENT: u32;
    const DEPTH_BUFFER_BIT: u32;
    const FRONT_AND_BACK: u32;
    const DEPTH_TEST: u32;
    const FRAMEBUFFER: u32;
    const COLOR_ATTACHMENT0: u32;
    const RASTERIZER_DISCARD: u32;
    const TRANSFORM_FEEDBACK_BUFFER: u32;
    const INTERLEAVED_ATTRIBS: u32;
    const STATIC_READ: u32;
    const LINK_STATUS: u32;
    const ONE_MINUS_SRC_ALPHA: u32;
    const SRC_ALPHA: u32;
    const ONE: u32;

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
    fn get_program_info_log(&self, program: &Program) -> Option<String>;
    fn transform_feedback_varyings(&self, program: &Program, varyings: &str, buffer_mode: GLEnum);
    fn get_program_parameter(&self, program: &Program, pname: GLEnum) -> Option<i32>;

    fn create_buffer(&self) -> Option<Buffer>;
    fn bind_buffer(&self, target: GLEnum, buffer: &Buffer);
    fn buffer_data<T: GlPrimitive>(&self, target: GLEnum, data: Option<&[T]>, usage: GLEnum);
    fn delete_buffer(&self, buffer: &Buffer);

    fn create_vertexbuffer(&self) -> Option<VertexArray>;
    fn bind_vertexbuffer(&self, vertex_array: Option<&VertexArray>);
    fn delete_vertexbuffer(&self, vertex_array: &VertexArray);

    fn create_framebuffer(&self) -> Option<FrameBuffer>;
    fn bind_framebuffer(&self, target: GLEnum, framebuffer: Option<&FrameBuffer>);
    fn delete_framebuffer(&self, framebuffer: &FrameBuffer);
    fn framebuffer_texture2d(&self, target: GLEnum, attachment: GLEnum, textarget: GLEnum, texture: &Texture, level: i32);
    fn framebuffer_texture_layer(&self, target: GLEnum, attachment: GLEnum, texture: &Texture, level: i32, layer: i32);

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
    fn uniform1f(&self, location: &UniformLocation, x: f32);
    fn uniform2f(&self, location: &UniformLocation, x: f32, y: f32);
    fn uniform3f(&self, location: &UniformLocation, x: f32, y: f32, z: f32);

    fn create_texture(&self) -> Option<Texture>;
    fn bind_texture(&self, target: GLEnum, texture: &Texture);
    fn unbind_texture(&self, target: GLEnum);
    fn tex_parameteri(&self, target: GLEnum, pname: GLEnum, param: i32);
    fn tex_image2d(&self, target: GLEnum, level: i32, internalformat: i32, width: i32, height: i32, border: i32, format: GLEnum, pixels: Option<&[u8]>);
    fn tex_sub_image2d(&self, target: GLEnum, level: i32, xoffset: i32, yoffset: i32, width: i32, height: i32, format: GLEnum, pixels: Option<&[u8]>);
    fn tex_image2d_f(&self, target: GLEnum, level: i32, internalformat: i32, width: i32, height: i32, border: i32, format: GLEnum, pixels: Option<&[f32]>);
    fn tex_image3d(&self, target: GLEnum, level: i32, internalformat: i32, width: i32, height: i32, depth: i32, border: i32, format: GLEnum, pixels: Option<&[u8]>);
    fn tex_image3d_f(&self, target: GLEnum, level: i32, internalformat: i32, width: i32, height: i32, depth: i32, border: i32, format: GLEnum, pixels: Option<&[f32]>);
    fn delete_texture(&self, texture: &Texture);
    fn active_texture(&self, _type: GLEnum);
    fn generate_mipmap(&self, target: GLEnum);

    fn draw_arrays(&self, type_: GLEnum, first: i32, count: i32);
    fn draw_elements(&self, mode: GLEnum, count: i32, type_: GLEnum, offset: GLintptr);
    fn flush(&self);

    fn viewport(&self, x: i32, y: i32, width: i32, height: i32);

    fn pixel_storei(&self, pname: GLEnum, param: i32);

    fn enable(&self, cap: GLEnum);    
    fn disable(&self, cap: GLEnum);    

    fn depth_mask(&self, flag: bool);

    fn bind_buffer_base(&self, target: GLEnum, index: u32, buffer: Option<&Buffer>);
    fn get_buffer_sub_data(&self, target: GLEnum, index: u32, data: &mut [f32]);

    fn begin_transform_feedback(&self, type_: GLEnum);
    fn end_transform_feedback(&self);

    fn blend_func(&self, s_factor: GLEnum, d_factor: GLEnum);
}

impl GlPrimitive for f32 {
    fn into(data: &[f32]) -> GlPrimitiveArray {
        unsafe {
            let len = data.len();
            let ptr = data.as_ptr();

            GlPrimitiveArray::F32(slice::from_raw_parts(ptr as *const f32, len))
        }
    }

    fn primitive() -> u32 {
        Context::FLOAT
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

    fn primitive() -> u32 {
        Context::UNSIGNED_SHORT
    }
}

impl GlPrimitive for u32 {
    fn into(data: &[u32]) -> GlPrimitiveArray {
        unsafe {
            let len = data.len();
            let ptr = data.as_ptr();

            GlPrimitiveArray::U32(slice::from_raw_parts(ptr as *const u32, len))
        }
    }

    fn primitive() -> u32 {
        Context::UNSIGNED_INT
    }
}