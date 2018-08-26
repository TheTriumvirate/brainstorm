use super::super::State;

pub struct Button {
    pub x1: f64,
    pub x2: f64,
    pub y1: f64,
    pub y2: f64,
    pub color: (u8, u8, u8),
    pub func: Box<dyn FnMut(&mut State)>,
}

impl Button {
    pub fn was_clicked(&self, x: f64, y: f64) -> bool {
        self.x1 < x && x < self.x2 && self.y1 < y && y < self.y2
    }

    pub fn click(&mut self, state: &mut State) {
        let func = &mut self.func;
        func(state);
    }
}
