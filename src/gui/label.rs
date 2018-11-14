use std::{cell::RefCell, rc::Rc};

use graphics::{position, Drawable, Font, Text, Rectangle};
use gui::UiElement;
use na::Matrix4;

/// A simple button that can be pressed.
pub struct Label<'a> {
    pos: position::Absolute,
    text: Text<'a>,
    rect: Rectangle,
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
        let mut rect_coord = pos.clone();
        rect_coord.width = 5;
        rect_coord.height = 5;
        Self {
            pos,
            text: Text::new(text, font, coords.x1, coords.y1, screensize),
            rect: Rectangle::new(rect_coord.to_relative(screensize).get_coordinates(), (1.0, 1.0, 1.0)),
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text.set_text(text);
    }
}

impl<'a> UiElement for Label<'a> {
    fn resize(&mut self, screensize: (f32, f32)) {
        let text_coords = self.pos.to_relative(screensize).get_coordinates();
        self.text.set_position(text_coords.x1, text_coords.y1, screensize);

        let mut rect_coord = self.pos.clone();
        rect_coord.width = 5;
        rect_coord.height = 5;
        self.rect = Rectangle::new(rect_coord.to_relative(screensize).get_coordinates(), (1.0, 1.0, 1.0));
    }
}

impl<'a> Drawable for Label<'a> {
    fn draw_transformed(&self, view_matrix: &Matrix4<f32>) {
        self.rect.draw_transformed(view_matrix);
        self.text.draw_transformed(view_matrix);
    }
}
