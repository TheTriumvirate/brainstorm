use graphics::Drawable;
use State;

pub trait UiElement: Drawable {
    fn is_within(&self, _x: f64, _y: f64) -> bool {
        false
    }

    fn mouse_over(&mut self, _x: f64, _y: f64, _state: &mut State) {}

    fn click(&mut self, _x: f64, _y: f64, _state: &mut State) {}

    fn click_release(&mut self, _x: f64, _y: f64, _state: &mut State) {}
}
