use std::str;

use super::State;
use window::*;
use gl_context::{AbstractContext, Context, shaders::*, VertexArray};

pub struct Gui {
    pub buttons: Vec<Button>,
    vb: VertexArray,
    vao: VertexArray,
    shaders: OurShader,
}

impl Gui {
    pub fn new() -> Self {
        let context = Context::get_context();

        // Bind the data buffer.
        let vb = context
            .create_buffer()
            .expect("Failed to create data buffer.");
        
        // Bind the vertex array.
        let vao = context
            .create_vertex_array()
            .expect("Failed to create vertex array.");
        
        // Set up shaders
        let vertex_shader = str::from_utf8(TRIANGLES_VERTEX_SHADER)
            .expect("Failed to read vertex shader");
        let fragment_shader = str::from_utf8(TRIANGLES_FRAGMENT_SHADER)
            .expect("Failed to read fragment shader");
        let shaders = OurShader::new(vertex_shader, fragment_shader, 2);
        
        let buttons = Vec::new();

        Gui { buttons, vb, vao, shaders }
    }

    pub fn draw(&self) {
        let context = Context::get_context();

        // Render particles
        let mut triangles = Vec::new();
        //for button in &self.buttons {
            triangles.push(-0.1);
            triangles.push(0.1);

            triangles.push(-0.1);
            triangles.push(-0.1);

            triangles.push(0.1);
            triangles.push(-0.1);
        //}
        
        context.bind_buffer(Context::ARRAY_BUFFER, &self.vb);
        context.buffer_data(
            Context::ARRAY_BUFFER,
            &triangles,
            Context::STATIC_DRAW,
        );
        context.bind_vertex_array(&self.vao);
        self.shaders.set_active();
        context.draw_arrays(Context::TRIANGLES, 0, (triangles.len() / 2) as i32);
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
