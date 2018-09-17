//! Holds various methods and frameworks for drawing,
//! in particular for drawing primitives.

mod circle;
mod rectangle;
pub mod position;
pub mod render_target;

pub use self::circle::Circle;
pub use self::rectangle::Rectangle;

/// Represents something that can be drawn.
pub trait Drawable {
    fn draw(&self);
}
