use na::Matrix4;
use rand::rngs::SmallRng;
use rand::{FromEntropy, Rng};

use std::str;

use gl_context::{shaders, AbstractContext, Buffer, Context, UniformLocation, VertexArray};
use particles::fieldprovider::{FieldProvider, SphereFieldProvider};

const PARTICLE_COUNT: usize = 100_000;

#[derive(Clone, Debug)]
struct ParticleData {
    position: (f32, f32, f32),
    isAlive: bool,
    lifetime: f32
}

pub struct ParticleEngine {
    particles: Vec<ParticleData>,
    particle_data: Vec<f32>,
    vertex_buffer: Buffer,
    vertex_array: VertexArray,
    field_provider: SphereFieldProvider,
    rng: SmallRng,
    mvp_uniform: UniformLocation,
    shader: shaders::OurShader,
    alive_count: usize,
}

impl ParticleEngine {
    pub fn new() -> Self {
        let context = Context::get_context();
        let mut rng = SmallRng::from_entropy();
        // Set up particles.
        let mut data = Vec::with_capacity(PARTICLE_COUNT * 3);
        data.resize(PARTICLE_COUNT * 3, 0.0);
        let mut particles = Vec::with_capacity(PARTICLE_COUNT);
        for i in 0..PARTICLE_COUNT {
            particles.push(ParticleData {position: (rng.gen_range::<f32>(-0.5, 0.5), rng.gen_range::<f32>(-0.5, 0.5),rng.gen_range::<f32>(-0.5, 0.5)), isAlive: true, lifetime: -(i as f32 / 100.0)});
        }
        //particles.resize(PARTICLE_COUNT, ParticleData {position: (0.0, 0.0, 0.0), isAlive: true, lifetime: 0.0});

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

        // Set up shaders
        let vertex_shader =
            str::from_utf8(shaders::PARTICLES_VERTEX_SHADER).expect("Failed to read vertex shader");
        let fragment_shader = str::from_utf8(shaders::PARTICLES_FRAGMENT_SHADER)
            .expect("Failed to read fragment shader");
        let shader = shaders::OurShader::new(vertex_shader, fragment_shader, 3, false);

        let mvp_uniform = shader.get_uniform_location();

        ParticleEngine {
            particles,
            particle_data: data,
            vertex_buffer: vb,
            vertex_array: vao,
            field_provider: SphereFieldProvider::new(),
            rng,
            shader,
            mvp_uniform,
            alive_count: 0
        }
    }

    pub fn update(&mut self) {
        let mut end = self.alive_count-1;
        self.alive_count = 0;
        for i in 0..PARTICLE_COUNT {

            while self.particles[i].lifetime > 100.0 && self.particles[i].isAlive && end > i {
                {
                    let mut data = &mut self.particles[i];
                    data.lifetime = 0.0;
                    data.position = (self.rng.gen_range::<f32>(-0.5, 0.5), self.rng.gen_range::<f32>(-0.5, 0.5),self.rng.gen_range::<f32>(-0.5, 0.5));
                }
                self.particles.swap(i, end);
                end -= 1;

            }

            let mut data = &mut self.particles[i];

            if data.isAlive {

                let delta = self.field_provider.delta(data.position);
                data.position.0 += delta.0 * 0.01;
                data.position.1 += delta.1 * 0.01;
                data.position.2 += delta.2 * 0.01;

                self.particle_data[self.alive_count*3] = data.position.0;
                self.particle_data[self.alive_count*3 + 1] = data.position.1;
                self.particle_data[self.alive_count*3 + 2] = data.position.2;

                data.lifetime += 1.0;
                self.alive_count += 1;
            } else {
                break;
            }
            /*if self.rng.gen_bool(0.02) {
                self.particles[i * 3] = self.rng.gen_range::<f32>(-0.5, 0.5);
                self.particles[i * 3 + 1] = self.rng.gen_range::<f32>(-0.5, 0.5);
                self.particles[i * 3 + 2] = self.rng.gen_range::<f32>(-0.5, 0.5);;
            }
            let data = self.field_provider.delta((
                self.particles[i * 3] * 100.0 + 50.0,
                self.particles[i * 3 + 1] * 100.0 + 50.0,
                self.particles[i * 3 + 2] * 100.0 + 50.0,
            ));
            self.particles[i * 3] += data.0 * 0.001;
            self.particles[i * 3 + 1] += data.1 * 0.01;
            self.particles[i * 3 + 2] += data.2 * 0.01;*/
        }
    }

    pub fn draw(&mut self, projection_matrix: &Matrix4<f32>) {
        let context = Context::get_context();
        if self.alive_count > 0 {
            context.bind_buffer(Context::ARRAY_BUFFER, &self.vertex_buffer);
            context.buffer_data(
                Context::ARRAY_BUFFER,
                &self.particle_data[0..self.alive_count * 3],
                Context::DYNAMIC_DRAW,
            );
            context.bind_vertex_array(&self.vertex_array);
            self.shader.set_active();
            context.uniform_matrix_4fv(&self.mvp_uniform, 1, false, &projection_matrix);
            context.draw_arrays(Context::POINTS, 0, self.alive_count as i32);
        }
    }
}
