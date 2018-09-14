use gl_context::{Buffer, BufferType};
use graphics::{render_target, Drawable};

pub struct Rectangle {
    vertices: Buffer<f32>,
    indices: Buffer<u16>,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32, color: (f32, f32, f32)) -> Self {
        let mut vertices: Buffer<f32> = Buffer::new(BufferType::Array);
        let mut indices: Buffer<u16> = Buffer::new(BufferType::IndexArray);

        vertices.set_data(&[
            x,
            y,
            color.0,
            color.1,
            color.2,
            0.0,
            0.0,
            x + width,
            y,
            color.0,
            color.1,
            color.2,
            0.0,
            0.0,
            x + width,
            y + height,
            color.0,
            color.1,
            color.2,
            0.0,
            0.0,
            x,
            y + height,
            color.0,
            color.1,
            color.2,
            0.0,
            0.0,
        ]);
        indices.set_data(&[0, 1, 2, 0, 2, 3]);

        vertices.bind();
        let len = vertices.len();
        vertices.upload_data(0, len, true);

        indices.bind();
        let len = indices.len();
        indices.upload_data(0, len, true);

        Rectangle { vertices, indices }
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        render_target::draw_indices(&self.vertices, &self.indices);
    }
}
