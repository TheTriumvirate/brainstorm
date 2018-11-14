#![cfg_attr(feature = "cargo-clippy", allow(cast_lossless))]
extern crate nalgebra as na;
extern crate rand;
#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate serde;

extern crate gl_bindings;

extern crate resources;

extern crate unicode_normalization;

extern crate rusttype;
extern crate window;

pub mod camera;
pub mod graphics;
pub mod gui;
pub mod particles;

use std::path::PathBuf;
use std::f32;
use std::io::Read;

use graphics::{Drawable, Circle, Cube, position, Rectangle};
use gl_bindings::{AbstractContext, Context, shaders::OurShader, VertexBuffer};
use particles::{fieldprovider::FieldProvider, ParticleEngine};
use camera::Camera;
use gui::{Gui};
use window::{AbstractWindow, Window, Event};

use particles::gpu_fieldprovider::GPUFieldProvider;
use particles::gpu_particles::GPUParticleEngine;
use particles::{MarchingCubes, Streamlines};

pub const WINDOW_WIDTH : u32 = 1000;
pub const WINDOW_HEIGHT : u32 = 1000;

/// Holds application resources.
pub struct App {
    camera: camera::ArcBall,
    window: Window,
    time: f32,
    gui: Gui,
    state: State,
    particles: ParticleEngine,
    circle1: Circle,
    circle2: Circle,
    circle3: Circle,
    bound: Cube,
    gpu_field: GPUFieldProvider,
    gpu_particles: GPUParticleEngine,
    march: MarchingCubes,
    streamlines: Streamlines,
    vbo: VertexBuffer
}

/// Holds application state.
pub struct State {
    mouse_x: f64,
    mouse_y: f64,
    is_running: bool,
    highpass_filter: f32,
    lowpass_filter: f32,
    speed_multiplier: f32,
    seeding_size: f32,
    lifetime: f32,
    mesh_transparency: f32,
    particle_size: f32,
    particle_respawn_per_tick: u32,
    texture_idx: f32,
    window_w: f32,
    window_h: f32,
    use_gpu_particles: bool,
}

impl State {
    /// Creates a new State instance with sane defaults.
    pub fn new() -> Self {
        State {
            mouse_x: 0.0,
            mouse_y: 0.0,
            is_running: true,
            highpass_filter: 0.0,
            lowpass_filter: 1.0,
            speed_multiplier: 0.5,
            seeding_size: 1.0,
            lifetime: 100.0,
            mesh_transparency: 0.1,
            particle_size: 8.0,
            particle_respawn_per_tick: 1000,
            texture_idx: 0.0,
            window_w: 0.0,
            window_h: 0.0,
            use_gpu_particles: true,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    /// Starts the application.
    /// Expects a file path for non-web compile targets.
    pub fn new(path: Option<PathBuf>) -> App {
        #[allow(unused_assignments)]
        let mut field_provider = None;
        #[allow(unused_assignments)]
        let mut gpu_field = None;
        let window = Window::new("Brainstorm!", WINDOW_WIDTH, WINDOW_HEIGHT);

        // For web we embed the data in the executable.
        #[cfg(target_arch = "wasm32")]
        {
            stdweb::initialize();
            field_provider = Some(FieldProvider::new(resources::fields::TEST_DATA));
            gpu_field = Some(GPUFieldProvider::new(resources::fields::TEST_DATA))
        }
        // For desktop we load a file.
        #[cfg(not(target_arch = "wasm32"))]
        {
            let path = path.expect("No file specified!");
            let mut file = std::fs::File::open(path).expect("Failed to open file!");
            let mut content = Vec::new();
            file.read_to_end(&mut content).expect("Failed to read file!");
            field_provider = Some(FieldProvider::new(&content));
            gpu_field = Some(GPUFieldProvider::new(&content))
        }

        let field_provider = field_provider.unwrap();
        let march = MarchingCubes::marching_cubes(&field_provider);
        let particles = Some(ParticleEngine::new(field_provider));
        
        let streamlines = Streamlines::new();
        let gpu_particles = GPUParticleEngine::new();

        let vbo = VertexBuffer::new();
        vbo.bind();

        App {
            window,
            particles: particles.unwrap(),
            camera: camera::ArcBall::new(),
            time: 0.0,
            gui: Gui::new((1000.0, 1000.0)),
            state: State::new(),
            circle1: Circle::new(0.0,0.0,0.0,0.5, 0.0, (1.0, 0.0, 0.0), false),
            circle2: Circle::new(0.0,0.0,0.0,0.5, 0.0, (0.0, 1.0, 0.0), false),
            circle3: Circle::new(0.0,0.0,0.0,0.5, 0.0, (0.0, 0.0, 1.0), false),
            bound: Cube::new((-0.5, -0.5, -0.5), (1.0,1.0,1.0), (1.0,1.0,1.0)),
            gpu_field: gpu_field.unwrap(),
            gpu_particles: gpu_particles,
            march,
            streamlines,
            vbo,
        }
    }

    /// Runs the application for one frame.
    pub fn run(&mut self) -> bool {
        let context = Context::get_context();

        // Handle events
        for event in &self.window.get_events() {
            match event {
                Event::Resized(w, h) => {
                    self.state.window_w = *w;
                    self.state.window_h = *h;
                    self.window.set_size(*w as u32, *h as u32)
                },
                _ => {}
            };
            let consumed = self.gui
                .handle_event(&event, &mut self.state, self.window.get_size());

            if !consumed {
                self.camera.handle_events(&event);
            }
        }

        // Update camera and particle system
        self.camera.update();
        
        let (cx, cy, cz) = self.camera.get_position();
        self.march.set_light_dir((cx, cy, cz));

        let radius = self.state.seeding_size * 0.6 + 0.01;
        let speed_multiplier = 0.016 * self.state.speed_multiplier;
        //self.streamlines.draw_streamlines(speed_multiplier, self.state.lifetime as i32, radius, &self.field_provider, self.camera.get_target());

        // Clear screen
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(Context::COLOR_BUFFER_BIT);
        context.clear(Context::DEPTH_BUFFER_BIT);

        // Draw everything
        Context::get_context().enable(Context::DEPTH_TEST);
        let projection_matrix = self.camera.get_projection_matrix();

        self.circle1.set_color(1.0, 0.0, 0.0);
        self.circle2.set_color(0.0, 1.0, 0.0);
        self.circle3.set_color(0.0, 0.0, 1.0);
        self.circle1.set_radius(self.state.seeding_size * 0.6 + 0.01);
        self.circle2.set_radius(self.state.seeding_size * 0.6 + 0.01);
        self.circle3.set_radius(self.state.seeding_size * 0.6 + 0.01);
        self.circle1.set_center(self.camera.get_target());
        self.circle2.set_center(self.camera.get_target());
        self.circle3.set_center(self.camera.get_target());
        self.circle1.rebuild_data();
        self.circle2.rebuild_data();
        self.circle3.rebuild_data();
        self.bound.draw_transformed(&projection_matrix);
        self.circle1.draw_transformed(&projection_matrix);
        self.circle2.draw_transformed(&projection_matrix);
        self.circle3.draw_transformed(&projection_matrix);


        if self.state.use_gpu_particles {
            Context::get_context().disable(Context::DEPTH_TEST);
            self.gpu_particles.update(&self.gpu_field, &self.state, &self.camera);
            Context::get_context().enable(Context::DEPTH_TEST);
            self.gpu_particles.draw_transformed(&projection_matrix);
        } else {
            self.particles.update(&self.state, &mut self.camera);
            self.particles.draw(&projection_matrix, &self.state);
        }
        
        if self.state.mesh_transparency < 1.0 {
            Context::get_context().depth_mask(false);
        }

        self.march.set_transparency(self.state.mesh_transparency);
        self.march.draw_transformed(&projection_matrix);
        
        if self.state.mesh_transparency < 1.0 {
            Context::get_context().depth_mask(true);
        }

        Context::get_context().disable(Context::DEPTH_TEST);

        //self.streamlines.draw_transformed(projection_matrix);
        
        self.gui.draw();

        self.window.swap_buffers();
        self.time += 0.01;
        self.state.is_running

    }
}

