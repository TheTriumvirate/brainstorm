//! Module for all things GUI.

mod button;
mod label;
mod slider;
mod ui_element;

use std::{cell::RefCell, rc::Rc};

use graphics::{position, Drawable, Font};
use resources::fonts;
use gl_context::window::{ModifierKeys, MouseButton, Event, Key};
use State;
use na::Matrix4;

use self::{button::Button, label::Label, slider::Slider, ui_element::UiElement};

/// Represents the GUI for the application.
pub struct Gui {
    pub ui_visible_label: Box<ui_element::UiElement>,
    pub ui_visible_button: Button,
    pub ui_elements: Vec<Box<ui_element::UiElement>>,
}

impl Gui {
    /// Creates the GUI for the application.
    pub fn new(screensize: (f32, f32)) -> Self {
        let font = Rc::from(RefCell::from(Font::from_bytes(fonts::DEFAULT)));

        let mut ui_elements: Vec<Box<ui_element::UiElement>> = Vec::new();
        ui_elements.push(Box::new(Slider::new(
            position::Absolute {
                height: 40,
                width: 225,
                anchor: position::WindowCorner::BotRight,
                margin_vertical: 40,
                margin_horizontal: 40,
            },
            20,
            1.0,
            screensize,
            Box::new(|ref mut context, value| {
                context.lowpass_filter = value;
            }),
        )));
        ui_elements.push(Box::new(Slider::new(
            position::Absolute {
                height: 40,
                width: 225,
                anchor: position::WindowCorner::BotRight,
                margin_vertical: 120,
                margin_horizontal: 40,
            },
            20,
            0.0,
            screensize,
            Box::new(|ref mut context, value| {
                context.highpass_filter = value;
            }),
        )));
        ui_elements.push(Box::new(Slider::new(
            position::Absolute {
                height: 40,
                width: 225,
                anchor: position::WindowCorner::BotRight,
                margin_vertical: 40,
                margin_horizontal: 285,
            },
            10,
            0.5,
            screensize,
            Box::new(|ref mut context, value| {
                context.speed_multiplier = value;
            }),
        )));
        ui_elements.push(Box::new(Slider::new(
            position::Absolute {
                height: 40,
                width: 225,
                anchor: position::WindowCorner::BotRight,
                margin_vertical: 120,
                margin_horizontal: 285,
            },
            80,
            1.0,
            screensize,
            Box::new(|ref mut context, value| {
                context.seeding_size = value;
            }),
        )));
        ui_elements.push(Box::new(Slider::new(
            position::Absolute {
                height: 40,
                width: 225,
                anchor: position::WindowCorner::BotRight,
                margin_vertical: 200,
                margin_horizontal: 40,
            },
            80,
            0.2,
            screensize,
            Box::new(|ref mut context, value| {
                context.lifetime = value * 500.0;
            }),
        )));
        ui_elements.push(Box::new(Slider::new(
            position::Absolute {
                height: 40,
                width: 225,
                anchor: position::WindowCorner::BotRight,
                margin_vertical: 200,
                margin_horizontal: 285,
            },
            50,
            0.1,
            screensize,
            Box::new(|ref mut context, value| {
                context.mesh_transparency = value;
            }),
        )));
        ui_elements.push(Box::new(Slider::new(
            position::Absolute {
                height: 40,
                width: 225,
                anchor: position::WindowCorner::BotRight,
                margin_vertical: 280,
                margin_horizontal: 40,
            },
            20,
            0.5,
            screensize,
            Box::new(|ref mut context, value| {
                context.particle_size = value * 16.0;
            }),
        )));
        ui_elements.push(Box::new(Slider::new(
            position::Absolute {
                height: 40,
                width: 225,
                anchor: position::WindowCorner::BotRight,
                margin_vertical: 280,
                margin_horizontal:285,
            },
            50,
            0.5,
            screensize,
            Box::new(|ref mut context, value| {
                context.particle_respawn_per_tick = (value * 2000.0) as u32;
            }),
        )));
        ui_elements.push(Box::new(Slider::new(
            position::Absolute {
                height: 40,
                width: 225,
                anchor: position::WindowCorner::BotRight,
                margin_vertical: 360,
                margin_horizontal:285,
            },
            50,
            0.5,
            screensize,
            Box::new(|ref mut context, value| {
                context.texture_idx = value;
            }),
        )));
        ui_elements.push(Box::new(Label::new(
            position::Absolute {
                height: 0,
                width: 0,
                anchor: position::WindowCorner::BotRight,
                margin_vertical: 85,
                margin_horizontal: 265,
            },
            screensize,
            "Low-pass filter".to_owned(),
            font.clone(),
        )));
        ui_elements.push(Box::new(Label::new(
            position::Absolute {
                height: 0,
                width: 0,
                anchor: position::WindowCorner::BotRight,
                margin_vertical: 165,
                margin_horizontal: 265,
            },
            screensize,
            "High-pass filter".to_owned(),
            font.clone(),
        )));
        ui_elements.push(Box::new(Label::new(
            position::Absolute {
                height: 0,
                width: 0,
                anchor: position::WindowCorner::BotRight,
                margin_vertical: 85,
                margin_horizontal: 510,
            },
            screensize,
            "Speed".to_owned(),
            font.clone(),
        )));
        ui_elements.push(Box::new(Label::new(
            position::Absolute {
                height: 0,
                width: 0,
                anchor: position::WindowCorner::BotRight,
                margin_vertical: 165,
                margin_horizontal: 510,
            },
            screensize,
            "Seeding size".to_owned(),
            font.clone(),
        )));
        ui_elements.push(Box::new(Label::new(
            position::Absolute {
                height: 0,
                width: 0,
                anchor: position::WindowCorner::BotRight,
                margin_vertical: 250,
                margin_horizontal: 265,
            },
            screensize,
            "Lifetime".to_owned(),
            font.clone(),
        )));
        ui_elements.push(Box::new(Label::new(
            position::Absolute {
                height: 0,
                width: 0,
                anchor: position::WindowCorner::BotRight,
                margin_vertical: 250,
                margin_horizontal: 510,
            },
            screensize,
            "Mesh transparency".to_owned(),
            font.clone(),
        )));
        ui_elements.push(Box::new(Label::new(
            position::Absolute {
                height: 0,
                width: 0,
                anchor: position::WindowCorner::BotRight,
                margin_vertical: 335,
                margin_horizontal: 265,
            },
            screensize,
            "Particle size".to_owned(),
            font.clone(),
        )));
        ui_elements.push(Box::new(Label::new(
            position::Absolute {
                height: 0,
                width: 0,
                anchor: position::WindowCorner::BotRight,
                margin_vertical: 335,
                margin_horizontal: 510,
            },
            screensize,
            "Particle spawn rate".to_owned(),
            font.clone(),
        )));
        ui_elements.push(Box::new(Label::new(
            position::Absolute {
                height: 0,
                width: 0,
                anchor: position::WindowCorner::BotRight,
                margin_vertical: 410,
                margin_horizontal: 510,
            },
            screensize,
            "Texture idx".to_owned(),
            font.clone(),
        )));
        
        let ui_visible_label = Box::new(Label::new(
            position::Absolute {
                height: 40,
                width: 120,
                anchor: position::WindowCorner::BotLeft,
                margin_vertical: 50,
                margin_horizontal: 60,
            },
            screensize,
            "Toggle UI".to_owned(),
            font.clone(),
        ));

        let ui_visible_button = Button::new(
            position::Absolute {
                height: 40,
                width: 120,
                anchor: position::WindowCorner::BotLeft,
                margin_vertical: 40,
                margin_horizontal: 40,
            },
            (0.44, 0.5, 0.56),
            screensize,
            true,
            Box::new(|ref mut _context, _toggle_state| {}),
        );

        Gui { ui_elements, ui_visible_button, ui_visible_label }
    }

    /// Handles events from the window, mutating application state as needed.
    /// Returns whether or not the event was "consumed".
    pub fn handle_event(&mut self, event: &Event, state: &mut State, size: (u32, u32)) -> bool {
        match event {
            Event::Resized(x, y) => {
                self.ui_visible_button.resize((*x, *y));
                self.ui_visible_label.resize((*x, *y));
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
            Event::CursorMoved { x, y } => {
                state.mouse_x = (x - (size.0 as f64 / 2.0)) * 2.0 / size.0 as f64;
                state.mouse_y = (y - (size.1 as f64 / 2.0)) * -2.0 / size.1 as f64;
                
                self.ui_visible_button.mouse_moved(state.mouse_x, state.mouse_y, state);
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
                    if self.ui_visible_button.is_within(state.mouse_x, state.mouse_y) {
                        self.ui_visible_button.click(state.mouse_x, state.mouse_y, state);
                        handled = true;
                    }
                    if self.ui_visible_button.toggle_state() {
                        for element in &mut self.ui_elements {
                            if element.is_within(state.mouse_x, state.mouse_y) {
                                element.click(state.mouse_x, state.mouse_y, state);
                                handled = true;
                            }
                        }
                    }
                } else {
                    self.ui_visible_button.click_release(state.mouse_x, state.mouse_y, state);
                    for element in &mut self.ui_elements {
                        element.click_release(state.mouse_x, state.mouse_y, state);
                    }
                }
                handled
            }
            _ => false,
        }
    }
}

impl Drawable for Gui {
    fn draw_transformed(&self, view_matrix: &Matrix4<f32>) {
        self.ui_visible_button.draw_transformed(view_matrix);
        self.ui_visible_label.draw_transformed(view_matrix);
        if self.ui_visible_button.toggle_state() {
            for element in &self.ui_elements {
                element.draw_transformed(view_matrix);
            }
        }
    }
}
