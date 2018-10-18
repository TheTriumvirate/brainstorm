use particles::fieldprovider::FieldProvider;

use gl_context::{shaders, Buffer, BufferType};

use resources::shaders::{STREAMLINES_VERTEX_SHADER, STREAMLINES_FRAGMENT_SHADER};

use graphics::{render_target, DrawMode, Drawable};
use std::str;
use na::Matrix4;

pub struct Streamlines {
    vertices: Buffer<f32>,
    shader: shaders::OurShader,
    transparency: f32
}

impl Drawable for Streamlines {
    fn get_shader(&self) -> Option<&shaders::OurShader> {
        Some(&self.shader)
    }

    fn draw_transformed(&self, view_matrix: &Matrix4<f32>) {
        let len = self.vertices.len() as i32 / 3;
        self.shader.uniform1f("u_transparency", self.transparency);
        render_target::draw_vertex_array(DrawMode::LINES, 0, len, &self.vertices, self.render_states(), view_matrix)
    }
}

impl Streamlines {
    pub fn new() -> Self {
        // Set up shaders
        let vertex_shader =
            str::from_utf8(STREAMLINES_VERTEX_SHADER).expect("Failed to read vertex shader");
        let fragment_shader =
            str::from_utf8(STREAMLINES_FRAGMENT_SHADER).expect("Failed to read fragment shader");

        let mut attributes = Vec::new();
        attributes.push(shaders::ShaderAttribute {
            name: "position".to_string(),
            size: 3,
        });
        
        let shader = shaders::OurShader::new(vertex_shader, fragment_shader, &attributes);
        shader.use_program();

        Streamlines {
            vertices: Buffer::<f32>::new(BufferType::Array),
            shader,
            transparency: 0.0
        }
    }

    pub fn draw_streamlines(&mut self, step_size: f32, step_count: i32, bounds: f32, field: &FieldProvider, start: (f32, f32, f32)) {
        self.vertices.clear();
        //const S : f32 = 0.05;
        let S = bounds / 6.0;
        self.transparency = (S *12.0).min(1.0).max(0.05);  

        {
            let step_count = step_count.max(1);
            let (x, y, z) = start;
            let (x, y, z) = (x - bounds, y - bounds, z - bounds);
            let (ox, oy, oz) = (x % S, y % S, z % S);

            let mut i = 0.0;
            while i < bounds * 2.0 - ox * 2.0 {
                let mut j = 0.0;
                while j < bounds * 2.0 - oy * 2.0 {
                    let mut k = 0.0;
                    while k < bounds * 2.0 - oz * 2.0 {
                        if (i - bounds - ox).powf(2.0) + (j - bounds - oy).powf(2.0) + (k - bounds - oz).powf(2.0) <= bounds * bounds {
                            self.vertices.push(&Streamlines::create_line(field, step_size, step_count, (x+i - ox, y+j - oy, z+k - oz))[..]);
                        }
                        k += S;
                    }
                    j += S;
                }
                i += S;
            }
        }

        self.vertices.bind();
        let len = self.vertices.len();
        if len > 0 {
            self.vertices.upload_data(0, len, true);
        }
    }

    fn create_line(field: &FieldProvider, step_size: f32, step_count: i32, start: (f32, f32, f32)) -> Vec<f32> {
        let mut data = Vec::new();

        let (mut x, mut y, mut z) = start;

        let (mut px, mut py, mut pz) = start;

        let mut k = -1;
        for _i in 1..step_count {
            let (dx, dy, dz, fa) = field.delta((x, y, z));
            let (dx, dy, dz) = (dx * fa * step_size, dy * fa * step_size, dz * fa * step_size);

            k += 1; 
            if dx.is_nan() || dy.is_nan() || dz.is_nan() {
                return data;
            }
            
            x = x + dx;
            y = y + dy;
            z = z + dz;

            if k % 10 != 0 { continue}

            data.push(px);
            data.push(py);
            data.push(pz);

            data.push(x);
            data.push(y);
            data.push(z);

            px = x;
            py = y;
            pz = z;
        }

        data
    }
}
