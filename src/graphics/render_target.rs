//! Methods for interacting with the render target.
use gl_context::{shaders::*, AbstractContext, Buffer, Context};
use graphics::RenderStates;

/// Draws vertices/indices to the render target.
/// Precondition: correct shader is bound.
pub fn draw_indices(vertex_data: &Buffer<f32>, index_data: &Buffer<u16>, states: RenderStates) {
    let context = Context::get_context();

    let shader : &OurShader =  match states.shader {
        Some(s) => s,
        _ => OurShader::default(),
    };

    shader.use_program();
    vertex_data.bind();
    index_data.bind();
    shader.bind_attribs();

    if let Some(texture) = &states.texture {
        texture.bind();
        texture.activate(Some(shader));
    }
    //Texture::default().activate(OurShader::default());

    context.draw_elements(
        Context::TRIANGLES,
        index_data.len() as i32,
        Context::UNSIGNED_SHORT,
        0,
    );
    
    if let Some(texture) = &states.texture {
        texture.unbind();
    }
}