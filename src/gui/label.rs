use std::{cell::RefCell, rc::Rc};

use graphics::{position, Drawable, Font, Text};
use gui::UiElement;
use na::Matrix4;

/// A simple button that can be pressed.
pub struct Label<'a> {
    pos: position::Absolute,
    text: Text<'a>,
}

impl<'a> Label<'a> {
    /// Creates a new button.
    pub fn new(
        pos: position::Absolute,
        screensize: (f32, f32),
        text: String,
        font: Rc<RefCell<Font<'a>>>,
    ) -> Self {
        let coords = pos.to_relative(screensize).get_coordinates();

        Self {
            pos,
            text: Text::new(text, font, coords.x1, coords.y1, 0.0, screensize),
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text.set_text(text);
    }
}

impl<'a> UiElement for Label<'a> {
    fn resize(&mut self, screensize: (f32, f32)) {
        let text_coords = self.pos.to_relative(screensize).get_coordinates();
        self.text
            .set_position(text_coords.x1, text_coords.y1, 0.0, screensize);
    }
}

impl<'a> Drawable for Label<'a> {
    fn draw_transformed(&self, view_matrix: &Matrix4<f32>) {
        self.text.draw_transformed(view_matrix);
    }
}
