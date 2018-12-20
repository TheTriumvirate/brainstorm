use gl_bindings::{
    shaders::OurShader, shaders::ShaderAttribute, AbstractContext, Buffer, BufferType, Context,
    FrameBuffer, Texture, TextureFormat,
};
use crate::graphics::{render_target, DrawMode, Drawable};
use resources::shaders::{
    GPU_PARTICLES_FRAGMENT_SHADER, GPU_PARTICLES_UPDATE_FRAGMENT_SHADER,
    GPU_PARTICLES_UPDATE_VERTEX_SHADER, GPU_PARTICLES_VERTEX_SHADER,
};

use crate::particles::gpu_fieldprovider::GPUFieldProvider;

use na::Matrix4;
use std::rc::Rc;
use std::str;

use crate::camera::ArcBall;
use crate::State;

use rand::rngs::SmallRng;
use rand::{FromEntropy, Rng};

const MAXSTREAMLETSIZE: usize = 4;

pub struct GPUParticleEngine {
    texture: Rc<Texture>,
    texture2: Rc<Texture>,
    noise: Rc<Texture>,
    vertices: Buffer<f32>,
    indices: Buffer<u32>,
    shader: Rc<OurShader>,
    update_shader: Rc<OurShader>,
    framebuffer: FrameBuffer,
    layer: i32,
    timer: f32,
    update: bool,
    swap: bool,
    texture_size: usize,
}

impl GPUParticleEngine {
    pub fn new(gpu_particle_count: usize) -> Self {
        let texture_size = gpu_particle_count;
        let mut data = Vec::new();
        let mut particle_data = Vec::new();
        let mut noise_data = Vec::new();
        let mut rng = SmallRng::from_entropy();
        let mut index_data = Vec::new();
        let mut index = 0;
        let offset = (texture_size * texture_size) as u32;
        for q in 0..MAXSTREAMLETSIZE {
            for u in 0..(texture_size) {
                for v in 0..(texture_size) {
                    data.push(0.0);
                    data.push(0.0);
                    data.push(0.0);
                    data.push(255.0);

                    particle_data.push(u as f32 / (texture_size as f32) + 0.5 / texture_size as f32);
                    particle_data.push(v as f32 / (texture_size as f32) + 0.5 / texture_size as f32);

                    if q == 0 {
                        noise_data.push(rng.gen_range(0.0, 1.0));
                        noise_data.push(rng.gen_range(0.0, 1.0));
                        noise_data.push(rng.gen_range(0.0, 1.0));
                        noise_data.push(rng.gen_range(0.0, 1.0));
                    } else {
                        index_data.push(index - offset);
                        index_data.push(index);
                    }
                    index += 1;
                }
            }
        }

        let shader: OurShader = OurShader::new(
            str::from_utf8(GPU_PARTICLES_VERTEX_SHADER).expect("Failed to read vertex shader"),
            str::from_utf8(GPU_PARTICLES_FRAGMENT_SHADER).expect("Failed to read fragment shader"),
            &[ShaderAttribute {
                name: "v_texpos".to_string(),
                size: 2,
            }],
        );

        let update_shader: OurShader = OurShader::new(
            str::from_utf8(GPU_PARTICLES_UPDATE_VERTEX_SHADER)
                .expect("Failed to read vertex shader"),
            str::from_utf8(GPU_PARTICLES_UPDATE_FRAGMENT_SHADER)
                .expect("Failed to read fragment shader"),
            &[ShaderAttribute {
                name: "v_texpos".to_string(),
                size: 2,
            }],
        );

        let mut vertices: Buffer<f32> = Buffer::new(BufferType::Array);
        vertices.set_data(&particle_data[..]);

        vertices.bind();
        let len = vertices.len();
        vertices.upload_data(0, len, true);

        let mut indices: Buffer<u32> = Buffer::new(BufferType::IndexArray);
        indices.set_data(&index_data[..]);

        indices.bind();
        let len = indices.len();
        indices.upload_data(0, len, true);

        let texture = Rc::new(Texture::from_3d_data_f(
            texture_size as u32,
            texture_size as u32,
            MAXSTREAMLETSIZE as u32,
            TextureFormat::RGBA,
            &data[..],
            false,
        ));
        let texture2 = Rc::new(Texture::from_3d_data_f(
            texture_size as u32,
            texture_size as u32,
            MAXSTREAMLETSIZE as u32,
            TextureFormat::RGBA,
            &data[..],
            false,
        ));

        let framebuffer = FrameBuffer::new();

        shader.uniform1f("u_size", texture_size as f32);
        shader.uniform1i("u_layer", 0);

        GPUParticleEngine {
            texture,
            texture2,
            noise: Rc::new(Texture::from_data(
                texture_size as u32,
                texture_size as u32,
                TextureFormat::RGBA,
                &noise_data[..],
            )),
            vertices,
            indices,
            shader: Rc::new(shader),
            update_shader: Rc::new(update_shader),
            framebuffer,
            layer: 0,
            timer: 0.0,
            update: false,
            swap: false,
            texture_size,
        }
    }

    pub fn update(&mut self, field_provider: &GPUFieldProvider, state: &State, camera: &ArcBall) {
        self.timer += 0.004;
        let context = Context::get_context();
        //if self.timer < 1.0 {
        //    return;
        //}
        self.update_shader.uniform1f("u_size", self.texture_size as f32);
        self.update_shader
            .uniform1f("u_speed", state.speed_multiplier * 0.016);
        self.update_shader
            .uniform1f("u_lowpass", state.lowpass_filter);
        self.update_shader
            .uniform1f("u_highpass", state.highpass_filter);
        self.update_shader
            .uniform1f("u_seedsize", state.seeding_size * 0.6 + 0.01);

        let (cx, cy, cz) = camera.get_target();
        self.update_shader
            .uniform3f("u_seedpos", cx + 0.5, cy + 0.5, cz + 0.5);

        self.update_shader.uniform1i("u_layer", (self.layer) as i32);
        self.layer = (self.layer + 1) % MAXSTREAMLETSIZE as i32;

        self.shader.uniform1i("u_layer", (self.layer) as i32);

        self.timer = 0.0;
        self.update = true;
        Context::get_context().viewport(0, 0, self.texture_size as i32, self.texture_size as i32);
        let len = self.vertices.len() as i32 / 2 / (MAXSTREAMLETSIZE) as i32;
        self.get_texture()
            .unwrap()
            .activate(Some(&self.update_shader), 0, "uSampler");
        field_provider
            .get_texture()
            .activate(Some(&self.update_shader), 1, "uData");
        self.noise.activate(Some(&self.update_shader), 2, "uNoise");
        self.framebuffer.bind();
        self.update_texture().bind();
        self.framebuffer
            .buffer_texture_layer(&self.update_texture(), self.layer);

        self.vertices.bind();
        self.update_shader.use_program();
        self.update_shader.bind_attribs();

        context.draw_arrays(Context::POINTS, 0, len);

        //render_target::draw_vertex_array(DrawMode::POINTS, 0, len, &self.vertices, self.render_states(), &Matrix4::<f32>::identity());
        self.framebuffer.unbind();
        // TODO: Resize to window size...
        Context::get_context().viewport(0, 0, state.window_w as i32, state.window_h as i32);
        self.update = false;
        self.swap = !self.swap;
    }

    pub fn update_texture(&self) -> Rc<Texture> {
        if !self.swap {
            self.texture.clone()
        } else {
            self.texture2.clone()
        }
    }
}

impl Drawable for GPUParticleEngine {
    fn get_texture(&self) -> Option<Rc<Texture>> {
        if self.swap {
            Some(self.texture.clone())
        } else {
            Some(self.texture2.clone())
        }
    }

    fn get_shader(&self) -> Option<Rc<OurShader>> {
        if self.update {
            Some(self.update_shader.clone())
        } else {
            Some(self.shader.clone())
        }
    }

    fn draw_transformed(&self, view_matrix: &Matrix4<f32>) {
        self.update_texture()
            .activate(Some(&self.shader), 1, "uOther");
        render_target::draw_indices(
            DrawMode::LINES,
            &self.vertices,
            &self.indices,
            &self.render_states(),
            view_matrix,
        );
        //let len = self.vertices.len() as i32 / 2 / (MAXSTREAMLETSIZE) as i32;
        //render_target::draw_vertex_array(DrawMode::POINTS, 0, len, &self.vertices, self.render_states(), view_matrix);
    }
}
