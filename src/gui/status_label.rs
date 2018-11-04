use std::{cell::RefCell, rc::Rc, time::Instant};

use graphics::{position, Drawable, Font};
use gui::{Label, UiElement};
use na::Matrix4;

const STATUS_TIMEOUT_SECONDS: u64 = 5;

/// A simple button that can be pressed.
pub struct StatusLabel {
    label: Label<'static>,
    text: String,
    last_set: Instant,
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
            last_set: Instant::now(),
            ellipsis: false,
        }
    }

    /// Set the status text.
    pub fn set_status(&mut self, text: String) {
        self.ellipsis = false;
        self.text = text.clone();
        self.label.set_text(text);
        self.last_set = Instant::now();
    }
    
    /// Set an ongoing status text with repeating ellipsis.
    pub fn set_status_ongoing(&mut self, text: String) {
        self.ellipsis = true;
        self.text.push_str(text.as_ref());
        self.label.set_text(text);
        self.last_set = Instant::now();
    }

    pub fn update_status(&mut self) {
        let since_last = Instant::now().duration_since(self.last_set).as_secs();
        if self.ellipsis {
            let dots = since_last % 4;
            let mut new_string = self.text.clone();
            for _ in 0..dots {
                new_string.push_str(".");
            }
            self.label.set_text(new_string);
        } else if !self.text.is_empty() && since_last > STATUS_TIMEOUT_SECONDS {
            self.text.clear();
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
