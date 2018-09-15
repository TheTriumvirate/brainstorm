//! This is heavily inspired by kiss3d's implementation of window and context.
//! Go check out their code! https://github.com/sebcrozet/kiss3d
#![allow(unused_results)]

use stdweb::unstable::TryInto;
use stdweb::web::event::MouseButton as WebMouseButton;
use stdweb::web::event::*;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, window, IEventTarget, IParentNode};

use std::cell::RefCell;
use std::rc::Rc;

use window::abstract_window::*;
use window::Event as EventWrapper;
use window::MouseButton as MouseButtonWrapper;
use window::*;

// Shamelessly stolen from stdweb,
// Shamelessly stolen from webplatform's TodoMVC example.
macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

#[derive(Clone)]
struct PointerData {
    id: i32,
    client_x: f32,
    client_y: f32,
}

/// Holds GL Window state.
pub struct WebGLWindow {
    events: Rc<RefCell<Vec<EventWrapper>>>,
    width: u32,
    height: u32,
}

impl AbstractWindow for WebGLWindow {
    fn new(_: &str, width: u32, height: u32) -> Self {
        let canvas: CanvasElement = document()
            .query_selector("#canvas")
            .expect("No canvas found")
            .unwrap()
            .try_into()
            .unwrap();

        canvas.set_width(width);
        canvas.set_height(height);

        let events = Rc::new(RefCell::new(Vec::new()));

        let pointers = RefCell::new(Vec::new());

        // TODO: Refractor event registration
        canvas.add_event_listener(enclose!((events) move |event: MouseMoveEvent| {
            events.borrow_mut().push(EventWrapper::CursorMoved{x: event.client_x() as f64, y: event.client_y() as f64});
        }));

        canvas.add_event_listener(enclose!((events) move |event: MouseDownEvent| {
            events.borrow_mut().push(EventWrapper::CursorInput {button: MouseButtonWrapper::from(event.button()), pressed: true});
        }));

        canvas.add_event_listener(enclose!((events) move |event: MouseUpEvent| {
            events.borrow_mut().push(EventWrapper::CursorInput {button: MouseButtonWrapper::from(event.button()), pressed: false});
        }));

        canvas.add_event_listener(enclose!((events) move |event: MouseWheelEvent| {
            events.borrow_mut().push(EventWrapper::CursorScroll(event.delta_x().signum() as f32, -event.delta_y().signum() as f32));
        }));

        canvas.add_event_listener(
            enclose!((events, pointers) move |event: PointerDownEvent| {
            pointers.borrow_mut().push(PointerData {
                id: event.pointer_id(),
                client_x: event.client_x() as f32,
                client_y: event.client_y() as f32,
            });
            events.borrow_mut().push(EventWrapper::CursorInput {button: MouseButtonWrapper::from(event.button()), pressed: true});
        }),
        );

        canvas.add_event_listener(
            enclose!((events, pointers) move |event: PointerMoveEvent| {
            let mut pointers = pointers.borrow_mut();
            for i in 0..pointers.len() {
                if pointers[i].id == event.pointer_id() {
                    pointers[i] = PointerData {
                        id: event.pointer_id(),
                        client_x: event.client_x() as f32,
                        client_y: event.client_y() as f32,
                    };
                    break;
                }
            }

            if pointers.len() == 2 {
                let dx = event.client_x() as f32;
                let dy = event.client_y() as f32;
                //let dist = (dx * dx + dy * dy).sqrt();
                events.borrow_mut().push(EventWrapper::CursorScroll(dx.signum() as f32, -dy.signum() as f32));

            }
            else if pointers.len() == 1 {

                events.borrow_mut().push(EventWrapper::CursorMoved{x: event.client_x() as f64, y: event.client_y() as f64});
            }

        }),
        );

        canvas.add_event_listener(enclose!((events, pointers) move |event: PointerUpEvent| {
            //let count = pointerCount.borrow();
            //pointerCount.borrow_mut() = count -  1;
            events.borrow_mut().push(EventWrapper::CursorInput {button: MouseButtonWrapper::from(event.button()), pressed: false});
            pointers.borrow_mut().retain(|ref x| x.id != event.pointer_id());
        }));

        // canvas does not support key events (for some reason...)
        window().add_event_listener(enclose!((events) move |event: KeyDownEvent| {
            events.borrow_mut().push(EventWrapper::KeyboardInput{pressed: true, key: Key::from(event.key()),
                modifiers: ModifierKeys{ctrl: event.ctrl_key(), shift: event.shift_key(), alt: event.alt_key(), logo: event.meta_key()}})
        }));
        window().add_event_listener(enclose!((events) move |event: KeyUpEvent| {
            events.borrow_mut().push(EventWrapper::KeyboardInput{pressed: false, key: Key::from(event.key()),
                modifiers: ModifierKeys{ctrl: event.ctrl_key(), shift: event.shift_key(), alt: event.alt_key(), logo: event.meta_key()}})
        }));

        WebGLWindow {
            events: events.clone(),
            width,
            height,
        }
    }

    fn run_loop(mut callback: impl FnMut(f64) -> bool + 'static) {
        let _ = window().request_animation_frame(|t| {
            if callback(t) {
                let _ = Self::run_loop(callback);
            }
        });
    }

    fn get_events(&mut self) -> Vec<EventWrapper> {
        let res = self.events.borrow().to_vec();
        self.events.borrow_mut().clear();
        res
    }

    fn swap_buffers(&mut self) {
        // No need to swap buffers on webgl
    }

    fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

// TODO: Microoptimization: use hash table
impl From<String> for Key {
    fn from(key: String) -> Self {
        match key.to_uppercase().as_str() {
            "1" => Key::Num1,
            "2" => Key::Num2,
            "3" => Key::Num3,
            "4" => Key::Num4,
            "5" => Key::Num5,
            "6" => Key::Num6,
            "7" => Key::Num7,
            "8" => Key::Num8,
            "9" => Key::Num9,
            "0" => Key::Num0,

            "A" => Key::A,
            "B" => Key::B,
            "C" => Key::C,
            "D" => Key::D,
            "E" => Key::E,
            "F" => Key::F,
            "G" => Key::G,
            "H" => Key::H,
            "I" => Key::I,
            "J" => Key::J,
            "K" => Key::K,
            "L" => Key::L,
            "M" => Key::M,
            "N" => Key::N,
            "O" => Key::O,
            "P" => Key::P,
            "Q" => Key::Q,
            "R" => Key::R,
            "S" => Key::S,
            "T" => Key::T,
            "U" => Key::U,
            "V" => Key::V,
            "W" => Key::W,
            "X" => Key::X,
            "Y" => Key::Y,
            "Z" => Key::Z,

            _ => Key::Unknown,
        }
    }
}

impl From<WebMouseButton> for MouseButtonWrapper {
    fn from(key: WebMouseButton) -> Self {
        match key {
            WebMouseButton::Left => MouseButtonWrapper::Left,
            WebMouseButton::Right => MouseButtonWrapper::Right,
            WebMouseButton::Wheel => MouseButtonWrapper::Middle,
            WebMouseButton::Button4 => MouseButtonWrapper::Other(4),
            WebMouseButton::Button5 => MouseButtonWrapper::Other(5),
        }
    }
}
