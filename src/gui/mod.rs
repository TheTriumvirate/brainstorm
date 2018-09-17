//! Module for all things GUI.

mod button;
mod slider;
mod ui_element;

use graphics::Drawable;
use window::*;
use State;

use self::button::Button;
use self::slider::Slider;
use self::ui_element::UiElement;

/// Represents the GUI for the application.
pub struct Gui {
    pub ui_elements: Vec<Box<ui_element::UiElement>>,
}

impl Default for Gui {
    fn default() -> Self {
        Self::new()
    }
}

impl Gui {
    /// Creates the GUI for the application.
    pub fn new() -> Self {
        let mut ui_elements: Vec<Box<ui_element::UiElement>> = Vec::new();
        ui_elements.push(Box::new(Button::new(
            -0.90,
            -0.60,
            -0.80,
            -0.90,
            (0.44, 0.5, 0.56),
            Box::new(|ref mut _context| {}),
        )));
        ui_elements.push(Box::new(Slider::new(
            0.60,
            0.90,
            -0.64,
            -0.73,
            20,
            0.0,
            Box::new(|ref mut context, value| {
                context.highpass_filter = value;
            }),
        )));
        ui_elements.push(Box::new(Slider::new(
            0.60,
            0.90,
            -0.75,
            -0.84,
            20,
            1.0,
            Box::new(|ref mut context, value| {
                context.lowpass_filter = value;
            }),
        )));
        ui_elements.push(Box::new(Slider::new(
            0.60,
            0.90,
            -0.86,
            -0.95,
            10,
            0.5,
            Box::new(|ref mut context, value| {
                context.speed_multiplier = value;
            }),
        )));

        Gui { ui_elements }
    }

    /// Handles events from the window, mutating application state as needed.
    pub fn handle_event(&mut self, event: &Event, state: &mut State, size: (u32, u32)) {
        match event {
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
    fn draw(&self) {
        for element in &self.ui_elements {
            element.draw();
        }
    }
}
