use gui::UiElement;
use graphics::{Drawable, Rectangle};
use State;

pub struct Button {
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
    rect: Rectangle,
    _color: (f32, f32, f32),
    func: Box<dyn FnMut(&mut State)>,
}

impl Button {
    pub fn new(x1: f32, x2: f32, y1: f32, y2: f32, color: (f32, f32, f32), func: Box<dyn FnMut(&mut State)>) -> Self {
        Self {
            x1,
            x2,
            y1,
            y2,
            func,
            _color: color,
            rect: Rectangle::new(x1, y1, x2 - x1, y2 - y1),
        }
    }
}

impl UiElement for Button {
    fn was_clicked(&self, x: f64, y: f64) -> bool {
        x > self.x1.into() && x < self.x2.into() && y < self.y1.into() && y > self.y2.into()
    }

    fn click(&mut self, state: &mut State) {
        let func = &mut self.func;
        func(state);
    }
}

impl Drawable for Button {
    fn draw(&self) {
        self.rect.draw();
    }
}
