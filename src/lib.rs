#![cfg_attr(feature = "cargo-clippy", allow(cast_lossless))]
extern crate noise;

#[cfg(target_arch = "wasm32")]
extern crate stdweb;
#[cfg(target_arch = "wasm32")]
extern crate stdweb_derive;
#[cfg(target_arch = "wasm32")]
extern crate serde_derive;
#[cfg(target_arch = "wasm32")]
extern crate serde;

extern crate nalgebra as na;
extern crate rand;

extern crate gl_context;

extern crate lazy_static;

pub mod window;
pub mod camera;

use gl_context::AbstractContext;
use gl_context::Context;

use rand::rngs::SmallRng;
use rand::{FromEntropy, Rng};
use std::{
    f32,
    str,
};

use camera::*;
use gl_context::shaders::OurShader;
use window::*;

const PARTICLE_COUNT: usize = 100_000;

pub struct App {
    camera: ArcBallCamera,
    window: Window,
    particles: Vec<f32>,
    time: f32,
    rng: SmallRng,
    running: bool,
    mvp_uniform: gl_context::UniformLocation,
    shaders: OurShader,
}

impl App {
    pub fn new() -> App {
        // Set up window
        let window = Window::new("Particles!", 1000, 1000);
        
        // Set up particles.
        let mut data = Vec::new();
        for i in 0..PARTICLE_COUNT {
            let x: f32 = (i as f32) / PARTICLE_COUNT as f32 - 0.5;
            data.push(x);
            data.push(-0.5);
            data.push(0.0);
        }

        let context = Context::get_context();

        // Set up shaders
        let vertex_shader =
                str::from_utf8(gl_context::shaders::VERTEX_SHADER).expect("Failed to read vertex shader");
        let fragment_shader =
                str::from_utf8(gl_context::shaders::FRAGMENT_SHADER).expect("Failed to read fragment shader");
        let shaders = gl_context::shaders::OurShader::new(vertex_shader, fragment_shader);

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
        
        // Enable the attribute arrays.
        let mvp_uniform = {
            let pos_attrib = context.get_attrib_location(&shaders.program, "position");
            context.vertex_attrib_pointer(&pos_attrib, 3, Context::FLOAT, false, 3, 0);
            context.enable_vertex_attrib_array(&pos_attrib);        
            context.get_uniform_location(&shaders.program, "MVP")
        };

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
            mvp_uniform,
            shaders,
        }
    }

    pub fn run(&mut self) -> bool {
        self.update()
    }

    fn update(&mut self) -> bool {
        let context = Context::get_context();
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
        context.uniform_matrix_4fv(&self.mvp_uniform, 1, false, &projection_matrix);

        context.buffer_data(Context::ARRAY_BUFFER, &self.particles, Context::DYNAMIC_DRAW);
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(Context::COLOR_BUFFER_BIT);
        context.draw_arrays(Context::POINTS, 0, PARTICLE_COUNT as i32);
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

