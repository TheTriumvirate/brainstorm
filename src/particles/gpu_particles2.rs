use gl_bindings::{Texture, TextureFormat, Buffer, shaders::OurShader, shaders::ShaderAttribute, BufferType, FrameBuffer, Context, AbstractContext, VertexBuffer};
use graphics::{Drawable, render_target, DrawMode};
use resources::shaders::{GPU_PARTICLES_VERTEX_SHADER, GPU_PARTICLES_FRAGMENT_SHADER, GPU_PARTICLES_UPDATE_VERTEX_SHADER, GPU_PARTICLES_UPDATE_FRAGMENT_SHADER};

use particles::gpu_fieldprovider::GPUFieldProvider;

use gl_bindings::{shaders::ShaderType};

use std::str;
use std::rc::Rc;
use na::Matrix4;

use State;
use camera::{ArcBall};

use rand::{FromEntropy, Rng};
use rand::rngs::SmallRng;

const TEXTURESIZE: usize = 512;
const MAXSTREAMLETSIZE: usize = 8;

pub struct GPUParticleEngine {
    noise: Rc<Texture>,

    vertices1: Buffer<f32>,
    indices1: Buffer<u32>,
    vertices2: Buffer<f32>,
    indices2: Buffer<u32>,

    draw_shader: Rc<OurShader>,
    update_shader: Rc<OurShader>,

    swap: bool,
    update: bool,
}

impl GPUParticleEngine {
    pub fn new() -> Self {
        let mut particle_data = Vec::new();
        let mut noise_data = Vec::new();
        let mut rng = SmallRng::from_entropy();
        let mut index_data = Vec::new();
        let mut index = 0;
        const offset : u32 = (TEXTURESIZE * TEXTURESIZE) as u32;
        for q in 0..MAXSTREAMLETSIZE {
            for u in 0..(TEXTURESIZE) {
                for v in 0..(TEXTURESIZE) {
                    particle_data.push(0.0);
                    particle_data.push(0.0);
                    particle_data.push(0.0);

                    if q == 0 {
                        noise_data.push(rng.gen_range::<f32>(0.0, 1.0));
                        noise_data.push(rng.gen_range::<f32>(0.0, 1.0));
                        noise_data.push(rng.gen_range::<f32>(0.0, 1.0));
                        noise_data.push(rng.gen_range::<f32>(0.0, 1.0));
                    } else {
                        index_data.push(index-offset);
                        index_data.push(index);
                    }
                    index += 1;
                }
            }
        }

        let shader: OurShader = OurShader::new(
            str::from_utf8(GPU_PARTICLES_VERTEX_SHADER).expect("Failed to read vertex shader"),
            str::from_utf8(GPU_PARTICLES_FRAGMENT_SHADER).expect("Failed to read fragment shader"),
            &[
                ShaderAttribute {
                    name: "v_pos".to_string(),
                    size: 3
                },
            ]
        );
        
        let update_shader: OurShader = OurShader::new_transform_feedback(
            str::from_utf8(GPU_PARTICLES_UPDATE_VERTEX_SHADER).expect("Failed to read vertex shader"),
            str::from_utf8(GPU_PARTICLES_UPDATE_FRAGMENT_SHADER).expect("Failed to read fragment shader"),
            &[
                ShaderAttribute {
                    name: "v_pos".to_string(),
                    size: 3
                },
            ],
            "o_pos"
        );

        let mut vertices1: Buffer<f32> = Buffer::new(BufferType::Array);
        vertices1.set_data(&particle_data[..]);

        vertices1.bind();
        let len = vertices1.len();
        vertices1.upload_data(0, len, true);

        let mut indices1: Buffer<u32> = Buffer::new(BufferType::IndexArray);
        indices1.set_data(&index_data[..]);

        indices1.bind();
        let len = indices1.len();
        indices1.upload_data(0, len, true);

        let mut vertices2: Buffer<f32> = Buffer::new(BufferType::Array);
        vertices2.set_data(&particle_data[..]);

        vertices2.bind();
        let len = vertices2.len();
        vertices2.upload_data(0, len, true);

        let mut indices2: Buffer<u32> = Buffer::new(BufferType::IndexArray);
        indices2.set_data(&index_data[..]);

        indices2.bind();
        let len = indices2.len();
        indices2.upload_data(0, len, true);

        GPUParticleEngine {
            noise: Rc::new(Texture::from_data(TEXTURESIZE as u32, TEXTURESIZE as u32, TextureFormat::RGBA, &noise_data[..])),
            vertices1,
            indices1,
            vertices2,
            indices2,
            draw_shader: Rc::new(shader),
            update_shader: Rc::new(update_shader),
            swap: false,
            update: false,
        }
    }

    pub fn update(&mut self, field_provider: &GPUFieldProvider, state: &State, camera: &ArcBall) {
        let context = Context::get_context();

        field_provider.get_texture().activate(Some(&self.update_shader), 0, "u_data");
        self.noise.activate(Some(&self.update_shader), 1, "u_noise");
        self.update_shader.uniform1f("u_size", TEXTURESIZE as f32);
        
        self.update_shader.uniform1f("u_speed", state.speed_multiplier * 0.016);
        self.update_shader.uniform1f("u_lowpass", state.lowpass_filter);
        self.update_shader.uniform1f("u_highpass", state.highpass_filter);
        self.update_shader.uniform1f("u_seedsize", state.seeding_size * 0.6 + 0.01);
        
        let (cx, cy, cz) = camera.get_target();
        self.update_shader.uniform3f("u_seedpos", cx + 0.5, cy + 0.5, cz + 0.5);

        self.update = true;
        {
            let (update_vertices, _) = self.update_buffer();
            let (draw_vertices, _) = self.draw_buffer();


            //draw_vbo.bind();
            draw_vertices.bind();
            render_target::bind_all(&self.render_states(), &Matrix4::identity());

            update_vertices.bind_buffer_base();
            self.update_shader.transform_feedback_varyings("o_pos");

            context.enable(Context::RASTERIZER_DISCARD);
            context.begin_transform_feedback(Context::POINTS);

            let len = draw_vertices.len() as i32 / 2 / (MAXSTREAMLETSIZE) as i32;
            context.draw_arrays(Context::POINTS, 0, len);
                context.end_transform_feedback();

            context.disable(Context::RASTERIZER_DISCARD);

            update_vertices.unbind_buffer_base();
            render_target::unbind_all(&self.render_states());
            //draw_vbo.unbind();
        }
        self.swap = !self.swap;
        self.update = false;
    }

    pub fn draw_buffer(&self) -> (&Buffer<f32>, &Buffer<u32>) {
        if !self.swap {
            (&self.vertices1, &self.indices1)
        } else {
            (&self.vertices2, &self.indices2)
        }
    }

    pub fn update_buffer(&self) -> (&Buffer<f32>, &Buffer<u32>) {
        if self.swap {
            (&self.vertices1, &self.indices1)
        } else {
            (&self.vertices2, &self.indices2)
        }
    }
}

impl Drawable for GPUParticleEngine {
    fn get_shader(&self) -> Option<Rc<OurShader>> {
        if self.update {
            Some(self.update_shader.clone())
        }
        else {
            Some(self.draw_shader.clone())
        }
    }

    fn draw_transformed(&self, view_matrix: &Matrix4<f32>) {
        let (vertices, indices) = self.draw_buffer();
        //render_target::draw_indices(DrawMode::LINES, &self.vertices1, &self.indices1, self.render_states(), view_matrix);
        let len = vertices.len() as i32 / 2 / (MAXSTREAMLETSIZE) as i32;
        render_target::draw_vertex_array(DrawMode::POINTS, 0, len, vertices, self.render_states(), view_matrix);
    }
}