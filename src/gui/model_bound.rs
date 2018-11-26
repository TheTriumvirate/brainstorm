use na::Matrix4;

use gl_bindings::{AbstractContext, Context};
use graphics::{Cube, Drawable};
use gui::UiElement;

pub struct ModelBound {
    bound: Cube,
}

impl ModelBound {
    /// Creates a new Misc.
    pub fn new() -> Self {
        Self {
            bound: Cube::new((-0.5, -0.5, -0.5), (1.0, 1.0, 1.0), (1.0, 1.0, 1.0)),
        }
    }
}

impl UiElement for ModelBound {}

impl Drawable for ModelBound {
    fn draw_transformed(&self, view_matrix: &Matrix4<f32>) {
        Context::get_context().enable(Context::DEPTH_TEST);
        self.bound.draw_transformed(view_matrix);
        Context::get_context().disable(Context::DEPTH_TEST);
    }
}
