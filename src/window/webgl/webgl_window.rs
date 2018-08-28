/*
 * Very inspired from kiss3d's implementation of window and context
 * link: https://github.com/sebcrozet/kiss3d
 */
#![allow(unused_results)]

use window::abstract_window::*;
use window::Event as EventWrapper;
use window::MouseButton as MouseButtonWrapper;
use window::*;

use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, window, IParentNode, IEventTarget};
use stdweb::web::event::MouseButton as WebMouseButton;
use stdweb::web::event::*;

use std::rc::Rc;
use std::cell::RefCell;

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

pub struct WebGLWindow {
    events: Rc<RefCell<Vec<EventWrapper>>>
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
            events.borrow_mut().push(EventWrapper::CursorScroll(event.delta_x() as f32, event.delta_y() as f32));
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


        WebGLWindow { events: events.clone() }
    }

    fn run_loop(mut callback: impl FnMut(f64) -> bool + 'static) {
        let _ = window().request_animation_frame(|t| {
            if (callback(t)) {
                let _ = Self::run_loop(callback);
            }
        });
    }

    fn get_events(&mut self) -> Vec<EventWrapper>
    {
        let res = self.events.borrow().to_vec();
        self.events.borrow_mut().clear();
        res
    }

    fn swap_buffers(&mut self) {
        // No need to swap buffers on webgl
    }

}


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

            _ => Key::Unknown
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