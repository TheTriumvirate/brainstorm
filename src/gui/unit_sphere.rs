use na::Matrix4;

use crate::graphics::{Circle, Drawable};
use crate::gui::UiElement;
use gl_bindings::{AbstractContext, Context};

/// A 3D
pub struct UnitSphere {
    circles: [Circle; 3],
}

impl UnitSphere {
    /// Creates a new Misc.
    pub fn new(target: (f32, f32, f32), size: f32) -> Self {
        let mut circle1 = Circle::new(0.0, 0.0, 0.0, 0.5, 0.0, (1.0, 0.0, 0.0), false);
        let mut circle2 = Circle::new(0.0, 0.0, 0.0, 0.5, 0.0, (0.0, 1.0, 0.0), false);
        let mut circle3 = Circle::new(0.0, 0.0, 0.0, 0.5, 0.0, (0.0, 0.0, 1.0), false);
        circle1.set_color(1.0, 0.0, 0.0);
        circle2.set_color(0.0, 1.0, 0.0);
        circle3.set_color(0.0, 0.0, 1.0);
        let mut circles = [circle1, circle2, circle3];
        for circle in circles.iter_mut() {
            circle.set_center(target);
            circle.set_radius(size);
            circle.rebuild_data();
        }
        Self { circles }
    }

    /// Change the centre/target of the sphere.
    pub fn retarget(&mut self, target: (f32, f32, f32)) {
        for circle in self.circles.iter_mut() {
            circle.set_center(target);
            circle.rebuild_data();
        }
    }

    /// Change the size of the sphere.
    pub fn resize(&mut self, size: f32) {
        let size = size * 0.6 + 0.01;
        for circle in self.circles.iter_mut() {
            circle.set_radius(size);
            circle.rebuild_data();
        }
    }
}

impl UiElement for UnitSphere {}

impl Drawable for UnitSphere {
    fn draw_transformed(&self, view_matrix: &Matrix4<f32>) {
        Context::get_context().enable(Context::DEPTH_TEST);
        for circle in self.circles.iter() {
            circle.draw_transformed(view_matrix);
        }
        Context::get_context().disable(Context::DEPTH_TEST);
    }
}
