use gl_context::{Buffer, BufferType};
use graphics::*;
use na::Matrix4;

/// Represents a drawable rectangle.
pub struct Cube {
    vertices: Buffer<f32>,
    indices: Buffer<u16>,
}

impl Cube {
    /// Creates a new rectangle with the chosen parameters.
    pub fn new(pos: (f32, f32, f32), size: (f32, f32, f32), color: (f32, f32, f32)) -> Self {
        let mut vertices: Buffer<f32> = Buffer::new(BufferType::Array);
        let mut indices: Buffer<u16> = Buffer::new(BufferType::IndexArray);

        let (x, y, z) = pos;
        let (w, h, d) = size;

        let mut data = Vec::new();

        /*
        // Side #1
        data.extend_from_slice(&[x  , y  , z  , color.0, color.1, color.2, 0.0, 0.0]);
        data.extend_from_slice(&[x+w, y  , z  , color.0, color.1, color.2, 1.0, 0.0]);
        data.extend_from_slice(&[x+w, y+h, z  , color.0, color.1, color.2, 1.0, 1.0]);
        data.extend_from_slice(&[x  , y+h, z  , color.0, color.1, color.2, 0.0, 1.0]);
        
        // Side #1
        data.extend_from_slice(&[x  , y  , z  , color.0, color.1, color.2, 0.0, 0.0]);
        data.extend_from_slice(&[x  , y  , z+d, color.0, color.1, color.2, 1.0, 0.0]);
        data.extend_from_slice(&[x  , y+h, z+d, color.0, color.1, color.2, 1.0, 1.0]);
        data.extend_from_slice(&[x  , y+h, z  , color.0, color.1, color.2, 0.0, 1.0]);
        
        // Side #1
        data.extend_from_slice(&[x  , y  , z  , color.0, color.1, color.2, 0.0, 0.0]);
        data.extend_from_slice(&[x+w, y  , z  , color.0, color.1, color.2, 1.0, 0.0]);
        data.extend_from_slice(&[x+w, y  , z+d, color.0, color.1, color.2, 1.0, 1.0]);
        data.extend_from_slice(&[x  , y  , z+d, color.0, color.1, color.2, 0.0, 1.0]);
        
        // Side #1
        data.extend_from_slice(&[x  , y  , z+d, color.0, color.1, color.2, 0.0, 0.0]);
        data.extend_from_slice(&[x+w, y  , z+d, color.0, color.1, color.2, 1.0, 0.0]);
        data.extend_from_slice(&[x+w, y+h, z+d, color.0, color.1, color.2, 1.0, 1.0]);
        data.extend_from_slice(&[x  , y+h, z+d, color.0, color.1, color.2, 0.0, 1.0]);
        
        // Side #1
        data.extend_from_slice(&[x+w, y  , z  , color.0, color.1, color.2, 0.0, 0.0]);
        data.extend_from_slice(&[x+w, y  , z+d, color.0, color.1, color.2, 1.0, 0.0]);
        data.extend_from_slice(&[x+w, y+h, z+d, color.0, color.1, color.2, 1.0, 1.0]);
        data.extend_from_slice(&[x+w, y+h, z  , color.0, color.1, color.2, 0.0, 1.0]);
        
        // Side #1
        data.extend_from_slice(&[x  , y+h, z  , color.0, color.1, color.2, 0.0, 0.0]);
        data.extend_from_slice(&[x+w, y+h, z  , color.0, color.1, color.2, 1.0, 0.0]);
        data.extend_from_slice(&[x+w, y+h, z+d, color.0, color.1, color.2, 1.0, 1.0]);
        data.extend_from_slice(&[x  , y+h, z+d, color.0, color.1, color.2, 0.0, 1.0]);
        */

        data.extend_from_slice(&[x  , y  , z  , color.0, color.1, color.2, 0.0, 0.0]);
        data.extend_from_slice(&[x+w, y  , z  , color.0, color.1, color.2, 0.0, 0.0]);
        data.extend_from_slice(&[x  , y+h, z  , color.0, color.1, color.2, 0.0, 0.0]);
        data.extend_from_slice(&[x  , y  , z+d, color.0, color.1, color.2, 0.0, 0.0]);
        data.extend_from_slice(&[x+w, y+h, z  , color.0, color.1, color.2, 0.0, 0.0]);
        data.extend_from_slice(&[x+w, y  , z+d, color.0, color.1, color.2, 0.0, 0.0]);
        data.extend_from_slice(&[x  , y+h, z+d, color.0, color.1, color.2, 0.0, 0.0]);
        data.extend_from_slice(&[x+w, y+h, z+d, color.0, color.1, color.2, 0.0, 0.0]);

        indices.set_data(&[
            0, 1,
            0, 2,
            0, 3,
            1, 4,
            1, 5,
            3, 5,
            3, 6,
            5, 7,
            2, 4,
            2, 6,
            4, 7,
            6, 7,
            ]);

        vertices.set_data(&data[..]);

        vertices.bind();
        let len = vertices.len();
        vertices.upload_data(0, len, true);

        indices.bind();
        let len = indices.len();
        indices.upload_data(0, len, true);

        Cube { vertices, indices }
    }
}

impl Drawable for Cube {
    fn draw_transformed(&self, view_matrix: &Matrix4<f32>) {
        render_target::draw_indices(DrawMode::LINES, &self.vertices, &self.indices, self.render_states(), view_matrix);
    }
}
