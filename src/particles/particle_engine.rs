use na::Matrix4;
use rand::rngs::SmallRng;
use rand::{FromEntropy, Rng};

use std::str;

use gl_context::{shaders, AbstractContext, Buffer, BufferType, Context, UniformLocation};
use particles::fieldprovider::{FieldProvider, SphereFieldProvider};

const PARTICLE_COUNT: usize = 100_000;

#[derive(Clone, Debug)]
struct ParticleData {
    position: (f32, f32, f32),
    is_alive: bool,
    lifetime: f32,
}

pub struct ParticleEngine {
    particles: Vec<ParticleData>,
    particle_data: Buffer<f32>,
    field_provider: SphereFieldProvider,
    rng: SmallRng,
    mvp_uniform: UniformLocation,
    shader: shaders::OurShader,
    alive_count: usize,
}

impl Default for ParticleEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ParticleEngine {
    pub fn new() -> Self {
        let mut rng = SmallRng::from_entropy();
        // Set up particles.
        let mut data: Buffer<f32> = Buffer::new(BufferType::Array);
        data.resize(PARTICLE_COUNT * 4, 0.0);
        let mut particles = Vec::with_capacity(PARTICLE_COUNT);
        for i in 0..PARTICLE_COUNT {
            particles.push(ParticleData {
                position: (
                    rng.gen_range::<f32>(-0.5, 0.5),
                    rng.gen_range::<f32>(-0.5, 0.5),
                    rng.gen_range::<f32>(-0.5, 0.5),
                ),
                is_alive: true,
                lifetime: (i as f32 / PARTICLE_COUNT as f32) * 100.0,
            });
        }
        //particles.resize(PARTICLE_COUNT, ParticleData {position: (0.0, 0.0, 0.0), is_alive: true, lifetime: 0.0});

        // Bind the window buffer.
        /*let mut vb = Buffer::new();
        vb.set_data(&data[..], false);*/
        data.bind();

        // Set up shaders
        let vertex_shader =
            str::from_utf8(shaders::PARTICLES_VERTEX_SHADER).expect("Failed to read vertex shader");
        let fragment_shader = str::from_utf8(shaders::PARTICLES_FRAGMENT_SHADER)
            .expect("Failed to read fragment shader");

        let mut attributes = Vec::new();
        attributes.push(shaders::ShaderAttribute {
            name: "position".to_string(),
            size: 4,
        });

        let shader = shaders::OurShader::new(vertex_shader, fragment_shader, &attributes);
        shader.use_program();

        let mvp_uniform = shader.get_uniform_location();

        ParticleEngine {
            particles,
            particle_data: data,
            field_provider: SphereFieldProvider::new(),
            rng,
            shader,
            mvp_uniform,
            alive_count: 0,
        }
    }

    pub fn update(&mut self) {
        self.alive_count = 0;
        for i in 0..PARTICLE_COUNT {
            if self.particles[i].lifetime > 100.0
            /* && self.particles[i].isAlive && end > i*/
            {
                {
                    let mut data = &mut self.particles[i];
                    data.lifetime = 0.0;
                    data.position = (
                        self.rng.gen_range::<f32>(-0.5, 0.5),
                        self.rng.gen_range::<f32>(-0.5, 0.5),
                        self.rng.gen_range::<f32>(-0.5, 0.5),
                    );
                }
                //  self.particles.swap(i, end);
            }

            let mut data = &mut self.particles[i];

            if data.is_alive {
                let delta = self.field_provider.delta(data.position);
                data.position.0 += delta.0 * 0.01;
                data.position.1 += delta.1 * 0.01;
                data.position.2 += delta.2 * 0.01;

                let dist = (delta.0 * delta.0 + delta.1 * delta.1 + delta.2 * delta.2).sqrt();

                self.particle_data[self.alive_count * 4] = data.position.0;
                self.particle_data[self.alive_count * 4 + 1] = data.position.1;
                self.particle_data[self.alive_count * 4 + 2] = data.position.2;
                self.particle_data[self.alive_count * 4 + 3] = dist * 4.0;

                data.lifetime += 1.0;
                self.alive_count += 1;
            } else {
                break;
            }
        }
    }

    pub fn draw(&mut self, projection_matrix: &Matrix4<f32>) {
        let context = Context::get_context();
        if self.alive_count > 0 {
            self.particle_data.bind();
            self.particle_data
                .upload_data(0, self.alive_count * 4, false);
            /*context.bind_buffer(Context::ARRAY_BUFFER, &self.vertex_buffer);
            context.buffer_data(
                Context::ARRAY_BUFFER,
                &self.particle_data[0..self.alive_count * 4],
                Context::DYNAMIC_DRAW,
            );*/
            self.shader.use_program();
            self.shader.bind_attribs();
            context.uniform_matrix_4fv(&self.mvp_uniform, 1, false, &projection_matrix);
            context.draw_arrays(Context::POINTS, 0, self.alive_count as i32);
        }
    }
}
