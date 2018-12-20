use std::{cell::RefCell, rc::Rc};

use crate::graphics::{Drawable, Font, Text, Circle};
use crate::gui::UiElement;
use na::{Matrix4, Isometry3, Vector3, Point3, Translation3, Vector4};
use gl_bindings::{Context, AbstractContext};
use crate::State;

use std::f32;

struct WorldPoint {
    text: Text<'static>,
    dot: Circle,
    hovered: bool,    
}

/// A simple button that can be pressed.
pub struct WorldPoints {
    screensize: (f32, f32),
    points: Vec<WorldPoint>,
    font: Rc<RefCell<Font<'static>>>,
    camera_pos: (f32, f32, f32),
    target_pos: (f32, f32, f32),
    view_matrix: Matrix4<f32>,   
}

impl WorldPoints {
    /// Creates a new button.
    pub fn new(
        screensize: (f32, f32),
        font: Rc<RefCell<Font<'static>>>,
    ) -> Self {

        Self {
            screensize,
            points: Vec::new(),// ,
            font,
            camera_pos: (0.0, 0.0, 0.0),
            target_pos: (0.0, 0.0, 0.0),
            view_matrix: Matrix4::<f32>::identity(),
        }
    }

    pub fn set_points(&mut self, points: Vec<(f32, f32, f32)>) {
        self.points.clear();

        self.points.push(WorldPoint {
            text: Text::new("Center".to_owned(), self.font.clone(), 0.0, 0.0 + 0.03, 0.0, self.screensize),
            dot: Circle::new(0.0, 0.0, 0.0, 0.005, f32::consts::PI / 2.0, (0.0,1.0,0.0), true),
            hovered: false,
        });

        let mut id = 1;
        for point in points {
            let (x, y, z) = point;
            self.points.push(WorldPoint {
                text: Text::new(format!("point {}", id).to_owned(), self.font.clone(), x, y + 0.03, z, self.screensize),
                dot: Circle::new(x, y, z, 0.005, f32::consts::PI / 2.0, (0.0,1.0,0.0), true),
                hovered: false,
            });
            id += 1;
        }
    }

    pub fn set_camera_pos(&mut self, pos: (f32, f32, f32)) {
        self.camera_pos = pos;
    }

    pub fn set_camera_target_pos(&mut self, pos: (f32, f32, f32)) {
        self.target_pos = pos;
    }

    pub fn set_view_matrix(&mut self, view_matrix: Matrix4<f32>) {
        self.view_matrix = view_matrix;
    }
}

impl UiElement for WorldPoints {
    fn resize(&mut self, _screensize: (f32, f32)) {
        //let text_coords = self.pos.to_relative(screensize).get_coordinates();
        //self.text.set_position(text_coords.x1, text_coords.y1, screensize);
    }
    
    fn is_within(&self, x: f64, y: f64) -> bool {
        for point in &self.points {
            let (cx, cy, cz) = point.dot.get_center();
            let screen_pos = self.view_matrix * Vector4::new(cx, cy, cz, 1.0);

            let dx = screen_pos.x / screen_pos.w - x as f32;
            let dy = screen_pos.y / screen_pos.w - y as f32;

            if dx * dx + dy * dy < 0.02 * 0.02 {
                return true;
            }
        }

        false
    }
    
    fn click(&mut self, x: f64, y: f64, state: &mut State) {
        for point in &self.points {
            let (cx, cy, cz) = point.dot.get_center();
            let screen_pos = self.view_matrix * Vector4::new(cx, cy, cz, 1.0);

            let dx = screen_pos.x / screen_pos.w - x as f32;
            let dy = screen_pos.y / screen_pos.w - y as f32;

            if dx * dx + dy * dy < 0.02 * 0.02 {
                state.camera_target = point.dot.get_center();
                return;
            }
        }
    }

    
    fn mouse_moved(&mut self, x: f64, y: f64, _: &mut State) {
        for point in &mut self.points {
            point.hovered = false;
        }
        for point in &mut self.points {
            let (cx, cy, cz) = point.dot.get_center();
            let screen_pos = self.view_matrix * Vector4::new(cx, cy, cz, 1.0);

            let dx = screen_pos.x / screen_pos.w - x as f32;
            let dy = screen_pos.y / screen_pos.w - y as f32;

            if dx * dx + dy * dy < 0.02 * 0.02 {
                point.hovered = true;
                return;
            }
        }
    
    }
}

impl Drawable for WorldPoints {
    fn draw_transformed(&self, view_matrix: &Matrix4<f32>) {
        let (tx, ty, tz) = self.target_pos;
        
        let (px, py, pz) = self.camera_pos;
        const ZOOM_FACTOR : f32 = 2.0;
        Context::get_context().disable(Context::DEPTH_TEST);
        for point in &self.points {
            let hover_factor = if point.hovered {
                0.3
            } else {
                1.0
            };
            
            let scale = Matrix4::new_orthographic(-ZOOM_FACTOR - hover_factor,ZOOM_FACTOR + hover_factor,-ZOOM_FACTOR - hover_factor,ZOOM_FACTOR + hover_factor,ZOOM_FACTOR + hover_factor,-ZOOM_FACTOR - hover_factor);
            let (ex, ey, ez) = point.text.get_position();
            let (cx, cy, cz) = point.text.get_center();
            let target: Point3<f32> = Point3::new(0.0, 0.0, 0.0);
            let eye = Point3::new(px - tx, py - ty, pz - tz);
            let view: Isometry3<f32> = Isometry3::look_at_lh(&target, &eye, &Vector3::y());
            let trans = Translation3::new(-cx, -cy, -cz);
            let trans2 = Translation3::new(ex, ey + (1.0 - hover_factor) / 50.0, ez);
            let projection = view_matrix * trans2.to_homogeneous() * scale * view.inverse().to_homogeneous() * trans.to_homogeneous();
            point.text.draw_transformed(&projection);
        }

        for point in &self.points {
            
            let hover_factor = if point.hovered {
                0.3
            } else {
                1.0
            };
            
            let scale = Matrix4::new_orthographic(-hover_factor,hover_factor,- hover_factor,hover_factor,hover_factor, - hover_factor);
            
            let (cx, cy, cz) = point.dot.get_center();
            let target: Point3<f32> = Point3::new(0.0, 0.0, 0.0);
            let eye = Point3::new(px - tx, py - ty, pz - tz);
            let view: Isometry3<f32> = Isometry3::look_at_lh(&target, &eye, &Vector3::y());
            let trans = Translation3::new(-cx, -cy, -cz);
            let trans2 = Translation3::new(cx, cy, cz);
            let projection = view_matrix * trans2.to_homogeneous() * scale * view.inverse().to_homogeneous() * trans.to_homogeneous();

            point.dot.draw_transformed(&projection);
            
        }
        Context::get_context().enable(Context::DEPTH_TEST);

    }
}
