use gl_context::{Texture, Buffer, BufferType};
use graphics::{Drawable, DrawMode, render_target, position};
use na::Matrix4;
use std::rc::Rc;

/// Represents a drawable rectangle.
pub struct Rectangle {
    vertices: Buffer<f32>,
    indices: Buffer<u16>,
    texture: Option<Rc<Texture>>
}

impl Rectangle {
    /// Creates a new rectangle with the chosen parameters.
    pub fn new(pos: position::Coordinates, color: (f32, f32, f32)) -> Self {
        let mut vertices: Buffer<f32> = Buffer::new(BufferType::Array);
        let mut indices: Buffer<u16> = Buffer::new(BufferType::IndexArray);

        vertices.set_data(&[
            pos.x1, pos.y1, 0.0, color.0, color.1, color.2, 0.0, 0.0, 
            pos.x2, pos.y1, 0.0, color.0, color.1, color.2, 1.0, 0.0, 
            pos.x2, pos.y2, 0.0, color.0, color.1, color.2, 1.0, 1.0, 
            pos.x1, pos.y2, 0.0, color.0, color.1, color.2, 0.0, 1.0,
        ]);
        indices.set_data(&[0, 1, 2, 0, 2, 3]);

        vertices.bind();
        let len = vertices.len();
        vertices.upload_data(0, len, true);

        indices.bind();
        let len = indices.len();
        indices.upload_data(0, len, true);

        Rectangle { vertices, indices, texture: None }
    }

    pub fn set_texture(&mut self, texture: Option<Rc<Texture>>) {
        self.texture = texture;
    }
}

impl Drawable for Rectangle {
    fn get_texture(&self) -> Option<Rc<Texture>> {
        self.texture.clone()
    }

    fn draw_transformed(&self, view_matrix: &Matrix4<f32>) {
        render_target::draw_indices(DrawMode::TRIANGLES, &self.vertices, &self.indices, self.render_states(), view_matrix);
    }
}
