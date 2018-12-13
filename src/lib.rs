#![cfg_attr(feature = "cargo-clippy", allow(cast_lossless))]

#[cfg(target_arch = "wasm32")]
extern crate base64;
#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;
#[cfg(not(target_arch = "wasm32"))]
extern crate nfd;

extern crate bincode;
extern crate nalgebra as na;
extern crate rand;
extern crate rusttype;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate unicode_normalization;

extern crate gl_bindings;
extern crate resources;
extern crate window;

pub mod camera;
pub mod file_loading;
pub mod graphics;
pub mod gui;
pub mod particles;

use std::f32;
#[cfg(not(target_arch = "wasm32"))]
use std::io::Read;
use std::path::PathBuf;

use camera::Camera;
use file_loading::FileResult;
use gl_bindings::{AbstractContext, Context};
use graphics::Drawable;
use gui::Gui;
use particles::{fieldprovider::FieldProvider, ParticleEngine};
use window::{AbstractWindow, Event, Window};

const INITIAL_WINDOW_WIDTH: u32 = 1000;
const INITIAL_WINDOW_HEIGHT: u32 = 1000;
use particles::gpu_fieldprovider::GPUFieldProvider;
use particles::gpu_particles::GPUParticleEngine;
use particles::MarchingCubes;

/// Holds application resources.
pub struct App {
    camera: camera::ArcBall,
    window: Window,
    time: f32,
    gui: Gui,
    state: State,
    particles: ParticleEngine,
    mid_reload: bool,
    gpu_field: GPUFieldProvider,
    gpu_particles: GPUParticleEngine,
    march: MarchingCubes,
    gpu_particle_count: usize,
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
    file_path: Option<PathBuf>,
    reload_file: bool,
    camera_target: (f32, f32, f32),
    window_w: f32,
    window_h: f32,
    use_gpu_particles: bool,
    directional_data: Vec<(f32, f32, f32)>,
    options_file: Option<reparser::Options>,
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
            file_path: None,
            reload_file: false,
            camera_target: (0.0, 0.0, 0.0),
            window_w: 0.0,
            window_h: 0.0,
            use_gpu_particles: false,
            directional_data: Vec::new(),
            options_file: None,
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
    pub fn new(path: Option<PathBuf>, start_with_gpu: bool, gpu_particle_count: usize) -> App {
        #[allow(unused_assignments)]
        let mut field_provider = None;
        #[allow(unused_assignments)]
        let mut gpu_field = None;
        let window = Window::new("Brainstorm!", INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT);

        // For web we embed the data in the executable.
        #[cfg(target_arch = "wasm32")]
        {
            stdweb::initialize();
            let vector_field =
                bincode::deserialize(&resources::fields::TEST_DATA).expect("Failed to parse data.");
            gpu_field = Some(GPUFieldProvider::new(&vector_field));
            field_provider = Some(FieldProvider::new(vector_field));
        }
        // For desktop we load a file if it exists.
        #[cfg(not(target_arch = "wasm32"))]
        {
            let content: Vec<u8> = if let Some(ref path) = path {
                let mut file = std::fs::File::open(path).expect("Failed to open file!");
                let mut content = Vec::new();
                file.read_to_end(&mut content)
                    .expect("Failed to read file!");
                content
            } else {
                Vec::from(resources::fields::DEFAULT_SPIRAL)
            };
            let vector_field = bincode::deserialize(&content).expect("Failed to parse data.");
            gpu_field = Some(GPUFieldProvider::new(&vector_field));
            field_provider = Some(FieldProvider::new(vector_field));
        }

        let field_provider = field_provider.unwrap();
        let gpu_field = gpu_field.unwrap();
        let march = MarchingCubes::marching_cubes(&field_provider);
        let particles = ParticleEngine::new(field_provider);
        let gpu_particles = GPUParticleEngine::new(gpu_particle_count);

        let mut state = State::new();
        state.file_path = path;
        state.use_gpu_particles = start_with_gpu;
        state.directional_data = particles.calculate_highly_directional_positions();

        let mut gui = Gui::new(
            (INITIAL_WINDOW_WIDTH as f32, INITIAL_WINDOW_HEIGHT as f32),
            &state,
        );
        gui.seeding_loc_slider
            .set_steps(state.directional_data.len().max(1) as u32);

        gui.map.set_texture(Some(gpu_field.get_texture()));

        gui.world_points
            .set_points(particles.calculate_highly_directional_positions());

        App {
            window,
            state,
            particles,
            camera: camera::ArcBall::new(),
            time: 0.0,
            gui,
            mid_reload: false,
            gpu_field,
            gpu_particles: gpu_particles,
            march,
            gpu_particle_count,
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
                }
                _ => {}
            };
            let consumed = self
                .gui
                .handle_event(&event, &mut self.state, self.window.get_size());

            if !consumed {
                self.camera.handle_events(&event);
                self.gui.world_points.set_camera_pos(self.camera.get_position());
                self.gui.world_points.set_camera_target_pos(self.camera.get_target());
            }
        }

        // Update camera position.
        {
            self.camera.set_target_position(self.state.camera_target);
            self.gui.seeding_sphere.retarget(self.state.camera_target);
            self.gui.map.set_target(self.state.camera_target);
        }        

        // Replace particle data if requested.
        // Special preparation for web due to it's asynchronous nature.
        #[cfg(target_arch = "wasm32")]
        {
            self.state.reload_file = match js!(return isUpdated();) {
                stdweb::Value::Bool(b) => b,
                _ => panic!("Unknown isUpdated return type"),
            };
        }

        // Two-step file reload:
        // Step 1 (reload_file): Write "Loading file".
        // Step 2 (mid_reload): Load file.
        // Done so in order to render "loading" before starting the process.
        if self.state.reload_file || self.mid_reload {
            self.state.reload_file = false;
            if self.mid_reload {
                match file_loading::reload_file(&self.state) {
                    Ok(res) => match res {
                        FileResult::OptionsFile(opt) => {
                            self.state.options_file = Some(opt);
                            self.gui
                                .status
                                .set_status("Options file loaded - load raw file next.".to_owned());
                        }
                        FileResult::VectorField((field_provider, gpu_field_provider)) => {
                            self.gui.status.set_status("File loaded!".to_owned());
                            self.state.options_file = None;
                            self.march = MarchingCubes::marching_cubes(&field_provider);
                            self.particles = ParticleEngine::new(field_provider);
                            self.state.directional_data =
                                self.particles.calculate_highly_directional_positions();
                            self.gui
                                .seeding_loc_slider
                                .set_steps(self.state.directional_data.len().max(1) as u32);
                            self.gpu_field = gpu_field_provider;
                            self.gpu_particles = GPUParticleEngine::new(self.gpu_particle_count);
                            self.gui.map.set_texture(Some(self.gpu_field.get_texture()));
                            self.gui.world_points
                                .set_points(self.particles.calculate_highly_directional_positions());
                        }
                    },
                    Err(e) => self.gui.status.set_status(e),
                }
                self.mid_reload = false;
            } else {
                self.gui
                    .status
                    .set_status_ongoing("Loading file".to_owned());
                self.mid_reload = true;
            }
        }

        // Update status label timer
        self.gui.status.update_status();

        // Update particle system
        let (cx, cy, cz) = self.camera.get_position();
        self.march.set_light_dir((cx, cy, cz));

        // Clear screen
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(Context::COLOR_BUFFER_BIT);
        context.clear(Context::DEPTH_BUFFER_BIT);

        // Draw everything
        Context::get_context().enable(Context::DEPTH_TEST);
        let projection_matrix = self.camera.get_projection_matrix();
        Context::get_context().disable(Context::DEPTH_TEST);

        self.gui.seeding_sphere.resize(self.state.seeding_size);

        if self.state.use_gpu_particles {
            Context::get_context().disable(Context::DEPTH_TEST);
            self.gpu_particles
                .update(&self.gpu_field, &self.state, &self.camera);
            Context::get_context().enable(Context::DEPTH_TEST);
            context.blend_func(Context::SRC_ALPHA, Context::ONE);
            Context::get_context().depth_mask(false);
            self.gpu_particles.draw_transformed(&projection_matrix);
            Context::get_context().depth_mask(true);
            context.blend_func(Context::SRC_ALPHA, Context::ONE_MINUS_SRC_ALPHA);
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
        self.gui.world_points.set_view_matrix(&projection_matrix);
        self.gui.draw_3d_elements(&projection_matrix);

        Context::get_context().disable(Context::DEPTH_TEST);

        self.gui.draw();

        self.window.swap_buffers();
        self.time += 0.01;
        self.state.is_running
    }
}
