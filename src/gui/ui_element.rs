use graphics::Drawable;
use State;

pub trait UiElement : Drawable {
    fn was_clicked(&self, _x: f64, _y: f64) -> bool { false }
    fn click(&mut self, _state: &mut State) { }
    fn click_release(&mut self, _state: &mut State) { }
}
