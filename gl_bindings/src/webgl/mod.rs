/*
 * Very inspired from kiss3d's implementation of window and context
 * link: https://github.com/sebcrozet/kiss3d
 */
#![allow(unused_results)]

// Ignore warnings in autogenerated bindings
#[allow(unused_parens)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(non_camel_case_types)]
#[cfg(target_arch = "wasm32")]
mod webgl2_bindings;

use shaders::ShaderType;

use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, IParentNode, TypedArray};
use stdweb::Value;

use std::{mem};

use na::{Matrix4};

use context::{GlPrimitive, GlPrimitiveArray};

use Program;
use Shader;
use AbstractContext;
use NativeBuffer;
use Context;
use NativeTexture;

use self::webgl2_bindings::{
    WebGL2RenderingContext, WebGLBuffer, WebGLProgram,
    WebGLShader, WebGLUniformLocation, WebGLTexture, WebGLFramebuffer, WebGLVertexArrayObject
};

pub use self::webgl2_bindings::{
    GLenum, GLintptr, GLsizeiptr
};

pub type GLShader = WebGLShader;
pub type GLProgram = WebGLProgram;
pub type UniformLocation = WebGLUniformLocation;
pub type GLEnum = GLenum;
pub type GLBuffer = WebGLBuffer;
pub type GLUint = u32;
pub type GLTexture = WebGLTexture;
pub type GLFrameBuffer = WebGLFramebuffer;
pub type GLVertexArray = WebGLVertexArrayObject;

lazy_static! {
    static ref CONTEXT: Context = WebGLContext::new();
}

pub struct WebGLContext {
    context: WebGL2RenderingContext
}

impl WebGLContext {
    fn new() -> Self {
        let canvas: CanvasElement = document()
            .query_selector("#canvas")
            .expect("No canvas found")
            .unwrap()
            .try_into()
            .unwrap();

        let context = js!(
            var gl = @{canvas}.getContext("webgl2", {alpha: false});
            console.log("NO_EXT? ", gl.getExtension("EXT_color_buffer_float"));
            return gl;
            ).try_into().unwrap();
        
        WebGLContext { context }
    }
}

impl AbstractContext for WebGLContext {
    const FLOAT: u32 = WebGL2RenderingContext::FLOAT;
    const COLOR_BUFFER_BIT: u32 = WebGL2RenderingContext::COLOR_BUFFER_BIT;
    const VERTEX_SHADER: u32 = WebGL2RenderingContext::VERTEX_SHADER;
    const FRAGMENT_SHADER: u32 = WebGL2RenderingContext::FRAGMENT_SHADER;
    const ARRAY_BUFFER: u32 = WebGL2RenderingContext::ARRAY_BUFFER;
    const ELEMENT_ARRAY_BUFFER: u32 = WebGL2RenderingContext::ELEMENT_ARRAY_BUFFER;
    const STATIC_DRAW: u32 = WebGL2RenderingContext::STATIC_DRAW;
    const DYNAMIC_DRAW: u32 = WebGL2RenderingContext::STATIC_DRAW;
    const COMPILE_STATUS: u32 = WebGL2RenderingContext::COMPILE_STATUS;
    const POINTS: u32 = WebGL2RenderingContext::POINTS;
    const LINE_STRIP: u32 = WebGL2RenderingContext::LINE_STRIP;
    const LINE_LOOP: u32 = WebGL2RenderingContext::LINE_LOOP;
    const LINES: u32 = WebGL2RenderingContext::LINES;
    const TRIANGLE_STRIP: u32 = WebGL2RenderingContext::TRIANGLE_STRIP;
    const TRIANGLE_FAN: u32 = WebGL2RenderingContext::TRIANGLE_FAN;
    const TRIANGLES: u32 = WebGL2RenderingContext::TRIANGLES;
    const UNSIGNED_SHORT: u32 = WebGL2RenderingContext::UNSIGNED_SHORT;
    const TEXTURE_2D: u32 = WebGL2RenderingContext::TEXTURE_2D;
    const TEXTURE_3D: u32 = WebGL2RenderingContext::TEXTURE_3D;
    const TEXTURE_2D_ARRAY: u32 = WebGL2RenderingContext::TEXTURE_2D_ARRAY;
    const UNSIGNED_BYTE: u32 = WebGL2RenderingContext::UNSIGNED_BYTE;
    const RGBA: u32 = WebGL2RenderingContext::RGBA;
    const RGBA32F: u32 = WebGL2RenderingContext::RGBA32F;
    const RGBA8: u32 = WebGL2RenderingContext::RGBA8;
    const LUMINANCE: u32 = WebGL2RenderingContext::LUMINANCE;
    const TEXTURE0: u32 = WebGL2RenderingContext::TEXTURE0;
    const TEXTURE_WRAP_S: u32 = WebGL2RenderingContext::TEXTURE_WRAP_S;
    const TEXTURE_WRAP_T: u32 = WebGL2RenderingContext::TEXTURE_WRAP_T;
    const CLAMP_TO_EDGE: u32 = WebGL2RenderingContext::CLAMP_TO_EDGE;
    const TEXTURE_MIN_FILTER: u32 = WebGL2RenderingContext::TEXTURE_MIN_FILTER;
    const TEXTURE_MAG_FILTER: u32 = WebGL2RenderingContext::TEXTURE_MAG_FILTER;
    const NEAREST: u32 = WebGL2RenderingContext::NEAREST;
    const LINEAR: u32 = WebGL2RenderingContext::LINEAR;
    const LINEAR_MIPMAP_LINEAR: u32 = WebGL2RenderingContext::LINEAR_MIPMAP_LINEAR;
    const UNPACK_ALIGNMENT: u32 = WebGL2RenderingContext::UNPACK_ALIGNMENT;
    const DEPTH_BUFFER_BIT: u32 = WebGL2RenderingContext::DEPTH_BUFFER_BIT;
    const FRONT_AND_BACK: u32 = WebGL2RenderingContext::FRONT_AND_BACK;
    const DEPTH_TEST: u32 = WebGL2RenderingContext::DEPTH_TEST;
    const UNSIGNED_INT: u32 = WebGL2RenderingContext::UNSIGNED_INT;
    const FRAMEBUFFER: u32 = WebGL2RenderingContext::FRAMEBUFFER;
    const COLOR_ATTACHMENT0: u32 = WebGL2RenderingContext::COLOR_ATTACHMENT0;
    const RASTERIZER_DISCARD: u32 = WebGL2RenderingContext::RASTERIZER_DISCARD;
    const TRANSFORM_FEEDBACK_BUFFER: u32 = WebGL2RenderingContext::TRANSFORM_FEEDBACK_BUFFER;
    const INTERLEAVED_ATTRIBS: u32 = WebGL2RenderingContext::INTERLEAVED_ATTRIBS;
    const STATIC_READ: u32 = WebGL2RenderingContext::STATIC_READ;
    const LINK_STATUS: u32 = WebGL2RenderingContext::LINK_STATUS;

    fn get_context() -> &'static Context {
        &CONTEXT
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
            ShaderType::Fragment => self.context.create_shader(Self::FRAGMENT_SHADER),
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

    fn get_shader_parameter(&self, shader: &Shader, pname: GLEnum) -> Option<i32> {
        // TODO: Handle all value types?
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
    
    fn get_program_info_log(&self, program: &Program) -> Option<String> {
        self.context.get_program_info_log(program)
    }
    
    fn transform_feedback_varyings(&self, program: &Program, varyings: &str, buffer_mode: GLEnum) {
        self.context.transform_feedback_varyings(program, &[varyings], buffer_mode);
    }

    fn get_program_parameter(&self, program: &Program, pname: GLEnum) -> Option<i32> {
        match self.context.get_program_parameter(program, pname) {
            Value::Number(n) => n.try_into().ok(),
            _ => None,
        }
    }

    fn create_buffer(&self) -> Option<NativeBuffer> {
        self.context.create_buffer()
    }

    fn bind_buffer(&self, target: GLEnum, buffer: &NativeBuffer) {
        self.context.bind_buffer(target, Some(buffer));
    }

    fn buffer_data<T: GlPrimitive>(&self, target: GLEnum, data: Option<&[T]>, usage: GLEnum) {
        let data = data.unwrap_or(&[]);
        match T::into(data) {
            GlPrimitiveArray::F32(data) => {
                let abuf = TypedArray::<f32>::from(data);
                self.context
                    .buffer_data_1(target, Some(&abuf.buffer()), usage);
            },
            GlPrimitiveArray::U16(data) => {
                let abuf = TypedArray::<u16>::from(data);
                self.context
                    .buffer_data_1(target, Some(&abuf.buffer()), usage);
            },
            GlPrimitiveArray::U32(data) => {
                let abuf = TypedArray::<u32>::from(data);
                self.context
                    .buffer_data_1(target, Some(&abuf.buffer()), usage);
            }
        }
    }

    fn delete_buffer(&self, buffer: &NativeBuffer) {
        self.context.delete_buffer(Some(buffer));
    }

    fn create_vertexbuffer(&self) -> Option<GLVertexArray> {
        self.context.create_vertex_array()
    }

    fn bind_vertexbuffer(&self, vertex_array: Option<&GLVertexArray>) {
        self.context.bind_vertex_array(vertex_array);
    }

    fn delete_vertexbuffer(&self, vertex_array: &GLVertexArray) {
        self.context.delete_vertex_array(Some(vertex_array));
    }
    
    fn create_framebuffer(&self) -> Option<GLFrameBuffer> {
        self.context.create_framebuffer()
    }

    fn bind_framebuffer(&self, target: GLEnum, framebuffer: Option<&GLFrameBuffer>) {
        self.context.bind_framebuffer(target, framebuffer)
    }

    fn delete_framebuffer(&self, framebuffer: &GLFrameBuffer) {
        self.context.delete_framebuffer(Some(framebuffer))
    }

    fn framebuffer_texture2d(
        &self,
        target: GLEnum,
        attachment: GLEnum,
        textarget: GLEnum,
        texture: &GLTexture,
        level: i32,
    ) {
        self.context
            .framebuffer_texture2_d(target, attachment, textarget, Some(texture), level)
    }
    
    fn framebuffer_texture_layer(
        &self,
        target: GLEnum,
        attachment: GLEnum,
        texture: &GLTexture,
        level: i32,
        layer: i32,
    ) {
        self.context
            .framebuffer_texture_layer(target, attachment, Some(texture), level, layer)
    }

    fn get_attrib_location(&self, program: &Program, name: &str) -> GLUint {
        self.context.get_attrib_location(program, name) as u32
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
        self.context.vertex_attrib_pointer(
            *pointer,
            size,
            type_,
            normalized,
            (stride * mem::size_of::<f32>() as i32) as i32,
            (offset * mem::size_of::<f32>() as i32) as GLintptr,
        ) // todo: offset as custom type
    }

    fn enable_vertex_attrib_array(&self, pointer: &GLUint) {
        self.context.enable_vertex_attrib_array(*pointer)
    }

    fn disable_vertex_attrib_array(&self, pointer: &GLUint) {
        self.context.disable_vertex_attrib_array(*pointer)
    }
    
    fn bind_attrib_location(&self, program: &Program, index: GLUint, name: &str) {
        self.context.bind_attrib_location(program, index, name)
    }

    fn get_uniform_location(&self, program: &Program, name: &str) -> UniformLocation {
        self.context.get_uniform_location(program, name).expect("Uniform location could not be found or does not exist")
    }

    fn uniform_matrix_4fv(&self, location: &UniformLocation, _size: i32, transpose: bool, matrix: &Matrix4<f32>) {
        self.context.uniform_matrix4fv_1(Some(location), transpose, matrix.as_slice())
    }
    
    fn uniform1i(&self, location: &UniformLocation, x: i32) {
        self.context.uniform1i(Some(location), x);
    }
    
    fn uniform1f(&self, location: &UniformLocation, x: f32) {
        self.context.uniform1f(Some(location), x);
    }
    
    fn uniform3f(&self, location: &UniformLocation, x: f32, y: f32, z: f32) {
        self.context.uniform3f(Some(location), x, y, z);
    }
    
    fn create_texture(&self) -> Option<NativeTexture> {
        self.context.create_texture()
    }

    fn bind_texture(&self, target: GLEnum, texture: &NativeTexture) {
        self.context.bind_texture(target, Some(texture));
    }
    
    fn unbind_texture(&self, target: GLEnum) {
        self.context.bind_texture(target, None);
    }
    
    fn tex_parameteri(&self, target: GLEnum, pname: GLEnum, param: i32) {
        self.context.tex_parameteri(target, pname, param)
    }

    fn tex_image2d(
        &self,
        target: GLenum,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: GLenum,
        pixels: Option<&[u8]>,
    ) {
        match pixels {
            Some(pixels) => self.context.tex_image2_d(
                target,
                level,
                internalformat,
                width,
                height,
                border,
                format,
                Self::UNSIGNED_BYTE,
                Some(pixels),
            ),
            None => self.context.tex_image2_d(
                target,
                level,
                internalformat,
                width,
                height,
                border,
                format,
                Self::UNSIGNED_BYTE,
                None::<&TypedArray<u8>>,
            ),
        }
    }

    fn tex_image2d_f(
        &self,
        target: GLenum,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: GLenum,
        pixels: Option<&[f32]>,
    ) {
        match pixels {
            Some(pixels) => self.context.tex_image2_d(
                target,
                level,
                internalformat,
                width,
                height,
                border,
                format,
                Self::FLOAT,
                Some(pixels),
            ),
            None => self.context.tex_image2_d(
                target,
                level,
                internalformat,
                width,
                height,
                border,
                format,
                Self::FLOAT,
                None::<&TypedArray<f32>>,
            ),
        }
    }

    fn tex_sub_image2d(
        &self,
        target: GLenum,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: GLenum,
        pixels: Option<&[u8]>,
    ) {
        match pixels {
            Some(pixels) => self.context.tex_sub_image2_d(
                target,
                level,
                xoffset,
                yoffset,
                width,
                height,
                format,
                Self::UNSIGNED_BYTE,
                Some(pixels),
            ),
            None => self.context.tex_sub_image2_d(
                target,
                level,
                xoffset,
                yoffset,
                width,
                height,
                format,
                Self::UNSIGNED_BYTE,
                None::<&TypedArray<u8>>,
            ),
        }
    }

    fn tex_image3d(
        &self,
        target: GLenum,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: GLenum,
        pixels: Option<&[u8]>,
    ) {
        match pixels {
            Some(pixels) => self.context.tex_image3_d_2(
                target,
                level,
                internalformat,
                width,
                height,
                depth,
                border,
                format,
                Self::UNSIGNED_BYTE,
                Some(pixels),
            ),
            None => self.context.tex_image3_d_2(
                target,
                level,
                internalformat,
                width,
                height,
                depth,
                border,
                format,
                Self::UNSIGNED_BYTE,
                None::<&TypedArray<u8>>,
            ),
        }
    }

    fn tex_image3d_f(
        &self,
        target: GLenum,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: GLenum,
        pixels: Option<&[f32]>,
    ) {
        match pixels {
            Some(pixels) => self.context.tex_image3_d_2(
                target,
                level,
                internalformat,
                width,
                height,
                depth,
                border,
                format,
                Self::FLOAT,
                Some(pixels),
            ),
            None => self.context.tex_image3_d_2(
                target,
                level,
                internalformat,
                width,
                height,
                depth,
                border,
                format,
                Self::FLOAT,
                None::<&TypedArray<f32>>,
            ),
        }
    }

    fn delete_texture(&self, texture: &NativeTexture) {
        self.context.delete_texture(Some(texture));
    }
    
    fn active_texture(&self, _type: GLEnum) {
        self.context.active_texture(_type);
    }
    
    fn generate_mipmap(&self, target: GLEnum) {
        self.context.generate_mipmap(target);
    }

    fn draw_arrays(&self, type_: GLEnum, first: i32, count: i32) {
        self.context.enable(WebGL2RenderingContext::BLEND);
        self.context.blend_func(
            WebGL2RenderingContext::SRC_ALPHA,
            WebGL2RenderingContext::ONE_MINUS_SRC_ALPHA,
        );
        self.context.draw_arrays(type_, first, count)
    }

    fn draw_elements(&self, mode: GLEnum, count: i32, type_: GLEnum, offset: GLintptr) {
        self.context.draw_elements(mode, count, type_, offset);
    }
    
    fn flush(&self) {
        self.context.flush();
    }

    fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        self.context.viewport(x, y, width, height);
    }
    

    fn pixel_storei(&self, pname: GLenum, param: i32) {
        self.context.pixel_storei(pname, param)
    }

    fn enable(&self, cap: GLEnum) {
        self.context.enable(cap);
    }

    fn disable(&self, cap: GLEnum) {
        self.context.disable(cap);
    }
    
    fn depth_mask(&self, flag: bool) {
        self.context.depth_mask(flag);
    }
    
    fn bind_buffer_base(&self, target: GLEnum, index: u32, buffer: Option<&GLBuffer>) {
        self.context.bind_buffer_base(target, index, buffer);
    }

    fn get_buffer_sub_data(&self, target: GLEnum, index: u32, data: &mut [f32]) {
        unimplemented!();
    }

    fn begin_transform_feedback(&self, type_: GLEnum) {
        self.context.begin_transform_feedback(type_);
    }

    fn end_transform_feedback(&self) {
        self.context.end_transform_feedback();
    }
}