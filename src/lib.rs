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

pub mod camera;
pub mod particles;
pub mod ui;
pub mod window;

use gl_context::AbstractContext;
use gl_context::Context;

use particles::ParticleEngine;

use std::f32;

use camera::*;
use ui::*;
use window::*;

pub struct App {
    camera: ArcBallCamera,
    window: Window,
    time: f32,

    gui: Gui,
    state: State,
    particles: ParticleEngine,
}

pub struct State {
    mouse_x: f64,
    mouse_y: f64,
    is_running: bool,
    ui_color: (f32, f32, f32),
}

impl State {
    pub fn new() -> Self {
        State {
            mouse_x: 0.0,
            mouse_y: 0.0,
            is_running: true,
            ui_color: (0.47, 0.53, 0.6),
        }
    }
}

impl App {
    pub fn new() -> App {
        App {
            window: Window::new("Particles!", 1000, 1000),
            camera: ArcBallCamera::new(),
            time: 0.0,
            gui: Gui::new(),
            state: State::new(),
            particles: ParticleEngine::new(),
        }
    }

    pub fn run(&mut self) -> bool {
        self.update()
    }

    fn update(&mut self) -> bool {
        let context = Context::get_context();
        for event in self.window.get_events().iter() {
            self.gui
                .handle_event(&event, &mut self.state, self.window.get_size());
            self.camera.handle_events(&event);
        }
        self.camera.update();
        self.particles.update();

        // Clear screen
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(Context::COLOR_BUFFER_BIT);

        // Draw everything
        let projection_matrix = self.camera.get_projection_matrix();
        self.particles.draw(&projection_matrix);
        self.gui.draw(&self.state);

        self.window.swap_buffers();
        self.time += 0.01;

        self.state.is_running
    }
}
