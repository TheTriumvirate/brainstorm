use na::Matrix4;
use std::{cell::RefCell, rc::Rc};

use graphics::{Drawable, Font, position, position::WindowCorner, Rectangle};
use gui::{Label, UiElement};
use State;

/// Vertical offset factor for the label.
const LABEL_V_FAC: f32 = 1.5;

/// A simple slider giving a value from 0.0 to 1.0.
pub struct Slider {
    pos_abs: position::Absolute,
    pos_rel: position::Relative,
    track_pos: position::Absolute,
    slider_pos: position::Absolute,
    cached_screensize: (f32, f32),
    value: f32,
    steps: f32,
    is_clicked: bool,
    rect_track: Rectangle,
    rect_slider: Rectangle,
    func: Box<dyn FnMut(&mut State, f32)>,
    label: Label<'static>,
}

impl Slider {
    /// Creates a new slider. Note that initial_value should be
    /// between 0.0 and 1.0.
    pub fn new(
        pos_abs: position::Absolute,
        steps: u32,
        initial_value: f32,
        screensize: (f32, f32),
        func: Box<dyn FnMut(&mut State, f32)>,
        text: String,
        font: Rc<RefCell<Font<'static>>>
    ) -> Self {
        assert!(initial_value >= 0.0 && initial_value <= 1.0);
        let track_pos = position::Absolute {
            height: pos_abs.height / 5,
            width: pos_abs.width,
            anchor: pos_abs.anchor,
            margin_vertical: pos_abs.margin_vertical + pos_abs.height / 10 * 4,
            margin_horizontal: pos_abs.margin_horizontal,
        };

        let initial_value_directional = match track_pos.anchor {
            WindowCorner::BotRight | WindowCorner::TopRight => 1.0 - initial_value,
            _ => initial_value,
        };

        let slider_pos = position::Absolute {
            height: pos_abs.height,
            width: pos_abs.width / 20,
            anchor: pos_abs.anchor,
            margin_vertical: pos_abs.margin_vertical,
            margin_horizontal: pos_abs.margin_horizontal - (pos_abs.width / 40)
                + (pos_abs.width as f32 * initial_value_directional) as u32,
        };

        let label = Label::new(
            position::Absolute {
                height: 0,
                width: 0,
                anchor: pos_abs.anchor,
                margin_vertical: match pos_abs.anchor {
                    WindowCorner::BotLeft | WindowCorner::BotRight
                        => pos_abs.margin_vertical + (pos_abs.height as f32 * LABEL_V_FAC) as u32,
                    _ => pos_abs.margin_vertical - (pos_abs.height as f32 * LABEL_V_FAC) as u32,
                },
                margin_horizontal: match pos_abs.anchor {
                    WindowCorner::BotRight | WindowCorner::TopRight
                        => pos_abs.margin_horizontal + pos_abs.width,
                    _ => pos_abs.margin_horizontal
                },
            },
            screensize,
            text,
            font,
        );

        Self {
            pos_abs,
            pos_rel: pos_abs.to_relative(screensize),
            track_pos,
            slider_pos,
            func,
            cached_screensize: screensize,
            is_clicked: false,
            value: initial_value,
            steps: steps as f32,
            rect_track: Rectangle::new(
                track_pos.to_relative(screensize).get_coordinates(),
                (0.58, 0.64, 0.7),
            ),
            rect_slider: Rectangle::new(
                slider_pos.to_relative(screensize).get_coordinates(),
                (0.7, 0.75, 0.8),
            ),
            label,
        }
    }

    pub fn set_steps(&mut self, steps: u32) {
        self.steps = steps as f32;
    }

    /// Moves the visible slider to match the `x` value.
    fn update_slider_pos(&mut self, x: f64) {
        use graphics::position::WindowCorner;

        // Cap to edges
        let c = self.pos_rel.get_coordinates();
        let x = (x as f32).max(c.x1).min(c.x2);

        // Quantize to set intervals.
        // Slightly offset so it reaches the end of the slider despite rounding down.
        let offset = (c.x2 - c.x1) / self.steps;
        let fraction = (x as f32 - c.x1) / (c.x2 - c.x1) + offset;
        self.value = (fraction * self.steps) as u32 as f32 / self.steps;

        // Calculate new slider position
        let mut margin = self.pos_abs.margin_horizontal - self.slider_pos.width / 2;
        let m = (self.pos_abs.width as f32 * self.value) as u32;
        margin += match self.slider_pos.anchor {
            WindowCorner::BotLeft | WindowCorner::TopLeft => m,
            _ => self.pos_abs.width - m,
        };
        self.slider_pos.margin_horizontal = margin;
        self.rect_slider = Rectangle::new(
            self.slider_pos
                .to_relative(self.cached_screensize)
                .get_coordinates(),
            (0.7, 0.75, 0.8),
        );
    }
}

impl UiElement for Slider {
    fn is_within(&self, x: f64, y: f64) -> bool {
        let c = self.pos_rel.get_coordinates();

        let slider_overflow = (c.x2 - c.x1) / 20.0;
        let x1 = c.x1 - slider_overflow;
        let x2 = c.x2 + slider_overflow;
        x > x1.into() && x < x2.into() && y > c.y1.into() && y < c.y2.into()
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

    fn resize(&mut self, screensize: (f32, f32)) {
        self.cached_screensize = screensize;
        self.pos_rel = self.pos_abs.to_relative(screensize);
        self.rect_track = Rectangle::new(
            self.track_pos.to_relative(screensize).get_coordinates(),
            (0.58, 0.64, 0.7),
        );
        self.rect_slider = Rectangle::new(
            self.slider_pos.to_relative(screensize).get_coordinates(),
            (0.7, 0.75, 0.8),
        );
        self.label.resize(screensize);
    }
}

impl Drawable for Slider {
    fn draw_transformed(&self, view_matrix: &Matrix4<f32>) {
        self.rect_track.draw_transformed(view_matrix);
        self.rect_slider.draw_transformed(view_matrix);
        self.label.draw_transformed(view_matrix);
    }
}
