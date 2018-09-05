
#[cfg(not(target_arch = "wasm32"))]
use opengl as ContextImpl;
#[cfg(target_arch = "wasm32")]
use webgl as ContextImpl;

use shaders::ShaderType;
use na::{Matrix4};
use Context;

// TODO: Auto-destruct Buffer, etc
pub type UniformLocation = ContextImpl::UniformLocation;
pub type GLEnum = ContextImpl::GLEnum;
pub type GLsizeiptr = ContextImpl::GLsizeiptr;
pub type GLintptr = ContextImpl::GLintptr;
pub type Buffer = ContextImpl::GLBuffer;
pub type VertexArray = ContextImpl::GLVertexArray;
pub type GLUint = ContextImpl::GLUint;
pub type Shader = ContextImpl::GLShader;
pub type Program = ContextImpl::GLProgram;

pub trait AbstractContext {
    const VERTEX_SHADER: u32;
    const FRAGMENT_SHADER: u32;
    const FLOAT: u32;
    const COLOR_BUFFER_BIT: u32;
    const ARRAY_BUFFER: u32;
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
    fn buffer_data(&self, target: GLEnum, data: &[f32], usage: GLEnum);
    fn delete_buffer(&self, buffer: &Buffer);

    fn create_vertex_array(&self) -> Option<VertexArray>;
    fn bind_vertex_array(&self, vbo: &VertexArray);
    fn delete_vertex_array(&self, vbo: &VertexArray);

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
    fn bind_attrib_location(&self, program: &Program, index: GLUint, name: &str);
    
    fn get_uniform_location(&self, program: &Program, name: &str) -> UniformLocation;
    fn uniform_matrix_4fv(&self, location: &UniformLocation, size: i32, transpose: bool, matrix: &Matrix4<f32>);

    fn draw_arrays(&self, type_: GLEnum, first: i32, count: i32);
}