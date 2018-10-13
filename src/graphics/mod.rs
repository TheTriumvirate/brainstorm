//! Holds various methods and frameworks for drawing,
//! in particular for drawing primitives.

mod circle;
pub mod position;
mod rectangle;
pub mod render_target;
mod text;
mod cube;

pub use self::circle::Circle;
pub use self::rectangle::Rectangle;
pub use self::text::font::Font;
pub use self::text::Text;
pub use self::cube::Cube;

use gl_context::{shaders::OurShader, Texture};

use std::rc::Rc;

use na::Matrix4;

pub enum DrawMode {
    TRIANGLES,
    LINES,
    LINE_STRIP,
}

/// Represents something that can be drawn.
pub trait Drawable {
    fn get_texture(&self) -> Option<Rc<Texture>> {
        None
    }
    fn get_shader(&self) -> Option<&OurShader> {
        None
    }

    fn get_transform(&self) -> Option<&Matrix4<f32>> {None}

    fn draw_transformed(&self, view_matrix: &Matrix4<f32>);

    fn draw(&self) {self.draw_transformed(&Matrix4::identity());}
}

pub struct RenderStates<'a> {
    pub texture: Option<Rc<Texture>>,
    pub shader: Option<&'a OurShader>,
    pub transform: Option<&'a Matrix4<f32>>,
}

impl<'a, T> From<&'a T> for RenderStates<'a>
where
    T: Drawable,
{
    fn from(drawable: &'a T) -> RenderStates {
        RenderStates {
            texture: drawable.get_texture(),
            shader: drawable.get_shader(),
            transform: drawable.get_transform(),
        }
    }
}
