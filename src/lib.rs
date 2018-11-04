#![cfg_attr(feature = "cargo-clippy", allow(cast_lossless))]

#[cfg(target_arch = "wasm32")]
extern crate base64;
#[cfg(target_arch = "wasm32")]
#[macro_use] extern crate stdweb;
#[cfg(not(target_arch = "wasm32"))]
extern crate nfd;

extern crate bincode;
extern crate nalgebra as na;
extern crate rand;
extern crate rusttype;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate unicode_normalization;

extern crate gl_bindings;
extern crate resources;
extern crate window;

pub mod camera;
pub mod graphics;
pub mod gui;
pub mod particles;

use std::path::PathBuf;
use std::f32;
use std::io::Read;

use gl_bindings::{AbstractContext, Context};
use graphics::{Drawable, Circle, Cube};
use particles::{fieldprovider::FieldProvider, ParticleEngine};
use camera::Camera;
use gui::{Gui};
use window::{AbstractWindow, Window, Event};

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
    show_streamlines: bool,
    file_path: Option<PathBuf>,
    reload_file: bool,
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
            show_streamlines: true,
            file_path: None,
            reload_file: false
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
        let mut particles = None;
        let window = Window::new("Brainstorm!", WINDOW_WIDTH, WINDOW_HEIGHT);

        // For web we embed the data in the executable.
        #[cfg(target_arch = "wasm32")]
        {
            stdweb::initialize();
            let field_provider = FieldProvider::new(resources::fields::TEST_DATA);
            particles = Some(ParticleEngine::new(field_provider));
        }
        // For desktop we load a file if it exists.
        #[cfg(not(target_arch = "wasm32"))]
        {
            let content: Vec<u8> = if let Some(ref path) = path {
                let mut file = std::fs::File::open(path).expect("Failed to open file!");
                let mut content = Vec::new();
                file.read_to_end(&mut content).expect("Failed to read file!");
                content
            } else {
                Vec::from(resources::fields::DEFAULT_SPIRAL)
            };
            let field_provider = FieldProvider::new(&content).expect("Failed to parse data.");
            particles = Some(ParticleEngine::new(field_provider));
        }
        let mut state = State::new();
        state.file_path = path;
        App {
            window,
            state,
            particles: particles.unwrap(),
            camera: camera::ArcBall::new(),
            time: 0.0,
            gui: Gui::new((1000.0, 1000.0)),
            circle1: Circle::new(0.0,0.0,0.0,0.5, 0.0, (1.0, 0.0, 0.0), false),
            circle2: Circle::new(0.0,0.0,0.0,0.5, 0.0, (0.0, 1.0, 0.0), false),
            circle3: Circle::new(0.0,0.0,0.0,0.5, 0.0, (0.0, 0.0, 1.0), false),
            bound: Cube::new((-0.5, -0.5, -0.5), (1.0,1.0,1.0), (1.0,1.0,1.0)),
        }
    }

    /// Runs the application for one frame.
    pub fn run(&mut self) -> bool {
        let context = Context::get_context();

        // Handle events
        for event in &self.window.get_events() {
            match event {
                Event::Resized(w, h) => self.window.set_size(*w as u32, *h as u32),
                _ => {}
            };
            let consumed = self.gui
                .handle_event(&event, &mut self.state, self.window.get_size());

            if !consumed {
                self.camera.handle_events(&event);
            }
        }

        // Replace particle data if requested.
        #[cfg(target_arch = "wasm32")]
        {
            let updated = match js!(return isUpdated();) {
                stdweb::Value::Bool(b) => b,
                a => panic!("Unknown isUpdated return type"),
            };
            if updated {
                match js!(return getData();).into_string() {
                    Some(b64) => {
                        let pos = b64.find(",").map(|i| i + 1).unwrap_or(0);
                        let b64 = b64.split_at(pos).1;
                        match base64::decode(b64) {
                            Ok(data) => {
                                let field_provider = FieldProvider::new(&data);
                                self.particles = ParticleEngine::new(field_provider);
                            }
                            Err(e) =>
                                self.gui.status.set_status("Failed to decode base64 content.".to_owned()),
                        }
                    }
                    None => self.gui.status.set_status("Failed to get string from JS.".to_owned()),
                }
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            if self.state.reload_file {
                self.state.reload_file = false;
                match self.reload_file() {
                    Ok(particle_engine) => self.particles = particle_engine,
                    Err(e) => self.gui.status.set_status(format!("Failed to load file: {}", e)),
                }
            }
        }

        // Update camera and particle system
        self.camera.update();
        self.particles.update(&self.state, &mut self.camera);

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

        self.particles.draw(&projection_matrix, &self.state);
        Context::get_context().disable(Context::DEPTH_TEST);
        self.gui.draw();

        self.window.swap_buffers();
        self.time += 0.01;

        self.state.is_running
    }

    fn reload_file(&self) -> Result<ParticleEngine, &'static str> {
        let path = self.state.file_path.as_ref().ok_or("No file path saved.")?;
        let mut file = std::fs::File::open(path).map_err(|_| "Failed to open file.")?;
        let mut content = Vec::new();
        file.read_to_end(&mut content).map_err(|_| "Failed to read file.")?;
        let field_provider = FieldProvider::new(&content).map_err(|_| "Failed to parse file.")?;
        return Ok(ParticleEngine::new(field_provider));
    }
}
