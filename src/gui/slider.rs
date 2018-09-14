use graphics::{Drawable, Rectangle};
use gui::UiElement;
use State;

pub struct Slider {
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
    value: f32,
    steps: f32,
    is_clicked: bool,
    rect_background: Rectangle,
    rect_track: Rectangle,
    rect_slider: Rectangle,
    func: Box<dyn FnMut(&mut State, f32)>,
}

impl Slider {
    pub fn new(
        x1: f32,
        x2: f32,
        y1: f32,
        y2: f32,
        steps: u32,
        func: Box<dyn FnMut(&mut State, f32)>,
    ) -> Self {
        let initial_value = 0.0;
        let height = y2 - y1;
        let width = x2 - x1;

        let track_height = (height / 10.0) * 2.0;
        let track_y1 = y1 + (height / 10.0) * 4.0;

        let slider_width = width / 20.0;
        let slider_x1 = x1 + (width * initial_value) - (slider_width / 2.0);

        Self {
            x1,
            x2,
            y1,
            y2,
            func,
            is_clicked: false,
            value: initial_value,
            steps: steps as f32,
            rect_background: Rectangle::new(x1, y1, width, height, (0.44, 0.5, 0.56)),
            rect_track: Rectangle::new(x1, track_y1, width, track_height, (0.58, 0.64, 0.7)),
            rect_slider: Rectangle::new(slider_x1, y1, slider_width, y2 - y1, (0.7, 0.75, 0.8)),
        }
    }

    fn update_slider_pos(&mut self, x: f64) {
        // Cap to edges
        let x = (x as f32).max(self.x1).min(self.x2);

        // Quantize to set intervals.
        // Slightly offset so it reaches the end of the slider despite rounding down.
        let offset = (self.x2 - self.x1) / self.steps;
        let fraction = (x as f32 - self.x1) / (self.x2 - self.x1) + offset;
        self.value = (fraction * self.steps) as u32 as f32 / self.steps;

        // Calculate new slider position
        let width = self.x2 - self.x1;
        let slider_width = width / 20.0;
        let slider_x1 = self.x1 + (width * self.value) - (slider_width / 2.0);
        self.rect_slider = Rectangle::new(
            slider_x1,
            self.y1,
            slider_width,
            self.y2 - self.y1,
            (0.7, 0.75, 0.8),
        );
    }
}

impl UiElement for Slider {
    fn is_within(&self, x: f64, y: f64) -> bool {
        x > self.x1.into() && x < self.x2.into() && y < self.y1.into() && y > self.y2.into()
    }

    fn click(&mut self, x: f64, _y: f64, state: &mut State) {
        self.is_clicked = true;
        self.update_slider_pos(x);

        let func = &mut self.func;
        func(state, self.value);
    }

    fn click_release(&mut self, _x: f64, _y: f64, _state: &mut State) {
        self.is_clicked = false;
    }

    fn mouse_moved(&mut self, x: f64, y: f64, state: &mut State) {
        if self.is_clicked {
            self.click(x, y, state);
        }
    }
}

impl Drawable for Slider {
    fn draw(&self) {
        self.rect_background.draw();
        self.rect_track.draw();
        self.rect_slider.draw();
    }
}
