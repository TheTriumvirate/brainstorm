//! Methods for interacting with the render target.
use gl_context::{shaders::*, AbstractContext, Buffer, Context, GLEnum};
use graphics::{RenderStates, DrawMode};
use na::Matrix4;

/// Draws vertices/indices to the render target.
/// Precondition: correct shader is bound.
pub fn draw_indices(mode: DrawMode, vertex_data: &Buffer<f32>, index_data: &Buffer<u16>, states: RenderStates, view_matrix: &Matrix4<f32>) {
    let context = Context::get_context();

    let shader: &OurShader = match states.shader {
        Some(s) => s,
        _ => OurShader::default(),
    };

    let proj_mat: Matrix4<f32> = match states.transform {
        Some(m) => m.clone(),
        _ => Matrix4::identity(),
    };

    let mode = match mode {
        DrawMode::TRIANGLES => Context::TRIANGLES,
        DrawMode::LINES => Context::LINES,
    };

    shader.use_program();
    vertex_data.bind();
    index_data.bind();
    shader.bind_attribs();

    let mvp = view_matrix * proj_mat;

    let mvp_uniform = shader.get_uniform_location();
    context.uniform_matrix_4fv(&mvp_uniform, 1, false, &mvp); // FIXME: use shader function isntead!

    if let Some(texture) = &states.texture {
        texture.bind();
        texture.activate(Some(shader));
    }

    context.draw_elements(
        mode,
        index_data.len() as i32,
        Context::UNSIGNED_SHORT,
        0,
    );

    if let Some(texture) = &states.texture {
        texture.unbind();
    }

    shader.unbind_attribs();
}