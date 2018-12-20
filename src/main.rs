mod camera;
mod file_loading;
mod graphics;
mod gui;
mod particles;
mod state;

pub use crate::state::State;
use crate::{
    camera::Camera,
    file_loading::FileResult,
    graphics::Drawable,
    gui::Gui,
    particles::{
        fieldprovider::FieldProvider, gpu_fieldprovider::GPUFieldProvider,
        gpu_particles::GPUParticleEngine, MarchingCubes, ParticleEngine,
    },
};
use gl_bindings::{AbstractContext, Context};
use std::{f32, io::Read, path::PathBuf};
#[cfg(not(target_arch = "wasm32"))]
use structopt::StructOpt;
use window::{AbstractWindow, Event, Window};

#[cfg(target_arch = "wasm32")]
use stdweb::*;

const INITIAL_WINDOW_WIDTH: u32 = 1000;
const INITIAL_WINDOW_HEIGHT: u32 = 1000;
#[allow(dead_code)]
const DEFAULT_GPU_PARTICLE_COUNT: usize = 768;

/// Main entry point for the Web application.
#[cfg(target_arch = "wasm32")]
fn main() {
    let mut app = App::new(None, false, DEFAULT_GPU_PARTICLE_COUNT);
    window::Window::run_loop(move |_| app.run());
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(StructOpt, Debug)]
struct Opt {
    /// File to process
    #[structopt(name = "FILE", parse(from_os_str))]
    file: Option<PathBuf>,

    /// Start with GPU particles instead of CPU particles.
    #[structopt(short = "g", long = "gpu")]
    gpu: bool,

    /// Number of particles (squared) to use on the GPU. It's recommended to use a power
    /// of two, like 256, 512 or 1024.
    #[structopt(short = "c", long = "gpu-particle-count", default_value = "512")]
    gpu_particle_count: usize,
}

/// Main entry point for the native application.
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let opt = Opt::from_args();

    let mut app = App::new(opt.file, opt.gpu, opt.gpu_particle_count);
    window::Window::run_loop(move |_| app.run());
}

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

        gui.map.set_texture(&Some(gpu_field.get_texture()));

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
            gpu_particles,
            march,
            gpu_particle_count,
        }
    }

    /// Runs the application for one frame.
    pub fn run(&mut self) -> bool {
        // Handle events
        for event in &self.window.get_events() {
            if let Event::Resized(w, h) = event {
                self.state.window_w = *w;
                self.state.window_h = *h;
                self.window.set_size(*w as u32, *h as u32)
            };

            let consumed = self
                .gui
                .handle_event(&event, &mut self.state, self.window.get_size());

            if !consumed {
                self.camera.handle_events(&event);
                self.gui
                    .world_points
                    .set_camera_pos(self.camera.get_position());
                self.gui
                    .world_points
                    .set_camera_target_pos(self.camera.get_target());
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

        // Load new file if requested.
        self.load_file();

        // Update status label timer
        self.gui.status.update_status();

        // Update particle system
        let (cx, cy, cz) = self.camera.get_position();
        self.march.set_light_dir((cx, cy, cz));

        self.render_all();
        self.time += 0.01;
        self.state.is_running
    }

    fn render_all(&mut self) {
        // Clear screen
        let context = Context::get_context();
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(Context::COLOR_BUFFER_BIT);
        context.clear(Context::DEPTH_BUFFER_BIT);

        // Draw everything
        context.enable(Context::DEPTH_TEST);
        let projection_matrix = self.camera.get_projection_matrix();
        self.gui.seeding_sphere.resize(self.state.seeding_size);

        if self.state.use_gpu_particles {
            context.disable(Context::DEPTH_TEST);
            self.gpu_particles
                .update(&self.gpu_field, &self.state, &self.camera);
            context.enable(Context::DEPTH_TEST);
            context.blend_func(Context::SRC_ALPHA, Context::ONE);
            context.depth_mask(false);
            self.gpu_particles.draw_transformed(&projection_matrix);
            context.depth_mask(true);
            context.blend_func(Context::SRC_ALPHA, Context::ONE_MINUS_SRC_ALPHA);
        } else {
            self.particles.update(&self.state, &self.camera);
            self.particles.draw(&projection_matrix, &self.state);
        }

        if self.state.mesh_transparency < 1.0 {
            context.depth_mask(false);
        }

        self.march.set_transparency(self.state.mesh_transparency);
        self.march.draw_transformed(&projection_matrix);

        if self.state.mesh_transparency < 1.0 {
            context.depth_mask(true);
        }
        self.gui.world_points.set_view_matrix(projection_matrix);
        self.gui.draw_3d_elements(&projection_matrix);

        context.disable(Context::DEPTH_TEST);

        self.gui.draw();

        self.window.swap_buffers();
    }

    fn load_file(&mut self) {
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
                            self.gui
                                .map
                                .set_texture(&Some(self.gpu_field.get_texture()));
                            self.gui.world_points.set_points(
                                self.particles.calculate_highly_directional_positions(),
                            );
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
    }
}
