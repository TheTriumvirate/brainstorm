//! Module for all things GUI.

mod button;
mod label;
mod map;
mod model_bound;
mod slider;
mod status_label;
mod ui_definitions;
mod ui_element;
mod unit_sphere;
mod world_points;

use std::{cell::RefCell, rc::Rc};

use crate::graphics::{Drawable, Font};
use crate::State;
use na::Matrix4;
use resources::fonts;
use window::{Event, Key, ModifierKeys, MouseButton};

use self::{
    button::Button, label::Label, map::Map, model_bound::ModelBound, slider::Slider,
    status_label::StatusLabel, ui_element::UiElement, unit_sphere::UnitSphere,
    world_points::WorldPoints,
};

/// Represents the GUI for the application.
pub struct Gui {
    pub seeding_sphere: UnitSphere,
    pub model_bound: ModelBound,
    pub status: StatusLabel,
    pub ui_visible_button: Button,
    pub map: Map,
    pub world_points: WorldPoints,
    pub world_points_toggle: Button,
    ui_elements: Vec<Box<dyn ui_element::UiElement>>,
    ui_elements_cpu: Vec<Box<dyn ui_element::UiElement>>,
    ui_elements_gpu: Vec<Box<dyn ui_element::UiElement>>,
    show_cpu: bool,
}

impl Gui {
    /// Creates the GUI for the application.
    pub fn new(screensize: (f32, f32), state: &State) -> Self {
        let map = ui_definitions::map(screensize);
        let font = Rc::from(RefCell::from(Font::from_bytes(fonts::DEFAULT)));

        let ui_elements: Vec<Box<dyn ui_element::UiElement>> = vec![
            ui_definitions::lowpass_filter(screensize, font.clone()),
            ui_definitions::highpass_filter(screensize, font.clone()),
            ui_definitions::speed_multiplier(screensize, font.clone()),
            ui_definitions::seeding_size(screensize, font.clone()),
            ui_definitions::mesh_transparency(screensize, font.clone()),
            ui_definitions::load_file(screensize, font.clone()),
            ui_definitions::credits_label(screensize, font.clone()),
            ui_definitions::cpu_gpu_particles_toggle(screensize, font.clone()),
        ];
        let ui_elements_cpu: Vec<Box<dyn ui_element::UiElement>> = vec![
            ui_definitions::cpu_lifetime(screensize, font.clone()),
            ui_definitions::cpu_particle_size(screensize, font.clone()),
            ui_definitions::cpu_particle_spawn_rate(screensize, font.clone()),
        ];
        let ui_elements_gpu: Vec<Box<dyn ui_element::UiElement>> =
            vec![ui_definitions::gpu_transparency(screensize, font.clone())];

        let ui_visible_button = ui_definitions::toggle_ui(screensize, font.clone());
        let status = ui_definitions::status_label(screensize, font.clone());
        let seeding_sphere = UnitSphere::new((0.0, 0.0, 0.0), state.seeding_size);
        let model_bound = ModelBound::new();
        let world_points = ui_definitions::world_points(screensize, font.clone());
        let world_points_toggle = ui_definitions::toggle_world_points(screensize, font.clone());

        Gui {
            model_bound,
            seeding_sphere,
            status,
            ui_elements,
            ui_visible_button,
            ui_elements_cpu,
            ui_elements_gpu,
            map,
            world_points,
            world_points_toggle,
            show_cpu: state.use_cpu_particles,
        }
    }

    /// Handles events from the window, mutating application state as needed.
    /// Returns whether or not the event was "consumed".
    pub fn handle_event(&mut self, event: &Event, state: &mut State, size: (u32, u32)) -> bool {
        self.show_cpu = state.use_cpu_particles;
        match event {
            Event::Resized(x, y) => {
                self.ui_visible_button.resize((*x, *y));
                self.world_points_toggle.resize((*x, *y));
                self.status.resize((*x, *y));
                self.map.resize((*x, *y));
                for element in self.iter_ui_mut() {
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
                pressed: true, key, ..
            } => {
                match key {
                    Key::W | Key::S | Key::A | Key::D | Key::Q | Key::E => {
                        let ch = 0.05;
                        state.camera_target.0 += match key {
                            Key::A => ch,
                            Key::D => -ch,
                            _ => 0.0,
                        };
                        state.camera_target.1 += match key {
                            Key::Q => ch,
                            Key::E => -ch,
                            _ => 0.0,
                        };
                        state.camera_target.2 += match key {
                            Key::W => ch,
                            Key::S => -ch,
                            _ => 0.0,
                        };
                    }
                    _ => {}
                }
                false
            }
            Event::CursorMoved { x, y } => {
                state.mouse_x = (x - (f64::from(size.0) / 2.0)) * 2.0 / f64::from(size.0);
                state.mouse_y = (y - (f64::from(size.1) / 2.0)) * -2.0 / f64::from(size.1);

                self.map.mouse_moved(state.mouse_x, state.mouse_y, state);
                self.world_points
                    .mouse_moved(state.mouse_x, state.mouse_y, state);
                if self.map.clicked() {
                    // TODO: Set camera position
                    state.camera_target = self.map.get_target();
                }

                for element in self.iter_ui_mut() {
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
                    if self.world_points_toggle.toggle_state()
                        && self.world_points.is_within(state.mouse_x, state.mouse_y)
                    {
                        self.world_points.click(state.mouse_x, state.mouse_y, state);
                        handled = true;
                    }
                    if self
                        .ui_visible_button
                        .is_within(state.mouse_x, state.mouse_y)
                    {
                        self.ui_visible_button
                            .click(state.mouse_x, state.mouse_y, state);
                        handled = true;
                    }
                    if self.ui_visible_button.toggle_state() {
                        if self
                            .world_points_toggle
                            .is_within(state.mouse_x, state.mouse_y)
                        {
                            self.world_points_toggle
                                .click(state.mouse_x, state.mouse_y, state);
                            handled = true;
                        }
                        for element in self.iter_ui_mut() {
                            if element.is_within(state.mouse_x, state.mouse_y) {
                                element.click(state.mouse_x, state.mouse_y, state);
                                handled = true;
                            }
                        }
                    }
                } else {
                    self.map.click_release(state.mouse_x, state.mouse_y, state);
                    self.ui_visible_button
                        .click_release(state.mouse_x, state.mouse_y, state);
                    self.world_points_toggle
                        .click_release(state.mouse_x, state.mouse_y, state);
                    for element in self.iter_ui_mut() {
                        element.click_release(state.mouse_x, state.mouse_y, state);
                    }
                }
                handled
            }
            _ => false,
        }
    }

    /// Draws the 3D elements of the UI
    pub fn draw_3d_elements(&self, view_matrix: &Matrix4<f32>) {
        if self.ui_visible_button.toggle_state() {
            self.model_bound.draw_transformed(view_matrix);
            self.seeding_sphere.draw_transformed(view_matrix);
        }
        if self.world_points_toggle.toggle_state() {
            self.world_points.draw_transformed(view_matrix);
        }
    }

    /// Creats a mutable iterator over the UI elements, including the GPU- or GPU-specific ones
    /// depending on the cached setting.
    #[inline]
    fn iter_ui_mut(&mut self) -> impl Iterator<Item = &mut Box<dyn ui_element::UiElement>> {
        if self.show_cpu {
            self.ui_elements
                .iter_mut()
                .chain(self.ui_elements_cpu.iter_mut())
        } else {
            self.ui_elements
                .iter_mut()
                .chain(self.ui_elements_gpu.iter_mut())
        }
    }

    /// Creats a mutable iterator over the UI elements, including the GPU- or CPU-specific ones
    /// depending on the cached setting.
    #[inline]
    fn iter_ui(&self) -> impl Iterator<Item = &Box<dyn ui_element::UiElement>> {
        if self.show_cpu {
            self.ui_elements.iter().chain(self.ui_elements_cpu.iter())
        } else {
            self.ui_elements.iter().chain(self.ui_elements_gpu.iter())
        }
    }
}

impl Drawable for Gui {
    fn draw_transformed(&self, view_matrix: &Matrix4<f32>) {
        self.ui_visible_button.draw_transformed(view_matrix);
        self.status.draw_transformed(view_matrix);
        self.map.draw_transformed(view_matrix);

        if self.ui_visible_button.toggle_state() {
            self.world_points_toggle.draw_transformed(view_matrix);
            for element in self.iter_ui() {
                element.draw_transformed(view_matrix);
            }
        }
    }
}
