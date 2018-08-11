/*
 * Very inspired from kiss3d's implementation of window and context
 * link: https://github.com/sebcrozet/kiss3d
 */
#![allow(unused_results)]

use window::abstract_window::*;

use stdweb::{Value};
use stdweb::web::{document, window, IParentNode, INode, TypedArray};
use stdweb::unstable::{TryInto};
use stdweb::web::html_element::{CanvasElement};

use std::mem;

use super::webgl_bindings::{
    WebGLBuffer, WebGLProgram, WebGLRenderingContext, WebGL2RenderingContext, WebGLShader,
    GLenum, GLintptr, GLsizeiptr, WebGLVertexArrayObject
};

pub struct WebGLWindow {
    context: WebGL2RenderingContext
}

impl AbstractWindow for WebGLWindow {
    
    const FLOAT: u32 = WebGLRenderingContext::FLOAT;
    const COLOR_BUFFER_BIT: u32 = WebGL2RenderingContext::COLOR_BUFFER_BIT;
    const VERTEX_SHADER: u32 = WebGL2RenderingContext::VERTEX_SHADER;
    const FRAGMENT_SHADER: u32 = WebGL2RenderingContext::FRAGMENT_SHADER; 
    const ARRAY_BUFFER:u32 = WebGL2RenderingContext::ARRAY_BUFFER;
    const STATIC_DRAW:u32 = WebGL2RenderingContext::STATIC_DRAW;
    const DYNAMIC_DRAW: u32 = WebGL2RenderingContext::STATIC_DRAW;
    const COMPILE_STATUS: u32 = WebGL2RenderingContext::COMPILE_STATUS;
    const POINTS: u32 = WebGL2RenderingContext::POINTS;
    const LINE_STRIP: u32 = WebGL2RenderingContext::LINE_STRIP;
    const LINE_LOOP: u32 = WebGL2RenderingContext::LINE_LOOP;
    const LINES: u32 = WebGL2RenderingContext::LINES;
    const TRIANGLE_STRIP: u32 = WebGL2RenderingContext::TRIANGLE_STRIP;
    const TRIANGLE_FAN: u32 = WebGL2RenderingContext::TRIANGLE_FAN;
    const TRIANGLES: u32 = WebGL2RenderingContext::TRIANGLES;
    
    type GLEnum = GLenum;
    type GLsizeiptr = GLsizeiptr;
    type GLintptr = GLintptr;
    type GLBuffer = WebGLBuffer;
    type GLShader = WebGLShader;
    type GLProgram = WebGLProgram;
    type GLVertexArray = WebGLVertexArrayObject;
    type GLUint = u32;
    
    fn new(_: &str, width: u32, height: u32) -> Self {
        let canvas: CanvasElement = document().create_element("canvas").unwrap().try_into().unwrap();
        canvas.set_width(width);
        canvas.set_height(height);
        document().query_selector("body").unwrap().unwrap().append_child(&canvas);
        
        //let res = js!(return @(canvas).getContext("2d", {alpha: true});).try_into();
        let context: WebGL2RenderingContext = canvas.get_context().unwrap();
        WebGLWindow {
            context
        }
    }

    fn run_loop(mut callback: impl FnMut(f64) -> bool + 'static) {
        let _ = window().request_animation_frame(move |t| {
            if(callback(t)) {
                let _ = Self::run_loop(callback);
            }
        });
    }

    fn swap_buffers(&mut self) {
        // No need to swap buffers on webgl
    }

    fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        self.context.clear_color(r, g, b, a);
    }

    fn clear(&self, mask: u32) {
        self.context.clear(mask);
    }
    fn create_shader(&self, type_: ShaderType) -> Option<Shader> {
        match type_ {
            ShaderType::Vertex => self.context.create_shader(Self::VERTEX_SHADER),
            ShaderType::Fragment => self.context.create_shader(Self::FRAGMENT_SHADER)
        }
    }

    fn shader_source(&self, shader: &Shader, source: &str) {
        self.context.shader_source(shader, source);
    }

    fn compile_shader(&self, shader: &Shader) {
        self.context.compile_shader(shader);
    }

    fn delete_shader(&self, shader: &Shader) {
        self.context.delete_shader(Some(shader));
    }

    fn get_shader_parameter(&self, shader: &Shader, pname: GLEnum) -> Option<i32> { // TODO: Handle all value types?
        match self.context.get_shader_parameter(shader, pname) {
            Value::Number(n) => n.try_into().ok(),
            _ => None,
        }
    }

    fn get_shader_info_log(&self, shader: &Shader) -> Option<String> {
        self.context.get_shader_info_log(shader)
    }

    fn create_program(&self) -> Option<Program> {
        self.context.create_program()
    }

    fn attach_shader(&self, program: &Program, shader: &Shader) {
        self.context.attach_shader(program, shader);
    } 

    fn link_program(&self, program: &Program) {
        self.context.link_program(program);
    }

    fn use_program(&self, program: &Program) {
        self.context.use_program(Some(program));
    }

    fn delete_program(&self, program: &Program) {
        self.context.delete_program(Some(program));
    }

    fn create_buffer(&self) -> Option<Buffer> {
        self.context.create_buffer()
    }

    fn bind_buffer(&self, target: GLEnum, buffer: &Buffer) {
        self.context.bind_buffer(target, Some(buffer));
    }

    fn buffer_data(&self, target: GLEnum, data: &[f32], usage: GLEnum) {
        let abuf = TypedArray::<f32>::from(data);
        self.context.buffer_data_1(target, Some(&abuf.buffer()), usage);
    }
    
    fn delete_buffer(&self, buffer: &Buffer) {
        self.context.delete_buffer(Some(buffer));
    }

    fn create_vertex_array(&self) -> Option<VertexArray> {
        self.context.create_vertex_array()
    }
    
    fn bind_vertex_array(&self, vbo: &VertexArray) {
        self.context.bind_vertex_array(Some(vbo));
    }

    fn delete_vertex_array(&self, vbo: &VertexArray) {
        self.context.delete_vertex_array(Some(vbo));
    }

    fn get_attrib_location(&self, program: &Program, name: &str) -> GLUint {
        self.context.get_attrib_location(program, name) as u32
    }

    fn vertex_attrib_pointer(&self, pointer: &GLUint, size: i32, type_: GLEnum, normalized: bool, stride: i32, offset: i32) {
        self.context.vertex_attrib_pointer(*pointer, size, type_, normalized, 
        (stride * mem::size_of::<f32>() as i32) as i32, 
        (offset * mem::size_of::<f32>() as i32) as GLintptr) // todo: offset as custom type
    }

    fn enable_vertex_attrib_array(&self, pointer: &GLUint) {
        self.context.enable_vertex_attrib_array(*pointer)
    }

    fn draw_arrays(&self, type_: GLEnum, first: i32, count: i32) {
        self.context.enable(WebGL2RenderingContext::BLEND);
        self.context.blend_func(WebGL2RenderingContext::SRC_ALPHA, WebGL2RenderingContext::ONE_MINUS_SRC_ALPHA);
        self.context.draw_arrays(type_, first, count)
    }

}