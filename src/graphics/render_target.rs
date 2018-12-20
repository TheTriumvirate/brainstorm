//! Methods for interacting with the render target.
use crate::graphics::{DrawMode, RenderStates};
use gl_bindings::{shaders::*, AbstractContext, Buffer, Context, GlPrimitive};
use na::Matrix4;

pub fn bind_all(states: &RenderStates, view_matrix: &Matrix4<f32>) {
    let context = Context::get_context();

    let shader: &OurShader = match states.shader {
        Some(ref s) => &s,
        _ => OurShader::default(),
    };

    let proj_mat: Matrix4<f32> = match states.transform {
        Some(m) => *m,
        _ => Matrix4::identity(),
    };

    shader.use_program();
    shader.bind_attribs();

    let mvp = view_matrix * proj_mat;

    let mvp_uniform = shader.get_uniform_location();
    context.uniform_matrix_4fv(&mvp_uniform, 1, false, &mvp); // FIXME: use shader function isntead!

    if let Some(texture) = &states.texture {
        texture.activate(Some(shader), 0, "uSampler");
    }
}

pub fn unbind_all(states: &RenderStates) {
    if let Some(texture) = &states.texture {
        texture.unbind();
    }

    let shader: &OurShader = match states.shader {
        Some(ref s) => &s,
        _ => OurShader::default(),
    };
    shader.unbind_attribs();
}

/// Draws vertices/indices to the render target.
/// Precondition: correct shader is bound.
pub fn draw_indices<T>(
    mode: DrawMode,
    vertex_data: &Buffer<f32>,
    index_data: &Buffer<T>,
    states: &RenderStates,
    view_matrix: &Matrix4<f32>,
) where
    T: GlPrimitive,
{
    let context = Context::get_context();
    vertex_data.bind();
    index_data.bind();
    bind_all(&states, view_matrix);

    let mode = match mode {
        DrawMode::TRIANGLES => Context::TRIANGLES,
        DrawMode::LINES => Context::LINES,
        DrawMode::LINESTRIP => Context::LINE_STRIP,
        DrawMode::POINTS => Context::POINTS,
    };

    context.draw_elements(mode, index_data.len() as i32, T::primitive(), 0);
    unbind_all(&states);
}

pub fn draw_vertex_array(
    mode: DrawMode,
    first: i32,
    count: i32,
    vertex_data: &Buffer<f32>,
    states: &RenderStates,
    view_matrix: &Matrix4<f32>,
) {
    let context = Context::get_context();

    vertex_data.bind();
    bind_all(&states, view_matrix);

    let mode = match mode {
        DrawMode::TRIANGLES => Context::TRIANGLES,
        DrawMode::LINES => Context::LINES,
        DrawMode::LINESTRIP => Context::LINE_STRIP,
        DrawMode::POINTS => Context::POINTS,
    };
    context.draw_arrays(mode, first, count);

    unbind_all(&states);
}
