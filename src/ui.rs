use rand::{rngs::SmallRng, FromEntropy, Rng};
use std::str;

use super::State;
use gl_context::{shaders::*, Buffer, BufferType, Context, AbstractContext};
use window::*;

pub struct Gui {
    pub buttons: Vec<Button>,
    vb: Buffer<f32>,
    shaders: OurShader,
}

impl Default for Gui {
    fn default() -> Self {
        Self::new()
    }
}

impl Gui {
    pub fn new() -> Self {
        // Bind the data buffer.
        let vb = Buffer::new(BufferType::Array);

        // Set up shaders
        let vertex_shader =
            str::from_utf8(TRIANGLES_VERTEX_SHADER).expect("Failed to read vertex shader");
        let fragment_shader =
            str::from_utf8(TRIANGLES_FRAGMENT_SHADER).expect("Failed to read fragment shader");
            
        let mut attributes = Vec::new();
        attributes.push(ShaderAttribute {name: "position".to_string(), size: 2});
        attributes.push(ShaderAttribute {name: "color".to_string(), size: 3});
        let shaders = OurShader::new(vertex_shader, fragment_shader, &attributes);

        let mut buttons = Vec::new();
        buttons.push(Button {
            x1: -0.90,
            x2: -0.60,
            y1: -0.80,
            y2: -0.90,
            color: (1.0, 1.0, 1.0),
            func: Box::new(|ref mut context| {
                let mut rng = SmallRng::from_entropy();
                context.ui_color = (
                    rng.gen_range::<f32>(0.0, 1.0),
                    rng.gen_range::<f32>(0.0, 1.0),
                    rng.gen_range::<f32>(0.0, 1.0),
                );
            }),
        });

        Gui {
            buttons,
            vb,
            shaders,
        }
    }

    pub fn draw(&mut self, state: &State) {
        // Render particles
        let mut triangles = Vec::new();
        for b in &self.buttons {
            let coords = [
                (b.x1, b.y1),
                (b.x1, b.y2),
                (b.x2, b.y2),
                (b.x1, b.y1),
                (b.x2, b.y2),
                (b.x2, b.y1),
            ];

            for c in &coords {
                triangles.push(c.0);
                triangles.push(c.1);
                triangles.push(state.ui_color.0);
                triangles.push(state.ui_color.1);
                triangles.push(state.ui_color.2);
            }
        }
        let context = Context::get_context();

        let len = triangles.len();
        self.vb.set_data(&triangles);
        self.vb.bind();
        self.vb.upload_data(0, len, false);
        self.shaders.use_program();
        self.shaders.bind_attribs();
        context.draw_arrays(Context::TRIANGLES, 0, (triangles.len() / 5) as i32);
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
                pressed: true,
                button: MouseButton::Left,
                ..
            } => {
                for button in &mut self.buttons {
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
        x > self.x1.into() && x < self.x2.into() && y < self.y1.into() && y > self.y2.into()
    }

    pub fn click(&mut self, state: &mut State) {
        let func = &mut self.func;
        func(state);
    }
}
