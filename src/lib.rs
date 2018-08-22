#![cfg_attr(feature = "cargo-clippy", allow(cast_lossless))]
extern crate noise;

#[macro_use]
#[cfg(target_arch = "wasm32")]
extern crate stdweb;
#[macro_use]
#[cfg(target_arch = "wasm32")]
extern crate stdweb_derive;
#[macro_use]
#[cfg(target_arch = "wasm32")]
extern crate serde_derive;
#[cfg(target_arch = "wasm32")]
extern crate serde;

extern crate nalgebra as na;

pub mod shaders;
pub mod window;
pub mod camera;

extern crate rand;

use rand::rngs::SmallRng;
use rand::{FromEntropy, Rng};
use std::f32;

use camera::*;

const PARTICLE_COUNT: usize = 100_000;

use window::*;

pub struct App {
    window: Window,
    camera: ArcBallCamera,
    particles: Vec<f32>,
    time: f32,
    rng: SmallRng,
    running: bool,
    mvp_uniform: UniformLocation
}

impl App {
    pub fn new() -> App {
        // Set up window and shaders.
        let mut window = Window::new("Particles!", 1000, 1000);
        let program = configure_shaders(&mut window);

        // Set up particles.
        let mut data = Vec::new();
        for i in 0..PARTICLE_COUNT {
            let x: f32 = (i as f32) / PARTICLE_COUNT as f32 - 0.5;
            data.push(x);
            data.push(-0.5);
            data.push(0.0);
        }

        // Bind the window buffer.
        let vb = window
            .create_buffer()
            .expect("Failed to create window buffer.");
        window.bind_buffer(Window::ARRAY_BUFFER, &vb);
        window.buffer_data(Window::ARRAY_BUFFER, &data, Window::DYNAMIC_DRAW);

        // Bind the vertex array.
        let vao = window
            .create_vertex_array()
            .expect("Failed to create vertex array.");
        window.bind_vertex_array(&vao);

        // Enable the attribute arrays.
        let pos_attrib = window.get_attrib_location(&program, "position");
        window.vertex_attrib_pointer(&pos_attrib, 3, Window::FLOAT, false, 3, 0);
        window.enable_vertex_attrib_array(&pos_attrib);

        let mvp_uniform = window.get_uniform_location(&program, "MVP");

        // Run main loop.
        let time: f32 = 0.0;
        let rng = SmallRng::from_entropy();
        let running = true;

        App {
            window,
            camera: ArcBallCamera::new(),
            particles: data,
            time,
            rng,
            running,
            mvp_uniform
        }
    }

    pub fn run(&mut self) -> bool {
        self.update()
    }

    fn update(&mut self) -> bool {
        let mut is_running = self.running;
        for event in self.window.get_events().iter() {
            //println!("Event fired: {:?}", event);
            match event {
                Event::KeyboardInput{pressed: true, key: Key::W, modifiers: ModifierKeys{ctrl: true, ..}} 
                    | Event::Quit => is_running = false,
                _ => ()
            }

            self.camera.handle_events(&event);
        }
            
        self.running = is_running;
        for i in 0..PARTICLE_COUNT {
            if self.rng.gen_bool(0.02) {
                self.particles[i * 3] = self.rng.gen_range::<f32>(-0.5, 0.5);
                self.particles[i * 3 + 1] = self.rng.gen_range::<f32>(-0.5, -0.47);
                self.particles[i * 3 + 2] = 0.0;
            }
            let (dx, dy, dz) = field((self.particles[i * 3], self.particles[i * 3 + 1], self.particles[i * 3 + 2]));
            self.particles[i * 3] += dx * 0.001;
            self.particles[i * 3 + 1] += dy * 0.001;
            self.particles[i * 3 + 2] += dz * 0.001;
        }

        
        let projection_matrix = self.camera.get_projection_matrix();
        self.window.uniform_matrix_4fv(&self.mvp_uniform, 1, false, &projection_matrix);

        self.window.buffer_data(Window::ARRAY_BUFFER, &self.particles, Window::DYNAMIC_DRAW);
        self.window.clear_color(0.0, 0.0, 0.0, 1.0);
        self.window.clear(Window::COLOR_BUFFER_BIT);
        self.window.draw_arrays(Window::POINTS, 0, PARTICLE_COUNT as i32);
        self.window.swap_buffers();
        self.time += 0.01;
        self.running
    }
}

/// Gets the new position for a particle at a given position.
/// Assumes normalized vector return.
pub fn field((x, y, z): (f32, f32, f32)) -> (f32, f32, f32) {
   /* let angle = noise.get([x as f64, y as f64, z as f64, time as f64]) as f32 * f32::consts::PI * 2.0;
    let strength = noise
        .get([x as f64 + 4000.0, y as f64 + 2000.0, z as f64 + 8000.0, time as f64])
        .abs() as f32 * 9.0 + 1.0;*/
        let strength = 4.0;
    (
        -z * strength * 5.0,
        strength * (y.abs() + 1.0),
        x * strength * 5.0
    )
}

fn configure_shaders(window: &mut Window) -> Program {
    // Compile vertex shader
    let vertex_shader =
        std::str::from_utf8(shaders::VERTEX_SHADER).expect("Failed to load vertex shader.");
    let vs = window
        .create_shader(ShaderType::Vertex)
        .expect("Failed to create vertex shader.");
    window.shader_source(&vs, vertex_shader);
    window.compile_shader(&vs);

    if let Some(log) = window.get_shader_info_log(&vs) {
        println!("vertex shader log: {}", log);
    }

    // Compile fragment shader
    let fragment_shader =
        std::str::from_utf8(shaders::FRAGMENT_SHADER).expect("Failed to load fragment shader.");
    let fs = window
        .create_shader(ShaderType::Fragment)
        .expect("Failed to create fragment shader.");
    window.shader_source(&fs, fragment_shader);
    window.compile_shader(&fs);
    if let Some(log) = window.get_shader_info_log(&fs) {
        println!("fragment shader log: {}", log);
    }

    // Link program
    let program = window
        .create_program()
        .expect("Failed to create shader program.");
    window.attach_shader(&program, &vs);
    window.attach_shader(&program, &fs);
    window.link_program(&program);
    window.use_program(&program);

    program
}
