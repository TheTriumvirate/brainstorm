use particles::fieldprovider::FieldProvider;

use gl_context::{Buffer, BufferType};

use graphics::*;

use na::Matrix4;

pub struct Streamlines {
    vertices: Buffer<f32>
}

const MAX_STEPS: usize = 100;
const FACTOR: f32 = 0.016;

impl Drawable for Streamlines {
    fn draw_transformed(&self, view_matrix: &Matrix4<f32>) {
        let len = self.vertices.len() as i32 / 6;
        render_target::draw_vertex_array(DrawMode::LINES, 0, len, &self.vertices, RenderStates::from(self), view_matrix)
    }
}

impl Streamlines {
    pub fn new() -> Self {
        Streamlines {
            vertices: Buffer::<f32>::new(BufferType::Array)
        }
    }

    pub fn draw_streamlines(&mut self, field: &FieldProvider, start: (f32, f32, f32)) {
        self.vertices.clear();
        const S : f32 = 0.01;

        {
            let (x, y, z) = start;
            self.vertices.push(&Streamlines::create_line(field, (x, y, z))[..]);
            self.vertices.push(&Streamlines::create_line(field, (x+S, y, z))[..]);
            self.vertices.push(&Streamlines::create_line(field, (x-S, y, z))[..]);
            self.vertices.push(&Streamlines::create_line(field, (x, y+S, z))[..]);
            self.vertices.push(&Streamlines::create_line(field, (x, y-S, z))[..]);
            self.vertices.push(&Streamlines::create_line(field, (x, y, z+S))[..]);
            self.vertices.push(&Streamlines::create_line(field, (x, y, z-S))[..]);
        }

        self.vertices.bind();
        let len = self.vertices.len();
        self.vertices.upload_data(0, len, true);
    }

    fn create_line(field: &FieldProvider, start: (f32, f32, f32)) -> Vec<f32> {
        let mut data = Vec::new();

        let (mut x, mut y, mut z) = start;

        for _i in 1..MAX_STEPS {
            let (dx, dy, dz, fa) = field.delta((x, y, z));
            let (dx, dy, dz) = (dx * fa * FACTOR, dy * fa * FACTOR, dz * fa * FACTOR);

            if fa < 0.001 {
                // TODO: This probably won't work, fix later
                return data;
            }

            data.push(x);
            data.push(y);
            data.push(z);
            data.push(1.0);
            data.push(1.0);
            data.push(1.0);
            data.push(1.0);
            data.push(1.0);
            
            x = x + dx;
            y = y + dy;
            z = z + dz;

            data.push(x);
            data.push(y);
            data.push(z);
            data.push(1.0);
            data.push(1.0);
            data.push(1.0);
            data.push(1.0);
            data.push(1.0);
        }

        data
    }
}
