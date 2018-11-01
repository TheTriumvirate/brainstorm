//! This is heavily inspired by kiss3d's implementation of window and context.
//! Go check out their code! https://github.com/sebcrozet/kiss3d
#![allow(unused_results)]

use gl_bindings::{AbstractContext, Context};

use {Event as EventWrapper, MouseButton as MouseButtonWrapper, ModifierKeys, Key, AbstractWindow};

use stdweb::unstable::TryInto;
use stdweb::web::event::MouseButton as WebMouseButton;
use stdweb::web::event::*;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, window, IEventTarget, IParentNode};

use std::cell::{Cell, RefCell};
use std::rc::Rc;

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
    fn new(_: &str, _width: u32, _height: u32) -> Self {
        let canvas: CanvasElement = document()
            .query_selector("#canvas")
            .expect("No canvas found")
            .unwrap()
            .try_into()
            .unwrap();

        let events = Rc::new(RefCell::new(Vec::new()));

        // TODO: Overrides specified size, this is bad...
        let width = window().inner_width() as u32;
        let height = window().inner_height() as u32;
        {
            canvas.set_width(width);
            canvas.set_height(height);
            events
                .borrow_mut()
                .push(EventWrapper::Resized(width as f32, height as f32))
        }

        let pointers = Rc::new(RefCell::new(Vec::new()));
        let prev_dist = Rc::new(Cell::new(-1.0));

        // TODO: Refractor event registration
        canvas.add_event_listener(enclose!((events) move |event: MouseWheelEvent| {
            events.borrow_mut().push(EventWrapper::CursorScroll(event.delta_x().signum() as f32, -event.delta_y().signum() as f32));
        }));

        canvas.add_event_listener(enclose!((events, pointers) move |event: PointerDownEvent| {
            let x = js!(return @{&event}.clientX;).try_into().unwrap_or(0.0);
            let y = js!(return @{&event}.clientY;).try_into().unwrap_or(0.0);

            pointers.borrow_mut().push(PointerData {
                id: event.pointer_id(),
                client_x: x as f32,
                client_y: y as f32,
            });
                events.borrow_mut().push(EventWrapper::CursorMoved{x, y});
            events.borrow_mut().push(EventWrapper::CursorInput {button: MouseButtonWrapper::Left, pressed: true});
        }));

        canvas.add_event_listener(
            enclose!((events, pointers, prev_dist) move |event: PointerMoveEvent| {
            let mut pointers = pointers.borrow_mut();
            let x = js!(return @{&event}.clientX;).try_into().unwrap_or(0.0);
            let y = js!(return @{&event}.clientY;).try_into().unwrap_or(0.0);
                
            let len = pointers.len().to_string();

            for i in 0..pointers.len() {
                if pointers[i].id == event.pointer_id() {
                    pointers[i] = PointerData {
                        id: event.pointer_id(),
                        client_x: x as f32,
                        client_y: y as f32,
                    };
                    break;
                }
            }

            if pointers.len() == 2 {

                let mut ox = 0.0;
                let mut oy = 0.0;
                for pointer in pointers.iter() {
                    if pointer.id != event.pointer_id() {
                        ox = pointer.client_x;
                        oy = pointer.client_y;
                    }
                }

                let dx = ox - x as f32;
                let dy = oy - y as f32;
                let dist = (dx * dx + dy * dy).sqrt();

                let p_dist = prev_dist.get();
                if p_dist > 0.0 {
                    let delta = (dist - p_dist) / 10.0;
                    // TODO: zoom event
                    events.borrow_mut().push(EventWrapper::CursorScroll(0.0, delta as f32));
                }

                prev_dist.set(dist);

            }
            else if pointers.len() == 1 {

                events.borrow_mut().push(EventWrapper::CursorMoved{x, y});
            }

        }),
        );

        canvas.add_event_listener(enclose!((events, pointers, prev_dist) move |event: PointerUpEvent| {
            //pointerCount.borrow_mut() = count -  1;
            events.borrow_mut().push(EventWrapper::CursorInput {button: MouseButtonWrapper::Left, pressed: false});
            pointers.borrow_mut().retain(|ref x| x.id != event.pointer_id());
            prev_dist.set(-1.0);
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

        window().add_event_listener(enclose!((events) move |_event: ResizeEvent| {
            let width = window().inner_width();
            let height = window().inner_height();
            let canvas: CanvasElement = document()
                .query_selector("#canvas")
                .expect("No canvas found")
                .unwrap()
                .try_into()
                .unwrap();

            canvas.set_width(width as u32);
            canvas.set_height(height as u32);
            events.borrow_mut().push(EventWrapper::Resized(width as f32, height as f32))
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

    fn set_size(&mut self, width: u32, height: u32) {
        Context::get_context().viewport(0, 0, width as i32, height as i32);
        self.height = height;
        self.width = width;
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
