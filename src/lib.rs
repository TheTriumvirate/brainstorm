#![cfg_attr(feature = "cargo-clippy", allow(cast_lossless))]
#[macro_use]
extern crate lazy_static;
extern crate nalgebra as na;
extern crate rand;
#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate serde;

extern crate gl_context;

extern crate resources;

extern crate unicode_normalization;

extern crate rusttype;

pub mod camera;
pub mod graphics;
pub mod gui;
pub mod particles;

use gl_context::{AbstractContext, Context};
use graphics::{Drawable, Circle, Cube};
use particles::ParticleEngine;

use std::f32;

use camera::Camera;
use gui::{Gui};
use gl_context::window::{AbstractWindow, Window, Event};

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
    pub fn new() -> App {
        #[cfg(target_arch = "wasm32")]
        {
            stdweb::initialize();
        }
        App {
            window: Window::new("Brainstorm!", 900, 900),
            camera: camera::ArcBall::new(),
            time: 0.0,
            gui: Gui::new((1000.0, 1000.0)),
            state: State::new(),
            particles: ParticleEngine::new(),
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
            self.gui
                .handle_event(&event, &mut self.state, self.window.get_size());
            self.camera.handle_events(&event);
        }

        // Update camera and particle system
        self.camera.update();
        self.particles.update(&self.state, &mut self.camera);

        // Clear screen
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(Context::COLOR_BUFFER_BIT);
        context.clear(Context::DEPTH_BUFFER_BIT);

        // Draw everything
        self.window.enable_depth();
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

        self.particles.draw(&projection_matrix, &self.state, &self.window);
        self.window.disable_depth();
        self.gui.draw();

        self.window.swap_buffers();
        self.time += 0.01;

        self.state.is_running
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
