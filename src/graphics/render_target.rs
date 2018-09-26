//! Methods for interacting with the render target.
use gl_context::{shaders::*, AbstractContext, Buffer, Context, Texture};

/// Draws vertices/indices to the render target.
/// Precondition: correct shader is bound.
pub fn draw_indices(vertex_data: &Buffer<f32>, index_data: &Buffer<u16>) {
    let context = Context::get_context();

    OurShader::default().use_program();
    vertex_data.bind();
    index_data.bind();
    OurShader::default().bind_attribs();
    //Texture::default().activate(OurShader::default());

    context.draw_elements(
        Context::TRIANGLES,
        index_data.len() as i32,
        Context::UNSIGNED_SHORT,
        0,
    );
}


pub fn test(vertex_data: &Buffer<f32>, index_data: &Buffer<u16>, texture: &Texture) {
    let context = Context::get_context();

    OurShader::default().use_program();
    vertex_data.bind();
    index_data.bind();
    OurShader::default().bind_attribs();
    texture.activate(Some(OurShader::default()));

    context.draw_elements(
        Context::TRIANGLES,
        index_data.len() as i32,
        Context::UNSIGNED_SHORT,
        0,
    );
}