use super::State;
use window::*;

pub struct Gui {
    pub buttons: Vec<Button>,
}

impl Gui {
    pub fn new() -> Self {
        Gui {
            buttons: Vec::new(),
        }
    }

    pub fn draw(&self) {
        unimplemented!()
    }

    pub fn handle_event(&mut self, event: &Event, state: &mut State) {
        match event {
            Event::KeyboardInput {
                pressed: true,
                key: Key::W,
                modifiers: ModifierKeys { ctrl: true, .. },
            }
            | Event::Quit => state.is_running = false,
            Event::CursorMoved { x, y } => {
                state.mouse_x = *x;
                state.mouse_y = *y;
            }
            Event::CursorInput {
                pressed: true,
                button: MouseButton::Left,
                ..
            } => {
                for button in self.buttons.iter_mut() {
                    if button.was_clicked(state.mouse_x, state.mouse_y) {
                        button.click(state);
                    }
                }
            }
            _ => (),
        }
    }
}

pub struct Button {
    pub x1: f32,
    pub x2: f32,
    pub y1: f32,
    pub y2: f32,
    pub color: (f32, f32, f32),
    pub func: Box<dyn FnMut(&mut State)>,
}

impl Button {
    pub fn was_clicked(&self, x: f64, y: f64) -> bool {
        x > self.x1.into() && x < self.x2.into() && y > self.y1.into() && y < self.y2.into()
    }

    pub fn click(&mut self, state: &mut State) {
        let func = &mut self.func;
        func(state);
    }
}
