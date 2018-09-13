use gl_context::{Buffer, BufferType};
use super::{Drawable, RenderTarget};

use std::f32;

const POINTS: usize = 40;

pub struct Circle {
    vertices: Buffer<f32>,
    indices: Buffer<u16>,
}

impl Circle {
    pub fn new(x: f32, y: f32, radius: f32) -> Self {
        let mut vertices : Buffer<f32> = Buffer::new(BufferType::Array);
        let mut indices : Buffer<u16> = Buffer::new(BufferType::IndexArray);

        let mut vertex_data = Vec::new();
        vertex_data.extend_from_slice(&[x, y, 1.0, 1.0, 1.0, 0.0, 0.0]);

        let mut index_data = Vec::new();

        for i in 0..POINTS {
            let progress = i as f32 / POINTS as f32;
            let dt = progress * f32::consts::PI * 2.0;
            let dx = x + radius * dt.cos(); 
            let dy = y + radius * dt.sin();
            vertex_data.extend_from_slice(&[dx, dy, 1.0, 1.0, 1.0, 0.0, 0.0]);

            if i > 0 {
                let idx = i as u16;
                index_data.extend_from_slice(&[0, idx+1, idx]);
            }
        }
        index_data.extend_from_slice(&[0, 1, POINTS as u16]);

        vertices.set_data(&vertex_data[..]);
        indices.set_data(&index_data[..]);

        vertices.bind();
        let len = vertices.len();
        vertices.upload_data(0, len, true);

        indices.bind();
        let len = indices.len();
        indices.upload_data(0, len, true);

        Circle {
            vertices,
            indices,
        }
    }
}

impl<'a> Drawable for &'a Circle {
    fn draw(&self, target: impl RenderTarget) {
        target.draw_indices(&self.vertices, &self.indices);
    }
}