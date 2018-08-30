#![cfg_attr(feature = "cargo-clippy", allow(cast_lossless))]
extern crate noise;

#[cfg(target_arch = "wasm32")]
extern crate stdweb;
#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb_derive;
#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate serde_derive;
#[cfg(target_arch = "wasm32")]
extern crate serde;

extern crate nalgebra as na;
extern crate rand;

extern crate gl_context;

extern crate lazy_static;

pub mod camera;
pub mod particles;
pub mod window;
pub mod ui;

use gl_context::AbstractContext;
use gl_context::Context;

use rand::rngs::SmallRng;
use rand::{FromEntropy, Rng};
use std::{f32, str};

use camera::*;
use gl_context::shaders::OurShader;
use particles::fieldprovider::{FieldProvider, SphereFieldProvider};
use ui::*;
use window::*;

const PARTICLE_COUNT: usize = 100_000;

pub struct App {
    camera: ArcBallCamera,
    window: Window,
    particles: Vec<f32>,
    time: f32,
    rng: SmallRng,
    mvp_uniform: gl_context::UniformLocation,
    shaders: OurShader,
    field_provider: SphereFieldProvider,
    gui: Gui,
    state: State,
    vb: gl_context::VertexArray,
    vao: gl_context::VertexArray,
}

pub struct State {
    mouse_x: f64,
    mouse_y: f64,
    is_running: bool,
}

impl State {
    pub fn new() -> Self {
        State {
            mouse_x: 0.0,
            mouse_y: 0.0,
            is_running: true,
        }
    }
}

impl App {
    pub fn new() -> App {
        // Set up window
        let window = Window::new("Particles!", 1000, 1000);

        let mut rng = SmallRng::from_entropy();
        // Set up particles.
        let mut data = Vec::new();
        for _ in 0..PARTICLE_COUNT {
            data.push(rng.gen_range::<f32>(-0.5, 0.5));
            data.push(rng.gen_range::<f32>(-0.5, 0.5));
            data.push(rng.gen_range::<f32>(-0.5, 0.5));
        }

        let context = Context::get_context();

        // Bind the data buffer.
        let vb = context
            .create_buffer()
            .expect("Failed to create data buffer.");
        context.bind_buffer(Context::ARRAY_BUFFER, &vb);
        context.buffer_data(Context::ARRAY_BUFFER, &data, Context::DYNAMIC_DRAW);

        // Bind the vertex array.
        let vao = context
            .create_vertex_array()
            .expect("Failed to create vertex array.");
        context.bind_vertex_array(&vao);

        // Set up shaders
        let vertex_shader = str::from_utf8(gl_context::shaders::PARTICLES_VERTEX_SHADER)
            .expect("Failed to read vertex shader");
        let fragment_shader = str::from_utf8(gl_context::shaders::PARTICLES_FRAGMENT_SHADER)
            .expect("Failed to read fragment shader");
        let shaders = gl_context::shaders::OurShader::new(vertex_shader, fragment_shader, 3);

        let mvp_uniform = shaders.get_uniform_location();

        // Run main loop.
        let time: f32 = 0.0;

        // Define initial state
        let state = State::new();

        // Add test button
        let mut gui = Gui::new();
        gui.buttons.push(Button {
            x1: 400.0,
            x2: 600.0,
            y1: 400.0,
            y2: 600.0,
            color: (1.0, 1.0, 1.0),
            func: Box::new(|ref mut _context| {
                println!("Hello, SPACE!");
            }),
        });

        App {
            window,
            camera: ArcBallCamera::new(),
            particles: data,
            time,
            rng,
            mvp_uniform,
            shaders,
            field_provider: SphereFieldProvider::new(),
            gui,
            state,
            vb,
            vao,
        }
    }

    pub fn run(&mut self) -> bool{
        self.update()
    }

    fn update(&mut self) -> bool {
        let context = Context::get_context();
        for event in self.window.get_events().iter() {
            self.gui.handle_event(&event, &mut self.state);
            self.camera.handle_events(&event);
        }

        for i in 0..PARTICLE_COUNT {
            if self.rng.gen_bool(0.02) {
                self.particles[i * 3] = self.rng.gen_range::<f32>(-0.5, 0.5);
                self.particles[i * 3 + 1] = self.rng.gen_range::<f32>(-0.5, 0.5);
                self.particles[i * 3 + 2] = self.rng.gen_range::<f32>(-0.5, 0.5);;
            }
            let (dx, dy, dz) = self.field_provider.delta((
                self.particles[i * 3] * 100.0 + 50.0,
                self.particles[i * 3 + 1] * 100.0 + 50.0,
                self.particles[i * 3 + 2] * 100.0 + 50.0,
            ));
            self.particles[i * 3] += dx * 0.001;
            self.particles[i * 3 + 1] += dy * 0.01;
            self.particles[i * 3 + 2] += dz * 0.01;
        }

        // Clear screen
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(Context::COLOR_BUFFER_BIT);

        // Render particles
        context.bind_buffer(Context::ARRAY_BUFFER, &self.vb);
        context.buffer_data(
            Context::ARRAY_BUFFER,
            &self.particles,
            Context::DYNAMIC_DRAW,
        );
        context.bind_vertex_array(&self.vao);
        self.shaders.set_active();
        let projection_matrix = self.camera.get_projection_matrix();
        context.uniform_matrix_4fv(&self.mvp_uniform, 1, false, &projection_matrix);
        context.draw_arrays(Context::POINTS, 0, PARTICLE_COUNT as i32);

        // Render GUI
        self.gui.draw();

        self.window.swap_buffers();
        self.time += 0.01;

        self.state.is_running
    }
}
