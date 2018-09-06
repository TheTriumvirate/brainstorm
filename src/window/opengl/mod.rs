//! Very inspired from kiss3d's implementation of window and context
//! link: https://github.com/sebcrozet/kiss3d

use window::{abstract_window::*, Event as EventWrapper, MouseButton as MouseButtonWrapper, *};

use gl;
use glutin::{
    self,
    dpi::*,
    Api::OpenGl,
    ElementState, GlContext, GlRequest, KeyboardInput as KeyboardData, ModifiersState,
    MouseScrollDelta, VirtualKeyCode,
    WindowEvent::{self, CloseRequested, CursorMoved, KeyboardInput, MouseInput, MouseWheel},
};

fn translate_event(event: &WindowEvent) -> Option<EventWrapper> {
    match *event {
        CloseRequested => Some(EventWrapper::Quit),
        CursorMoved {
            position: LogicalPosition { x, y },
            ..
        } => Some(EventWrapper::CursorMoved { x, y }),
        MouseInput { state, button, .. } => Some(EventWrapper::CursorInput {
            pressed: state == ElementState::Pressed,
            button: MouseButtonWrapper::from(button),
        }),
        MouseWheel {
            delta: MouseScrollDelta::LineDelta(x, y),
            ..
        } => Some(EventWrapper::CursorScroll(x.signum() as f32, y.signum() as f32)),
        MouseWheel {
            delta: MouseScrollDelta::PixelDelta(LogicalPosition { x, y }),
            ..
        } => Some(EventWrapper::CursorScroll(x.signum() as f32, y.signum() as f32)),
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
        _ => None, // Unknown or Unhandled event
    }
}

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
            .with_gl(GlRequest::Specific(OpenGl, (3, 2)))
            .with_vsync(true);
        let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

        unsafe {
            gl_window.make_current().unwrap();
        }

        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

        GLWindow {
            window: gl_window,
            events: events_loop,
            width,
            height,
        }
    }

    fn run_loop(mut callback: impl FnMut(f64) -> bool + 'static) {
        while callback(0.0) {
            // TODO: Proper wrapper
            // Temporary solution
            unsafe {
                gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                gl::Enable(gl::BLEND);
                gl::Enable(gl::PROGRAM_POINT_SIZE);
            }
        }
    }

    fn get_events(&mut self) -> Vec<EventWrapper> {
        let mut events: Vec<EventWrapper> = Vec::new();
        self.events.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                if let Some(x) = translate_event(&event) {
                    events.push(x);
                }
            };
        });
        events
    }

    fn swap_buffers(&mut self) {
        self.window.swap_buffers().unwrap();
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
