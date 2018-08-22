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

extern crate rand;

use rand::rngs::SmallRng;
use rand::{FromEntropy, Rng};
use std::{
    f32,
    str,
    rc::Rc,
    cell::RefCell,
};
use na::{Isometry3, Perspective3, Point3, Vector3};

const PARTICLE_COUNT: usize = 100_000;

use window::*;
use shaders::OurShader;

pub struct App {
    window: Rc<RefCell<Window>>,
    particles: Vec<f32>,
    time: f32,
    rng: SmallRng,
    running: bool,
    mvp_uniform: UniformLocation,
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

        // Set up shaders
        let window = Rc::new(RefCell::new(window));
        let vertex_shader =
                str::from_utf8(shaders::VERTEX_SHADER).expect("Failed to read vertex shader");
        let fragment_shader =
                str::from_utf8(shaders::FRAGMENT_SHADER).expect("Failed to read fragment shader");
        let shaders = shaders::OurShader::new(window.clone(), vertex_shader, fragment_shader);

        // Bind the window buffer.
        let vb = window
            .borrow_mut()
            .create_buffer()
            .expect("Failed to create window buffer.");
        window.borrow_mut().bind_buffer(Window::ARRAY_BUFFER, &vb);
        window.borrow_mut().buffer_data(Window::ARRAY_BUFFER, &data, Window::DYNAMIC_DRAW);

        // Bind the vertex array.
        let vao = window
            .borrow_mut()
            .create_vertex_array()
            .expect("Failed to create vertex array.");
        window.borrow_mut().bind_vertex_array(&vao);
        
        // Enable the attribute arrays.
        let mvp_uniform = {
            let window_mut = window.borrow_mut();
            let pos_attrib = window_mut.get_attrib_location(&shaders.program, "position");
            window_mut.vertex_attrib_pointer(&pos_attrib, 3, Window::FLOAT, false, 3, 0);
            window_mut.enable_vertex_attrib_array(&pos_attrib);        
            window_mut.get_uniform_location(&shaders.program, "MVP")
        };

        // Run main loop.
        let time: f32 = 0.0;
        let rng = SmallRng::from_entropy();
        let running = true;

        App {
            window,
            particles: data,
            time,
            rng,
            running,
            mvp_uniform,
            shaders,
        }
    }

    pub fn run(&mut self) {
        Window::run_loop(|_| self.update());
    }

    fn update(&mut self) -> bool {
        let mut is_running = self.running;
        let mut window = self.window.borrow_mut();
        window.handle_events(|event| {
            //println!("Event: {:?}", event);
            match event {
                Event::KeyboardInput{pressed: true, key: Key::W, modifiers: ModifierKeys{ctrl: true, ..}} 
                    | Event::Quit => is_running = false,
                _ => ()
            }
        });
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

        
        let perspective = Perspective3::new(1.0, f32::consts::PI / 4.0, 0.1, 1024.0);
        let eye = Point3::new(self.time.sin() * 2.0, 0.75, self.time.cos() * 2.0);
        let at = Point3::new(0.0, 0.0, 0.0);
        let view: Isometry3<f32> = Isometry3::look_at_rh(&eye, &at, &Vector3::y());
        let projection_matrix = perspective.as_matrix() * view.to_homogeneous();
        window.uniform_matrix_4fv(&self.mvp_uniform, 1, false, &projection_matrix);

        window.buffer_data(Window::ARRAY_BUFFER, &self.particles, Window::DYNAMIC_DRAW);
        window.clear_color(0.0, 0.0, 0.0, 1.0);
        window.clear(Window::COLOR_BUFFER_BIT);
        window.draw_arrays(Window::POINTS, 0, PARTICLE_COUNT as i32);
        window.swap_buffers();
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

