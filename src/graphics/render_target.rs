use gl_context::{shaders::*, AbstractContext, Context, Buffer};
use std::marker::Sized;

use super::Drawable;

pub trait RenderTarget {
    fn draw(&self, drawable: impl Drawable) where Self: Sized {
        drawable.draw(self);
    }

    // pre-condition: correct shader is bound
    fn draw_indices(&self, vertex_data: &Buffer<f32>, index_data: &Buffer<u16>) {
        let context = Context::get_context();
        
        OurShader::default().use_program();
        vertex_data.bind();
        index_data.bind();
        OurShader::default().bind_attribs();

        context.draw_elements(Context::TRIANGLES, index_data.len() as i32, Context::UNSIGNED_SHORT, 0);
    }
}

impl<'a, T> RenderTarget for &'a T where T: RenderTarget {}