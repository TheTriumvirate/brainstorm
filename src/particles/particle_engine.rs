use gl_context::Buffer;
use gl_context::VertexArray;
use gl_context::AbstractContext;
use gl_context::Context;

use rand::rngs::SmallRng;
use rand::{FromEntropy, Rng};

use particles::fieldprovider::{SphereFieldProvider, FieldProvider};

const PARTICLE_COUNT: usize = 100_000;

pub struct ParticleEngine {
    particles: Vec<f32>,
    vertex_buffer: Buffer,
    vertex_array: VertexArray,
    field_provider: SphereFieldProvider,
    rng: SmallRng
}

impl ParticleEngine {
    pub fn new() -> Self {
        let context = Context::get_context();
        let mut rng = SmallRng::from_entropy();
        // Set up particles.
        let mut data = Vec::new();
        for _ in 0..PARTICLE_COUNT {
            data.push(rng.gen_range::<f32>(-0.5, 0.5));
            data.push(rng.gen_range::<f32>(-0.5, 0.5));
            data.push(rng.gen_range::<f32>(-0.5, 0.5));
        }

        // Bind the window buffer.
        let vb = context
            .create_buffer()
            .expect("Failed to create window buffer.");
        context.bind_buffer(Context::ARRAY_BUFFER, &vb);
        context.buffer_data(Context::ARRAY_BUFFER, &data, Context::DYNAMIC_DRAW);

        // Bind the vertex array.
        let vao = context
            .create_vertex_array()
            .expect("Failed to create vertex array.");
        context.bind_vertex_array(&vao);

        ParticleEngine {
            particles: data,
            vertex_buffer: vb,
            vertex_array: vao,
            field_provider: SphereFieldProvider::new(),
            rng
        }
    }

    pub fn update(&mut self) {
        let context = Context::get_context();
        for i in 0..PARTICLE_COUNT {
            if self.rng.gen_bool(0.02) {
                self.particles[i * 3] = self.rng.gen_range::<f32>(-0.5, 0.5);
                self.particles[i * 3 + 1] = self.rng.gen_range::<f32>(-0.5, 0.5);
                self.particles[i * 3 + 2] = self.rng.gen_range::<f32>(-0.5, 0.5);;
            }
            let (dx, dy, dz) = self.field_provider.delta((self.particles[i * 3] * 100.0 + 50.0, self.particles[i * 3 + 1] * 100.0 + 50.0, self.particles[i * 3 + 2] * 100.0 + 50.0));
            self.particles[i * 3] += dx * 0.001;
            self.particles[i * 3 + 1] += dy * 0.01;
            self.particles[i * 3 + 2] += dz * 0.01;
        }
        context.bind_buffer(Context::ARRAY_BUFFER, &self.vertex_buffer);
        context.buffer_data(Context::ARRAY_BUFFER, &self.particles, Context::DYNAMIC_DRAW);

    }

    pub fn draw(&mut self) {
        let context = Context::get_context();
        context.bind_vertex_array(&self.vertex_array);
        context.draw_arrays(Context::POINTS, 0, PARTICLE_COUNT as i32);

    }
}