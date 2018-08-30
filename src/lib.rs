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

use particles::ParticleEngine;

use std::{
    f32,
    str,
};

use camera::*;
use gl_context::shaders::OurShader;
use ui::*;
use window::*;

pub struct App {
    camera: ArcBallCamera,
    window: Window,
    time: f32,
    mvp_uniform: gl_context::UniformLocation,
    shaders: OurShader,
    gui: Gui,
    state: State,
    particles: ParticleEngine,
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
        
        let context = Context::get_context();
        let particles = ParticleEngine::new();

        // Set up shaders
        let vertex_shader = str::from_utf8(gl_context::shaders::VERTEX_SHADER)
            .expect("Failed to read vertex shader");
        let fragment_shader = str::from_utf8(gl_context::shaders::FRAGMENT_SHADER)
            .expect("Failed to read fragment shader");
        let shaders = gl_context::shaders::OurShader::new(vertex_shader, fragment_shader);
        
        // Enable the attribute arrays.
        let mvp_uniform = {
            let pos_attrib = context.get_attrib_location(&shaders.program, "position");
            context.vertex_attrib_pointer(&pos_attrib, 3, Context::FLOAT, false, 3, 0);
            context.enable_vertex_attrib_array(&pos_attrib);
            context.get_uniform_location(&shaders.program, "MVP")
        };

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
                //println!("Hello, SPACE!");
            }),
        });

        App {
            window,
            camera: ArcBallCamera::new(),
            time,
            mvp_uniform,
            shaders,
            gui,
            state,
            particles 
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
        self.camera.update();
        self.particles.update();

        let projection_matrix = self.camera.get_projection_matrix();
        context.uniform_matrix_4fv(&self.mvp_uniform, 1, false, &projection_matrix);

        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(Context::COLOR_BUFFER_BIT);
        
        self.particles.draw();

        self.window.swap_buffers();
        self.time += 0.01;

        self.state.is_running
    }
}
