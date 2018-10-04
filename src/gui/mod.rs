//! Module for all things GUI.

mod button;
mod label;
mod slider;
mod ui_element;

use std::{cell::RefCell, rc::Rc};

use graphics::{position, Drawable, Font};
use resources::fonts;
use window::*;
use State;
use na::Matrix4;

use self::{button::Button, label::Label, slider::Slider, ui_element::UiElement};

/// Represents the GUI for the application.
pub struct Gui {
    pub ui_elements: Vec<Box<ui_element::UiElement>>,
}

impl Gui {
    /// Creates the GUI for the application.
    pub fn new(screensize: (f32, f32)) -> Self {
        let font = Rc::from(RefCell::from(Font::from_bytes(fonts::DEFAULT)));

        let mut ui_elements: Vec<Box<ui_element::UiElement>> = Vec::new();
        ui_elements.push(Box::new(Button::new(
            position::Absolute {
                height: 40,
                width: 120,
                anchor: position::WindowCorner::BotLeft,
                margin_vertical: 40,
                margin_horizontal: 40,
            },
            (0.44, 0.5, 0.56),
            screensize,
            Box::new(|ref mut _context| {}),
        )));
        ui_elements.push(Box::new(Slider::new(
            position::Absolute {
                height: 40,
                width: 225,
                anchor: position::WindowCorner::BotRight,
                margin_vertical: 40,
                margin_horizontal: 40,
            },
            20,
            0.0,
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
            1.0,
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
                context.transparency = value;
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
            "Transparency".to_owned(),
            font.clone(),
        )));

        Gui { ui_elements }
    }

    /// Handles events from the window, mutating application state as needed.
    pub fn handle_event(&mut self, event: &Event, state: &mut State, size: (u32, u32)) {
        match event {
            Event::Resized(x, y) => {
                for element in &mut self.ui_elements {
                    element.resize((*x, *y));
                }
            }
            Event::KeyboardInput {
                pressed: true,
                key: Key::W,
                modifiers: ModifierKeys { ctrl: true, .. },
            }
            | Event::Quit => state.is_running = false,
            Event::CursorMoved { x, y } => {
                state.mouse_x = (x - (size.0 as f64 / 2.0)) * 2.0 / size.0 as f64;
                state.mouse_y = (y - (size.1 as f64 / 2.0)) * -2.0 / size.1 as f64;

                for element in &mut self.ui_elements {
                    element.mouse_moved(state.mouse_x, state.mouse_y, state);
                }
            }
            Event::CursorInput {
                button: MouseButton::Left,
                pressed,
                ..
            } => {
                if *pressed {
                    for element in &mut self.ui_elements {
                        if element.is_within(state.mouse_x, state.mouse_y) {
                            element.click(state.mouse_x, state.mouse_y, state);
                        }
                    }
                } else {
                    for element in &mut self.ui_elements {
                        element.click_release(state.mouse_x, state.mouse_y, state);
                    }
                }
            }
            _ => (),
        }
    }
}

impl Drawable for Gui {
    fn draw_transformed(&self, view_matrix: &Matrix4<f32>) {
        for element in &self.ui_elements {
            element.draw_transformed(view_matrix);
        }
    }
}
