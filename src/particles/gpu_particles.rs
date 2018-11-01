use gl_bindings::{Texture, TextureFormat, Buffer, shaders::OurShader, shaders::ShaderAttribute, BufferType, FrameBuffer, Context, AbstractContext};
use graphics::{Drawable, render_target, DrawMode};
use resources::shaders::{GPU_PARTICLES_VERTEX_SHADER, GPU_PARTICLES_FRAGMENT_SHADER, GPU_PARTICLES_UPDATE_VERTEX_SHADER, GPU_PARTICLES_UPDATE_FRAGMENT_SHADER};

use particles::gpu_fieldprovider::GPUFieldProvider;

use std::str;
use std::rc::Rc;
use na::Matrix4;

use State;
use camera::{Camera, ArcBall};

use rand::{FromEntropy, Rng};
use rand::rngs::SmallRng;

const TEXTURESIZE: usize = 512;
const MAXSTREAMLETSIZE: usize = 16;

pub struct GPUParticleEngine {
    texture: Rc<Texture>,
    noise: Rc<Texture>,
    vertices: Buffer<f32>,
    shader: Rc<OurShader>,
    update_shader: Rc<OurShader>,
    framebuffer: FrameBuffer,
    layer: i32,
    timer: f32,
    update: bool,
}

impl GPUParticleEngine {
    pub fn new() -> Self {
        let mut data = Vec::new();
        let mut particle_data = Vec::new();
        let mut noise_data = Vec::new();
        let mut rng = SmallRng::from_entropy();
        for q in 0..MAXSTREAMLETSIZE {
            for u in 0..(TEXTURESIZE) {
                for v in 0..(TEXTURESIZE) {
                    data.push(rng.gen_range::<f32>(0.0, 1.0));
                    data.push(rng.gen_range::<f32>(0.0, 1.0));
                    data.push(rng.gen_range::<f32>(0.0, 1.0));
                    //data.push(u as f32 / TEXTURESIZE as f32);
                    //data.push(v as f32 / TEXTURESIZE as f32);
                    //data.push(v as f32 / TEXTURESIZE as f32);
                    data.push(1.0);
                    if q == 0 {
                        particle_data.push(u as f32 / (TEXTURESIZE as f32) + 0.5 / TEXTURESIZE as f32);
                        particle_data.push(v as f32 / (TEXTURESIZE as f32) + 0.5 / TEXTURESIZE as f32);
                        noise_data.push(rng.gen_range::<f32>(0.0, 1.0));
                        noise_data.push(rng.gen_range::<f32>(0.0, 1.0));
                        noise_data.push(rng.gen_range::<f32>(0.0, 1.0));
                        noise_data.push(rng.gen_range::<f32>(0.0, 1.0));
                    }
                }
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
        let update_shader: OurShader = OurShader::new(
            str::from_utf8(GPU_PARTICLES_UPDATE_VERTEX_SHADER).expect("Failed to read vertex shader"),
            str::from_utf8(GPU_PARTICLES_UPDATE_FRAGMENT_SHADER).expect("Failed to read fragment shader"),
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

        let texture = Rc::new(Texture::from_3d_data(TEXTURESIZE as u32, TEXTURESIZE as u32, MAXSTREAMLETSIZE as u32, TextureFormat::RGBA, &data[..]));

        let framebuffer = FrameBuffer::new();
        framebuffer.buffer_texture_layer(&texture, 0);

        GPUParticleEngine {
            texture,
            noise: Rc::new(Texture::from_data(TEXTURESIZE as u32, TEXTURESIZE as u32, TextureFormat::RGBA, &noise_data[..])),
            vertices,
            shader: Rc::new(shader),
            update_shader: Rc::new(update_shader),
            framebuffer,
            layer: 0,
            timer: 0.0,
            update: false
        }
    }

    pub fn update(&mut self, field_provider: &GPUFieldProvider, state: &State, camera: &ArcBall) {
        self.timer += 0.1;
        //if self.timer < 1.0 {
        //    return;
        //}
        self.update_shader.uniform1f("u_size", TEXTURESIZE as f32);
        self.shader.uniform1f("u_size", TEXTURESIZE as f32);

        self.update_shader.uniform1f("u_speed", state.speed_multiplier * 0.016);
        self.update_shader.uniform1f("u_lowpass", state.highpass_filter);
        self.update_shader.uniform1f("u_highpass", state.lowpass_filter);
        self.update_shader.uniform1f("u_seedsize", state.seeding_size * 0.6 + 0.01);

        let (cx, cy, cz) = camera.get_target();
        self.update_shader.uniform3f("u_seedpos", cx / 2.0 + 0.5, cy / 2.0 + 0.5, cz / 2.0 + 0.5);

        self.update_shader.uniform1i("u_layer", (self.layer) as i32);
        self.layer = (self.layer + 1) % MAXSTREAMLETSIZE as i32;
        self.framebuffer.buffer_texture_layer(&self.texture, self.layer);
        self.shader.uniform1i("u_layer", (self.layer) as i32);
        
        self.timer = 0.0;
        self.update = true;
        Context::get_context().viewport(0, 0, TEXTURESIZE as i32, TEXTURESIZE as i32);
        let len = self.vertices.len() as i32 / 2;
        self.framebuffer.bind();
        field_provider.get(0).activate(Some(&self.update_shader), 1, "uData");
        self.noise.activate(Some(&self.update_shader), 2, "uNoise");
        render_target::draw_vertex_array(DrawMode::POINTS, 0, len, &self.vertices, self.render_states(), &Matrix4::<f32>::identity());
        self.framebuffer.unbind();
        // TODO: Resize to window size...
        Context::get_context().viewport(0, 0, state.window_w as i32, state.window_h as i32);
        self.update = false;
    }

    pub fn get_layer(&self) -> i32 {
        self.layer
    }
}

impl Drawable for GPUParticleEngine {
    fn get_texture(&self) -> Option<Rc<Texture>> {
        Some(self.texture.clone())
    }

    fn get_shader(&self) -> Option<Rc<OurShader>> {
        if self.update {
            Some(self.update_shader.clone())
        }
        else {
            Some(self.shader.clone())
        }
    }

    fn draw_transformed(&self, view_matrix: &Matrix4<f32>) {
        let len = self.vertices.len() as i32 / 2;
        render_target::draw_vertex_array(DrawMode::POINTS, 0, len, &self.vertices, self.render_states(), view_matrix);
    }
}