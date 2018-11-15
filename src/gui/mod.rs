//! Module for all things GUI.

mod button;
mod label;
mod model_bound;
mod slider;
mod status_label;
mod ui_element;
mod ui_definitions;
mod unit_sphere;
mod map;

use std::{cell::RefCell, rc::Rc};

use graphics::{Drawable, Font};
use resources::fonts;
use window::{ModifierKeys, MouseButton, Event, Key};
use State;
use na::Matrix4;

use self::{
    button::Button,
    label::Label,
    model_bound::ModelBound,
    slider::Slider,
    status_label::StatusLabel,
    ui_element::UiElement,
    unit_sphere::UnitSphere,
    map::Map,
};

/// Represents the GUI for the application.
pub struct Gui {
    pub seeding_sphere: UnitSphere,
    pub model_bound: ModelBound,
    pub status: StatusLabel,
    pub ui_visible_button: Button,
    pub seeding_loc_slider: Slider,
    pub ui_elements: Vec<Box<ui_element::UiElement>>,
    pub map: Map,
}

impl Gui {
    /// Creates the GUI for the application.
    pub fn new(screensize: (f32, f32), state: &State) -> Self {
        let map = ui_definitions::map(screensize);
        let font = Rc::from(RefCell::from(Font::from_bytes(fonts::DEFAULT)));

        let ui_elements: Vec<Box<ui_element::UiElement>> = vec![
            ui_definitions::lowpass_filter(screensize, font.clone()),
            ui_definitions::highpass_filter(screensize, font.clone()),
            ui_definitions::speed_multiplier(screensize, font.clone()),
            ui_definitions::seeding_size(screensize, font.clone()),
            ui_definitions::lifetime(screensize, font.clone()),
            ui_definitions::mesh_transparency(screensize, font.clone()),
            ui_definitions::particle_size(screensize, font.clone()),
            ui_definitions::particle_spawn_rate(screensize, font.clone()),
            ui_definitions::load_file(screensize, font.clone()),
            ui_definitions::credits_label(screensize, font.clone()),
            ui_definitions::texture_idx(screensize, font.clone()),
            ui_definitions::cpu_gpu_particles_toggle(screensize, font.clone()),
            ui_definitions::move_camera_x_f(screensize, font.clone()),
            ui_definitions::move_camera_x_b(screensize, font.clone()),
            ui_definitions::move_camera_y_f(screensize, font.clone()),
            ui_definitions::move_camera_y_b(screensize, font.clone()),
            ui_definitions::move_camera_z_f(screensize, font.clone()),
            ui_definitions::move_camera_z_b(screensize, font.clone()),
        ];
        
        let seeding_loc_slider = ui_definitions::directional_areas(screensize, font.clone());
        let ui_visible_button = ui_definitions::toggle_ui(screensize, font.clone());
        let status = ui_definitions::status_label(screensize, font.clone());
        let seeding_sphere = UnitSphere::new((0.0, 0.0, 0.0), state.seeding_size);
        let model_bound = ModelBound::new();
        Gui { model_bound, seeding_sphere, status, ui_elements, ui_visible_button, seeding_loc_slider, map  }
    }

    /// Handles events from the window, mutating application state as needed.
    /// Returns whether or not the event was "consumed".
    pub fn handle_event(&mut self, event: &Event, state: &mut State, size: (u32, u32)) -> bool {
        match event {
            Event::Resized(x, y) => {
                self.ui_visible_button.resize((*x, *y));
                self.seeding_loc_slider.resize((*x, *y));
                self.status.resize((*x, *y));
                self.map.resize((*x, *y));
                for element in &mut self.ui_elements {
                    element.resize((*x, *y));
                }
                false
            }
            Event::KeyboardInput {
                pressed: true,
                key: Key::W,
                modifiers: ModifierKeys { ctrl: true, .. },
            }
            | Event::Quit => {
                state.is_running = false;
                false
            }
            Event::KeyboardInput {
                pressed: true,
                key,
                ..
            } => {
                match key {
                    Key::W | Key::S | Key::A | Key::D | Key::Q | Key::E => {
                        let ch = 1.0;
                        state.camera_delta_movement = match key {
                            Key::W => (0.0, 0.0, ch),
                            Key::S => (0.0, 0.0, -ch),
                            Key::A => (ch, 0.0, 0.0),
                            Key::D => (-ch, 0.0, 0.0),
                            Key::Q => (0.0, ch, 0.0),
                            Key::E => (0.0, -ch, 0.0),
                            _ => (0.0, 0.0, 0.0),
                        }
                    }
                    _ => {}
                }
                false
            }
            Event::CursorMoved { x, y } => {
                state.mouse_x = (x - (size.0 as f64 / 2.0)) * 2.0 / size.0 as f64;
                state.mouse_y = (y - (size.1 as f64 / 2.0)) * -2.0 / size.1 as f64;

                self.map.mouse_moved(state.mouse_x, state.mouse_y, state);
                if self.map.clicked() {
                    // TODO: Set camera position
                }

                self.ui_visible_button.mouse_moved(state.mouse_x, state.mouse_y, state);
                self.seeding_loc_slider.mouse_moved(state.mouse_x, state.mouse_y, state);
                for element in &mut self.ui_elements {
                    element.mouse_moved(state.mouse_x, state.mouse_y, state);
                }
                false
            }
            Event::CursorInput {
                button: MouseButton::Left,
                pressed,
                ..
            } => {
                let mut handled = false;
                if *pressed {
                    if self.map.is_within(state.mouse_x, state.mouse_y) {
                        self.map.click(state.mouse_x, state.mouse_y, state);
                        handled = true;
                    }
                    if self.ui_visible_button.is_within(state.mouse_x, state.mouse_y) {
                        self.ui_visible_button.click(state.mouse_x, state.mouse_y, state);
                        handled = true;
                    }
                    if self.ui_visible_button.toggle_state() {
                        if self.seeding_loc_slider.is_within(state.mouse_x, state.mouse_y) {
                            self.seeding_loc_slider.click(state.mouse_x, state.mouse_y, state);
                            handled = true;
                        }
                        for element in &mut self.ui_elements {
                            if element.is_within(state.mouse_x, state.mouse_y) {
                                element.click(state.mouse_x, state.mouse_y, state);
                                handled = true;
                            }
                        }
                    }
                } else {
                    self.map.click_release(state.mouse_x, state.mouse_y, state);
                    self.ui_visible_button.click_release(state.mouse_x, state.mouse_y, state);
                    self.seeding_loc_slider.click_release(state.mouse_x, state.mouse_y, state);
                    for element in &mut self.ui_elements {
                        element.click_release(state.mouse_x, state.mouse_y, state);
                    }
                }
                handled
            }
            _ => false,
        }
    }

    pub fn draw_3d_elements(&self, view_matrix: &Matrix4<f32>) {
        if self.ui_visible_button.toggle_state() {
            self.model_bound.draw_transformed(view_matrix);
            self.seeding_sphere.draw_transformed(view_matrix);
        }
        self.map.draw_transformed(view_matrix);
    }
}

impl Drawable for Gui {
    fn draw_transformed(&self, view_matrix: &Matrix4<f32>) {
        self.ui_visible_button.draw_transformed(view_matrix);
        self.status.draw_transformed(view_matrix);
        if self.ui_visible_button.toggle_state() {
            self.seeding_loc_slider.draw_transformed(view_matrix);
            for element in &self.ui_elements {
                element.draw_transformed(view_matrix);
            }
        }
    }
}
