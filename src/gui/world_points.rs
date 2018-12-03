use std::{cell::RefCell, rc::Rc};

use gl_bindings::{AbstractContext, Context};
use graphics::{Circle, Drawable, Font, Text};
use gui::UiElement;
use na::{Isometry3, Matrix4, Point3, Translation3, Vector3};

use std::f32;

/// A simple button that can be pressed.
pub struct WorldPoints {
    screensize: (f32, f32),
    texts: Vec<Text<'static>>,
    dots: Vec<Circle>,
    font: Rc<RefCell<Font<'static>>>,
    camera_pos: (f32, f32, f32),
    dir: f32,
}

impl WorldPoints {
    /// Creates a new button.
    pub fn new(screensize: (f32, f32), font: Rc<RefCell<Font<'static>>>) -> Self {
        Self {
            screensize,
            texts: Vec::new(), // ,
            dots: Vec::new(),
            font: font.clone(),
            camera_pos: (0.0, 0.0, 0.0),
            dir: 0.0,
        }
    }

    pub fn set_points(&mut self, points: Vec<(f32, f32, f32)>) {
        self.texts.clear();
        self.dots.clear();

        let mut id = 1;
        for point in points {
            let (x, y, z) = point;
            self.texts.push(Text::new(
                format!("point {}", id).to_owned(),
                self.font.clone(),
                x,
                y + 0.03,
                z,
                self.screensize,
            ));
            self.dots.push(Circle::new(
                x,
                y,
                z,
                0.005,
                3.1415 / 2.0,
                (0.0, 1.0, 0.0),
                true,
            ));
            id += 1;
        }
    }

    pub fn set_camera_pos(&mut self, pos: (f32, f32, f32)) {
        self.camera_pos = pos;
        self.dir += 0.1;
    }
}

impl UiElement for WorldPoints {
    fn resize(&mut self, _screensize: (f32, f32)) {
        //let text_coords = self.pos.to_relative(screensize).get_coordinates();
        //self.text.set_position(text_coords.x1, text_coords.y1, screensize);
    }
}

impl Drawable for WorldPoints {
    fn draw_transformed(&self, view_matrix: &Matrix4<f32>) {
        let (ex, ey, ez) = self.camera_pos;
        let eye = Point3::new(ex, ey, ez);
        const ZOOM_FACTOR: f32 = 3.0;
        let scale = Matrix4::new_orthographic(
            -ZOOM_FACTOR,
            ZOOM_FACTOR,
            -ZOOM_FACTOR,
            ZOOM_FACTOR,
            ZOOM_FACTOR,
            -ZOOM_FACTOR,
        );
        Context::get_context().disable(Context::DEPTH_TEST);
        for text in &self.texts {
            let (ex, ey, ez) = text.get_position();
            let (cx, cy, cz) = text.get_center();
            let target: Point3<f32> = Point3::new(0.0, 0.0, 0.0);
            let view: Isometry3<f32> = Isometry3::look_at_lh(&target, &eye, &Vector3::y());
            let trans = Translation3::new(-cx, -cy, -cz);
            let trans2 = Translation3::new(ex, ey, ez);
            let projection = view_matrix
                * trans2.to_homogeneous()
                * scale
                * view.inverse().to_homogeneous()
                * trans.to_homogeneous();
            text.draw_transformed(&projection);
        }

        for dot in &self.dots {
            let (cx, cy, cz) = dot.get_center();
            let target: Point3<f32> = Point3::new(0.0, 0.0, 0.0);
            let view: Isometry3<f32> = Isometry3::look_at_lh(&target, &eye, &Vector3::y());
            let trans = Translation3::new(-cx, -cy, -cz);
            let trans2 = Translation3::new(cx, cy, cz);
            let projection = view_matrix
                * trans2.to_homogeneous()
                * view.inverse().to_homogeneous()
                * trans.to_homogeneous();
            dot.draw_transformed(&projection);
        }
        Context::get_context().enable(Context::DEPTH_TEST);
    }
}
