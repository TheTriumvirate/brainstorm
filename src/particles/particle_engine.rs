use na::Matrix4;
use rand::rngs::SmallRng;
use rand::{FromEntropy, Rng};

use std::str;

use gl_context::{shaders, AbstractContext, Buffer, BufferType, Context, UniformLocation};
use particles::fieldprovider::{FieldProvider, SphereFieldProvider};
use State;

const PARTICLE_COUNT: usize = 100_000;

/// Struct containing the data for a single particle.
#[derive(Clone, Debug)]
struct ParticleData {
    position: (f32, f32, f32),
    lifetime: f32,
}

/// The particle engine itself.
/// Hold all particles and is responsible for updating the system.
pub struct ParticleEngine {
    particles: Vec<ParticleData>,
    particle_data: Buffer<f32>,
    field_provider: SphereFieldProvider,
    rng: SmallRng,
    mvp_uniform: UniformLocation,
    shader: shaders::OurShader,
    alive_count: usize,
    max_dist: f32,
}

impl Default for ParticleEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ParticleEngine {
    /// Initializes a new particle engine.
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
                lifetime: (i as f32 / PARTICLE_COUNT as f32) * 100.0,
            });
        }
        
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

        // Find the max velocity to be used with the high-pass filter later.
        let field_provider = SphereFieldProvider::new();
        let mut max_dist: f32 = 0.0;
        for delta in field_provider.data() {
            let dist = (delta.0 * delta.0 + delta.1 * delta.1 + delta.2 * delta.2).sqrt();
            max_dist = max_dist.max(dist);
        }
        
        ParticleEngine {
            particles,
            particle_data: data,
            field_provider,
            rng,
            shader,
            mvp_uniform,
            alive_count: 0,
            max_dist,
        }
    }

    /// Update the particle system, advancing 1 tick.
    /// Uses settings from `state` to let the user interface with the system.
    pub fn update(&mut self, state: &State) {
        self.alive_count = 0;
        for i in 0..PARTICLE_COUNT {
            // Respawn particle if it's too old.
            if self.particles[i].lifetime > 100.0 {
                let mut data = &mut self.particles[i];
                data.lifetime = 0.0;
                data.position = (
                    self.rng.gen_range::<f32>(-0.5, 0.5),
                    self.rng.gen_range::<f32>(-0.5, 0.5),
                    self.rng.gen_range::<f32>(-0.5, 0.5),
                );
            }

            let mut data = &mut self.particles[i];

            // Update particle position
            let delta = self.field_provider.delta(data.position);
            let speed_multiplier = 0.02 * state.speed_multiplier;
            data.position.0 += delta.0 * speed_multiplier;
            data.position.1 += delta.1 * speed_multiplier;
            data.position.2 += delta.2 * speed_multiplier;

            let dist = (delta.0 * delta.0 + delta.1 * delta.1 + delta.2 * delta.2).sqrt();
            if dist.is_nan() {
                data.lifetime = 500.0;
                continue;
            }

            // High-pass filter
            if dist < self.max_dist * state.highpass_filter {
                data.lifetime = 500.0;
                continue;
            }
            // Low-pass filter
            if dist > self.max_dist * state.lowpass_filter {
                data.lifetime = 500.0;
                continue;
            }

            // Send the data to the GPU.
            self.particle_data[self.alive_count * 4] = data.position.0;
            self.particle_data[self.alive_count * 4 + 1] = data.position.1;
            self.particle_data[self.alive_count * 4 + 2] = data.position.2;
            self.particle_data[self.alive_count * 4 + 3] = dist * 4.0;

            // Update lifetime and alive count.
            data.lifetime += 1.0;
            self.alive_count += 1;
        }
    }

    /// Draw the particles to the screen using the provided (camera)
    /// projection matrix.
    pub fn draw(&mut self, projection_matrix: &Matrix4<f32>) {
        let context = Context::get_context();
        if self.alive_count > 0 {
            self.particle_data.bind();
            self.particle_data.upload_data(0, self.alive_count * 4, false);
            self.shader.use_program();
            self.shader.bind_attribs();
            context.uniform_matrix_4fv(&self.mvp_uniform, 1, false, &projection_matrix);
            context.draw_arrays(Context::POINTS, 0, self.alive_count as i32);
        }
    }
}
