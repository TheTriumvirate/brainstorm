use graphics::Drawable;
use State;

/// Defines the basic requirements of a UI element,
/// with default behaviour of not doing anything.
pub trait UiElement: Drawable {
    /// Returns whether or not a coordinate overlaps with the element.
    fn is_within(&self, _x: f64, _y: f64) -> bool {
        false
    }

    /// A method called every time the mouse moves.
    fn mouse_moved(&mut self, _x: f64, _y: f64, _state: &mut State) {}

    /// A method called whenever the user clicks and `is_within` is true.
    fn click(&mut self, _x: f64, _y: f64, _state: &mut State) {}

    /// A method called whenever the mouse-click is released.
    fn click_release(&mut self, _x: f64, _y: f64, _state: &mut State) {}

    /// Update element to match new screen size
    fn resize(&mut self, _screensize: (f32, f32)) {}
}
