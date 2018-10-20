use na::Matrix4;
use rand::rngs::SmallRng;
use rand::{FromEntropy, Rng};

use std::{f32, str};

use gl_bindings::{shaders, AbstractContext, Buffer, BufferType, Context, UniformLocation};
use particles::fieldprovider::FieldProvider;
use State;

use camera::{Camera, ArcBall};

use resources::shaders::{PARTICLES_VERTEX_SHADER, PARTICLES_FRAGMENT_SHADER};
use graphics::Drawable;

use particles::MarchingCubes;
use particles::Streamlines;

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
    field_provider: FieldProvider,
    rng: SmallRng,
    mvp_uniform: UniformLocation,
    shader: shaders::OurShader,
    alive_count: usize,
    max_dist: f32,
    max_camera_dist: f32,
    min_camera_dist: f32,
    march: MarchingCubes,
    streamlines: Streamlines,
}

impl ParticleEngine {
    /// Initializes a new particle engine.
    pub fn new(field_provider: FieldProvider) -> Self {
        let mut rng = SmallRng::from_entropy();

        // Set up particles.
        let mut data: Buffer<f32> = Buffer::new(BufferType::Array);
        data.resize(PARTICLE_COUNT * 3, 0.0);
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
            str::from_utf8(PARTICLES_VERTEX_SHADER).expect("Failed to read vertex shader");
        let fragment_shader =
            str::from_utf8(PARTICLES_FRAGMENT_SHADER).expect("Failed to read fragment shader");

        let mut attributes = Vec::new();
        attributes.push(shaders::ShaderAttribute {
            name: "position".to_string(),
            size: 3,
        });

        let shader = shaders::OurShader::new(vertex_shader, fragment_shader, &attributes);
        shader.use_program();

        let mvp_uniform = shader.get_uniform_location();

        // Find the max velocity to be used with the high-pass filter later.
        let mut max_dist: f32 = 0.0;
        for (dx,dy,dz,fa) in field_provider.data() {
            let dist = ((dx*fa).powf(2.0) + (dy*fa).powf(2.0) + (dz*fa).powf(2.0)).sqrt();
            max_dist = max_dist.max(dist);
        }
        
        let march = MarchingCubes::marching_cubes(&field_provider);
        let streamlines = Streamlines::new();

        ParticleEngine {
            particles,
            particle_data: data,
            field_provider,
            rng,
            shader,
            mvp_uniform,
            alive_count: 0,
            max_dist,
            max_camera_dist: 0.0,
            min_camera_dist: 0.0,
            march,
            streamlines,
        }
    }

    /// Update the particle system, advancing 1 tick.
    /// Uses settings from `state` to let the user interface with the system.
    pub fn update(&mut self, state: &State, camera: &ArcBall) {
        self.alive_count = 0;
        let (cx, cy, cz) = camera.get_position();
        let (tx, ty, tz) = camera.get_target();

        self.march.set_light_dir((cx, cy, cz));

        self.max_camera_dist = 0.0;
        self.min_camera_dist = f32::MAX;
        let radius = state.seeding_size * 0.6 + 0.01;

        let speed_multiplier = 0.016 * state.speed_multiplier;

        self.streamlines.draw_streamlines(speed_multiplier, state.lifetime as i32, radius, &self.field_provider, camera.get_target());
        
        let mut respawned = 0;
        
        for i in 0..PARTICLE_COUNT {

            let mut data = &mut self.particles[i];
            // Respawn particle if it's too old.
            if data.lifetime > state.lifetime {
                data.lifetime = 500.0;
                if respawned > state.particle_respawn_per_tick {continue;}
                data.lifetime = 0.0;
                let mut dx = self.rng.gen_range::<f32>(-1.0, 1.0);
                let mut dy = self.rng.gen_range::<f32>(-1.0, 1.0);
                let mut dz = self.rng.gen_range::<f32>(-1.0, 1.0);
                let dist = self.rng.gen_range::<f32>(0.0, radius*radius).sqrt();
                let dt = (dx * dx + dy * dy + dz * dz).sqrt();
                dx = dx / dt;
                dy = dy / dt;
                dz = dz / dt;
                data.position = (
                    dx * dist + tx,
                    dy * dist + ty,
                    dz * dist + tz,
                );
                respawned += 1;
            }

            // Update particle position
            let (dx,dy,dz,fa) = self.field_provider.delta(data.position);
            let (dx,dy,dz) = (fa*dx,fa*dy,fa*dz);
            data.position.0 += dx * speed_multiplier;
            data.position.1 += dy * speed_multiplier;
            data.position.2 += dz * speed_multiplier;

            let dist = (dx*dx+dy*dy+dz*dz).sqrt();
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

            let (dx, dy, dz) = (
                cx - data.position.0,
                cy - data.position.1,
                cz - data.position.2,
            );
            self.max_camera_dist = self
                .max_camera_dist
                .max((dx * dx + dy * dy + dz * dz).sqrt());
            self.min_camera_dist = self
                .min_camera_dist
                .min((dx * dx + dy * dy + dz * dz).sqrt());

            // Send the data to the GPU.
            self.particle_data[self.alive_count * 3] = data.position.0;
            self.particle_data[self.alive_count * 3 + 1] = data.position.1;
            self.particle_data[self.alive_count * 3 + 2] = data.position.2;

            // Update lifetime and alive count.
            data.lifetime += 1.0;
            self.alive_count += 1;
        }
    }

    /// Draw the particles to the screen using the provided (camera)
    /// projection matrix.
    pub fn draw(&mut self, projection_matrix: &Matrix4<f32>, state: &State) {
        let context = Context::get_context();
        if self.alive_count > 0 {
            self.particle_data.bind();
            self.particle_data
                .upload_data(0, self.alive_count * 3, false);
            self.shader.use_program();
            self.shader.uniform1f("min_dist", self.min_camera_dist);
            self.shader.uniform1f("max_dist", self.max_camera_dist);
            self.shader.uniform1f("transparency", 0.5);
            self.shader.uniform1f("part_size", state.particle_size);
            self.shader.bind_attribs();
            context.uniform_matrix_4fv(&self.mvp_uniform, 1, false, &projection_matrix);
            context.draw_arrays(Context::POINTS, 0, self.alive_count as i32);
            self.shader.unbind_attribs();

        }
        Context::get_context().depth_mask(false);
        self.march.set_transparency(state.mesh_transparency);
        self.march.draw_transformed(projection_matrix);
        Context::get_context().depth_mask(true);
        Context::get_context().disable(Context::DEPTH_TEST);        
        self.streamlines.draw_transformed(projection_matrix);
        Context::get_context().enable(Context::DEPTH_TEST);

    }
}
