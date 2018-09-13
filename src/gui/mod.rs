mod button;
mod ui_element;

use rand::{rngs::SmallRng, FromEntropy, Rng};

use graphics::Drawable;
use window::*;
use State;

pub use self::button::Button;
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
            (1.0, 1.0, 1.0),
            Box::new(|ref mut context| {
                let mut rng = SmallRng::from_entropy();
                context.ui_color = (
                    rng.gen_range::<f32>(0.0, 1.0),
                    rng.gen_range::<f32>(0.0, 1.0),
                    rng.gen_range::<f32>(0.0, 1.0),
                );
            }),
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
            }
            Event::CursorInput {
                button: MouseButton::Left,
                pressed,
                ..
            } => {
                if *pressed {
                    for button in &mut self.ui_elements {
                        button.click_release(state);
                    }
                } else {
                    for button in &mut self.ui_elements {
                        if button.was_clicked(state.mouse_x, state.mouse_y) {
                            button.click(state);
                        }
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
