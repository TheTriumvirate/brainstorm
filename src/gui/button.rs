use graphics::*;
use gui::UiElement;
use State;

/// A simple button that can be pressed.
pub struct Button {
    pos_abs: position::Absolute,
    pos_rel: position::Relative,
    rect: Rectangle,
    color: (f32, f32, f32),
    func: Box<dyn FnMut(&mut State)>,
}

impl Button {
    /// Creates a new button.
    pub fn new(
        pos_abs: position::Absolute,
        color: (f32, f32, f32),
        screensize: (f32, f32),
        func: Box<dyn FnMut(&mut State)>,
    ) -> Self {
        let pos_rel = pos_abs.to_relative(screensize);
        Self {
            pos_abs,
            pos_rel,
            func,
            color: color,
            rect: Rectangle::new(pos_rel.get_coordinates(), color),
        }
    }
}

impl UiElement for Button {
    fn is_within(&self, x: f64, y: f64) -> bool {
        let c = self.pos_rel.get_coordinates();
        x > c.x1.into() && x < c.x2.into() && y > c.y1.into() && y < c.y2.into()
    }

    fn click(&mut self, _x: f64, _y: f64, state: &mut State) {
        let func = &mut self.func;
        self.color = (1.0, 0.0, 0.0);
        self.rect = Rectangle::new(self.pos_rel.get_coordinates(), self.color);
        func(state);
    }

    fn resize(&mut self, screensize: (f32, f32)) {
        self.pos_rel = self.pos_abs.to_relative(screensize);
        self.rect = Rectangle::new(self.pos_rel.get_coordinates(), self.color);
    }
}

impl Drawable for Button {
    fn draw(&self) {
        self.rect.draw();
    }
}
