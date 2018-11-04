#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;
use std::{cell::RefCell, rc::Rc};

use graphics::{position, Drawable, Font};
use gui::{Label, UiElement};
use na::Matrix4;

const STATUS_TIMEOUT_SECONDS: u64 = 5;

/// A simple button that can be pressed.
pub struct StatusLabel {
    label: Label<'static>,
    text: String,
    timer: Timer,
    ellipsis: bool,
}

impl StatusLabel {
    /// Creates a new button.
    pub fn new(
        pos: position::Absolute,
        screensize: (f32, f32),
        font: Rc<RefCell<Font<'static>>>,
    ) -> StatusLabel {
        Self {
            label: Label::new(pos, screensize, String::new(), font),
            text: String::new(),
            timer: Timer::new(),
            ellipsis: false,
        }
    }

    /// Set the status text.
    pub fn set_status(&mut self, text: String) {
        self.ellipsis = false;
        self.text = text.clone();
        self.label.set_text(text);
        self.timer.set();
    }
    
    /// Set an ongoing status text with repeating ellipsis.
    pub fn set_status_ongoing(&mut self, text: String) {
        self.ellipsis = true;
        self.text.push_str(text.as_ref());
        self.label.set_text(text);
        self.timer.set();
    }

    /// Updates the timers and text.
    pub fn update_status(&mut self) {
        let since_last = self.timer.elapsed_seconds();
        if self.ellipsis {
            let dots = since_last % 4;
            let mut new_string = self.text.clone();
            for _ in 0..dots {
                new_string.push_str(".");
            }
            self.label.set_text(new_string);
        } else if !self.text.is_empty() && since_last > STATUS_TIMEOUT_SECONDS {
            self.text.clear();
            self.label.set_text(String::new());
        }
    }
}

impl UiElement for StatusLabel {
    fn resize(&mut self, screensize: (f32, f32)) {
        self.label.resize(screensize);        
    }
}

impl Drawable for StatusLabel {
    fn draw_transformed(&self, view_matrix: &Matrix4<f32>) {
        self.label.draw_transformed(view_matrix);
    }
}

/// A struct acting as a stopwatch timer for both web and desktop.
struct Timer {
    #[cfg(target_arch = "wasm32")]
    time: f64,
    #[cfg(not(target_arch = "wasm32"))]
    time: Instant,
}

impl Timer {
    /// Starts a new timer.
    fn new() -> Self {
        Self {
            #[cfg(target_arch = "wasm32")]
            time: stdweb::web::Date::now(),
            #[cfg(not(target_arch = "wasm32"))]
            time: Instant::now(),
        }
    }

    /// Resets the timer
    fn set(&mut self) {
        #[cfg(target_arch = "wasm32")] {
            self.time = stdweb::web::Date::now();
        }
        #[cfg(not(target_arch = "wasm32"))] {
            self.time = Instant::now();
        }
    }

    /// Returns the number of elapsed seconds.
    fn elapsed_seconds(&self) -> u64 {
        #[cfg(target_arch = "wasm32")] {
            return ((stdweb::web::Date::now() - self.time) / 1000.0) as u64;
        }
        #[cfg(not(target_arch = "wasm32"))] {
            return Instant::now().duration_since(self.time).as_secs();
        }
    }
}