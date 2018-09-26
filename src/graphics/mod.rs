//! Holds various methods and frameworks for drawing,
//! in particular for drawing primitives.

mod circle;
mod rectangle;
mod text;
pub mod position;
pub mod render_target;

pub use self::circle::Circle;
pub use self::rectangle::Rectangle;
pub use self::text::Text;
pub use self::text::font::Font;

use gl_context::{Texture, shaders::OurShader};

/// Represents something that can be drawn.
pub trait Drawable {
    fn get_texture(&self) -> Option<&Texture> {None}
    fn get_shader(&self) -> Option<&OurShader> {None}
    fn draw(&self);
}

pub struct RenderStates<'a> {
    pub texture: Option<&'a Texture>,
}

impl<'a, T:'a> From<&'a T> for RenderStates<'a> where T: Drawable {
    fn from(drawable: &'a T) -> RenderStates<'a> {
        RenderStates {
            texture: drawable.get_texture()
        }
    }    
}