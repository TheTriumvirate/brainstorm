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

use std::rc::Rc;

/// Represents something that can be drawn.
pub trait Drawable {
    fn get_texture(&self) -> Option<Rc<Texture>> {None}
    fn get_shader(&self) -> Option<&OurShader> {None}
    fn draw(&self);
}

pub struct RenderStates {
    pub texture: Option<Rc<Texture>>,
}

impl<'a, T> From<&'a T> for RenderStates where T: Drawable {
    fn from(drawable: &'a T) -> RenderStates {
        RenderStates {
            texture: drawable.get_texture()
        }
    }    
}