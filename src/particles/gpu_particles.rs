use gl_bindings::{Texture, TextureFormat, Buffer, shaders::OurShader, shaders::ShaderAttribute, BufferType};
use graphics::{Drawable, render_target, DrawMode};
use resources::shaders::{GPU_PARTICLES_VERTEX_SHADER, GPU_PARTICLES_FRAGMENT_SHADER};

use std::str;
use std::rc::Rc;
use na::Matrix4;

pub struct GPUParticleEngine {
    texture: Rc<Texture>,
    vertices: Buffer<f32>,
    shader: Rc<OurShader>
}

impl GPUParticleEngine {
    pub fn new() -> Self {
        let mut data = Vec::new();
        let mut particle_data = Vec::new();
        for u in 0..2048 {
            for v in 0..2048 {
                data.push(u as f32 / 2048.0);
                data.push(v as f32 / 2048.0);
                data.push(u as f32 / 2048.0);
                data.push(1.0);
                particle_data.push(u as f32 / 2048.0);
                particle_data.push(v as f32 / 2048.0);
            }
        }
        let shader: OurShader = OurShader::new(
            str::from_utf8(GPU_PARTICLES_VERTEX_SHADER).expect("Failed to read vertex shader"),
            str::from_utf8(GPU_PARTICLES_FRAGMENT_SHADER).expect("Failed to read fragment shader"),
            &[
                ShaderAttribute {
                    name: "v_texpos".to_string(),
                    size: 2
                },
            ]
        );
        let mut vertices: Buffer<f32> = Buffer::new(BufferType::Array);
        vertices.set_data(&particle_data[..]);

        vertices.bind();
        let len = vertices.len();
        vertices.upload_data(0, len, true);

        GPUParticleEngine {
            texture: Rc::new(Texture::from_data(2048, 2048, TextureFormat::RGBA, &data[..])),
            vertices,
            shader: Rc::new(shader),
        }
    }
}

impl Drawable for GPUParticleEngine {
    fn get_texture(&self) -> Option<Rc<Texture>> {
        Some(self.texture.clone())
    }

    fn get_shader(&self) -> Option<Rc<OurShader>> {
        Some(self.shader.clone())
    }

    fn draw_transformed(&self, view_matrix: &Matrix4<f32>) {
        let len = self.vertices.len() as i32 / 2;
        render_target::draw_vertex_array(DrawMode::POINTS, 0, len, &self.vertices, self.render_states(), view_matrix)
    }
}