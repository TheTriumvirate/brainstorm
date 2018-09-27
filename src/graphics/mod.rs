//! Holds various methods and frameworks for drawing,
//! in particular for drawing primitives.

mod circle;
pub mod position;
mod rectangle;
pub mod render_target;
mod text;

pub use self::circle::Circle;
pub use self::rectangle::Rectangle;
pub use self::text::font::Font;
pub use self::text::Text;

use gl_context::{shaders::OurShader, Texture};

use std::rc::Rc;

/// Represents something that can be drawn.
pub trait Drawable {
    fn get_texture(&self) -> Option<Rc<Texture>> {
        None
    }
    fn get_shader(&self) -> Option<&OurShader> {
        None
    }
    fn draw(&self);
}

pub struct RenderStates<'a> {
    pub texture: Option<Rc<Texture>>,
    pub shader: Option<&'a OurShader>,
}

impl<'a, T> From<&'a T> for RenderStates<'a>
where
    T: Drawable,
{
    fn from(drawable: &'a T) -> RenderStates {
        RenderStates {
            texture: drawable.get_texture(),
            shader: drawable.get_shader(),
        }
    }
}
