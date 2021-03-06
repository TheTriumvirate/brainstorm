//! This is heavily inspired by kiss3d's implementation of window and context.
//! Go check out their code! https://github.com/sebcrozet/kiss3d
extern crate gl;

use crate::{
    AbstractWindow, Event as EventWrapper, Key, ModifierKeys, MouseButton as MouseButtonWrapper,
};

use glutin::{
    self,
    dpi::{LogicalPosition, LogicalSize},
    ElementState, GlContext, GlRequest, KeyboardInput as KeyboardData, ModifiersState,
    MouseScrollDelta, VirtualKeyCode,
    WindowEvent::{
        self, CloseRequested, CursorMoved, KeyboardInput, MouseInput, MouseWheel, Resized,
    },
};

/// Translates from OpenGL events to our own event enum.
fn translate_event(event: &WindowEvent, hidpi_factor: f64) -> Option<EventWrapper> {
    match *event {
        CloseRequested => Some(EventWrapper::Quit),
        CursorMoved {
            position: LogicalPosition { x, y },
            ..
        } => {
            // OpenGL has to deal with High-DPI window scaling.
            // We handle this inside the OpenGL module to avoid dealing with it
            // everywhere in the codebase.
            let x = x * hidpi_factor;
            let y = y * hidpi_factor;
            Some(EventWrapper::CursorMoved { x, y })
        }
        MouseInput { state, button, .. } => Some(EventWrapper::CursorInput {
            pressed: state == ElementState::Pressed,
            button: MouseButtonWrapper::from(button),
        }),
        MouseWheel {
            delta: MouseScrollDelta::LineDelta(x, y),
            ..
        } => Some(EventWrapper::CursorScroll(
            x.signum() as f32,
            y.signum() as f32,
        )),
        MouseWheel {
            delta: MouseScrollDelta::PixelDelta(LogicalPosition { x, y }),
            ..
        } => Some(EventWrapper::CursorScroll(
            x.signum() as f32,
            y.signum() as f32,
        )),
        KeyboardInput {
            input:
                KeyboardData {
                    state,
                    virtual_keycode,
                    modifiers,
                    ..
                },
            ..
        } => Some(EventWrapper::KeyboardInput {
            pressed: state == ElementState::Pressed,
            key: Key::from(virtual_keycode),
            modifiers: ModifierKeys::from(modifiers),
        }),
        Resized(LogicalSize { width, height }) => {
            // OpenGL has to deal with High-DPI window scaling.
            // We handle this inside the OpenGL module to avoid dealing with it
            // everywhere in the codebase.
            let width = width * hidpi_factor;
            let height = height * hidpi_factor;
            Some(EventWrapper::Resized(width as f32, height as f32))
        }
        _ => None, // Unknown or Unhandled event
    }
}

/// Holds GL Window state.
pub struct GLWindow {
    height: u32,
    width: u32,
    window: glutin::GlWindow,
    events: glutin::EventsLoop,
}

impl AbstractWindow for GLWindow {
    fn new(title: &str, width: u32, height: u32) -> Self {
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(LogicalSize::new(width as f64, height as f64));
        let context = glutin::ContextBuilder::new()
            .with_gl(GlRequest::GlThenGles {
                opengl_version: (3, 2),
                opengles_version: (2, 0),
            })
            .with_multisampling(0)
            .with_vsync(true);
        let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

        unsafe {
            gl_window.make_current().unwrap();
        };

        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

        GLWindow {
            window: gl_window,
            events: events_loop,
            width,
            height,
        }
    }

    fn run_loop(mut callback: impl FnMut(f64) -> bool + 'static) {
        while callback(0.0) {}
    }

    fn get_events(&mut self) -> Vec<EventWrapper> {
        let mut events: Vec<EventWrapper> = Vec::new();
        let hidpi_factor = self.window.get_hidpi_factor();
        self.events.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                if let Some(x) = translate_event(&event, hidpi_factor) {
                    events.push(x);
                }
            };
        });
        events
    }

    fn swap_buffers(&mut self) {
        self.window.swap_buffers().unwrap();
    }

    fn set_size(&mut self, width: u32, height: u32) {
        // FIXME: Do this with a call from context instead
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }
        self.height = height;
        self.width = width;
    }

    fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

impl From<glutin::MouseButton> for MouseButtonWrapper {
    fn from(button: glutin::MouseButton) -> Self {
        match button {
            glutin::MouseButton::Left => MouseButtonWrapper::Left,
            glutin::MouseButton::Right => MouseButtonWrapper::Right,
            glutin::MouseButton::Middle => MouseButtonWrapper::Middle,
            glutin::MouseButton::Other(d) => MouseButtonWrapper::Other(d),
        }
    }
}

impl From<Option<VirtualKeyCode>> for Key {
    fn from(key: Option<VirtualKeyCode>) -> Self {
        match key {
            None => Key::Unknown,
            Some(x) => match x {
                VirtualKeyCode::Key1 => Key::Num1,
                VirtualKeyCode::Key2 => Key::Num2,
                VirtualKeyCode::Key3 => Key::Num3,
                VirtualKeyCode::Key4 => Key::Num4,
                VirtualKeyCode::Key5 => Key::Num5,
                VirtualKeyCode::Key6 => Key::Num6,
                VirtualKeyCode::Key7 => Key::Num7,
                VirtualKeyCode::Key8 => Key::Num8,
                VirtualKeyCode::Key9 => Key::Num9,
                VirtualKeyCode::Key0 => Key::Num0,

                VirtualKeyCode::A => Key::A,
                VirtualKeyCode::B => Key::B,
                VirtualKeyCode::C => Key::C,
                VirtualKeyCode::D => Key::D,
                VirtualKeyCode::E => Key::E,
                VirtualKeyCode::F => Key::F,
                VirtualKeyCode::G => Key::G,
                VirtualKeyCode::H => Key::H,
                VirtualKeyCode::I => Key::I,
                VirtualKeyCode::J => Key::J,
                VirtualKeyCode::K => Key::K,
                VirtualKeyCode::L => Key::L,
                VirtualKeyCode::M => Key::M,
                VirtualKeyCode::N => Key::N,
                VirtualKeyCode::O => Key::O,
                VirtualKeyCode::P => Key::P,
                VirtualKeyCode::Q => Key::Q,
                VirtualKeyCode::R => Key::R,
                VirtualKeyCode::S => Key::S,
                VirtualKeyCode::T => Key::T,
                VirtualKeyCode::U => Key::U,
                VirtualKeyCode::V => Key::V,
                VirtualKeyCode::W => Key::W,
                VirtualKeyCode::X => Key::X,
                VirtualKeyCode::Y => Key::Y,
                VirtualKeyCode::Z => Key::Z,

                _ => Key::Unknown,
            },
        }
    }
}

impl From<ModifiersState> for ModifierKeys {
    fn from(
        ModifiersState {
            ctrl,
            shift,
            alt,
            logo,
        }: ModifiersState,
    ) -> Self {
        ModifierKeys {
            ctrl,
            shift,
            alt,
            logo,
        }
    }
}
