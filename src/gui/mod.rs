mod button;
mod slider;
mod ui_element;

use graphics::Drawable;
use window::*;
use State;

pub use self::button::Button;
pub use self::slider::Slider;
pub use self::ui_element::UiElement;

pub struct Gui {
    pub ui_elements: Vec<Box<ui_element::UiElement>>,
}

impl Default for Gui {
    fn default() -> Self {
        Self::new()
    }
}

impl Gui {
    pub fn new() -> Self {
        let mut ui_elements: Vec<Box<ui_element::UiElement>> = Vec::new();
        ui_elements.push(Box::new(button::Button::new(
            -0.90,
            -0.60,
            -0.80,
            -0.90,
            (0.44, 0.5, 0.56),
            Box::new(|ref mut _context| {}),
        )));
        ui_elements.push(Box::new(slider::Slider::new(
            0.60,
            0.90,
            -0.80,
            -0.90,
            Box::new(|ref mut context, _value| {}),
        )));

        Gui { ui_elements }
    }

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
                    if element.is_within(state.mouse_x, state.mouse_y) {
                        element.mouse_over(state.mouse_x, state.mouse_y, state);
                    }
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
