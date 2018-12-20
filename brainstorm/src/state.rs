/// Holds application state.
pub struct State {
    pub mouse_x: f64,
    pub mouse_y: f64,
    pub is_running: bool,
    pub highpass_filter: f32,
    pub lowpass_filter: f32,
    pub speed_multiplier: f32,
    pub seeding_size: f32,
    pub lifetime: f32,
    pub mesh_transparency: f32,
    pub particle_size: f32,
    pub particle_respawn_per_tick: u32,
    pub file_path: Option<std::path::PathBuf>,
    pub reload_file: bool,
    pub camera_target: (f32, f32, f32),
    pub window_w: f32,
    pub window_h: f32,
    pub use_gpu_particles: bool,
    pub directional_data: Vec<(f32, f32, f32)>,
    pub options_file: Option<reparser::Options>,
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
            mesh_transparency: 0.02,
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
