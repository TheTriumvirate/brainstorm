use crate::camera::Camera;
use na::{Isometry3, Matrix4, Perspective3, Point3, Vector2, Vector3};
use std::f32;
use window::{Event, MouseButton};

/// A camera that orbits around a point in space.
pub struct ArcBall {
    target: Point3<f32>,
    yaw: f32,
    pitch: f32,
    distance: f32,
    projection: Matrix4<f32>,
    last_cursor_pos: Vector2<f32>,
    is_pressed: bool,
    idle: f32,
    aspect: f32,
}

impl ArcBall {
    /// Creates a new instance of ArcBallCamera.
    pub fn new() -> Self {
        let mut res = Self {
            target: Point3::new(0.0, 0.0, 0.0),
            yaw: f32::consts::PI / 2.0,
            pitch: f32::consts::PI / 2.0,
            distance: 5.0,
            projection: Matrix4::identity(),
            last_cursor_pos: Vector2::new(0.0, 0.0),
            is_pressed: false,
            idle: 0.0,
            aspect: 1.0,
        };
        res.recalculate_matrices();
        res
    }

    // Updates the internal state of the camera.
    fn recalculate_matrices(&mut self) {
        if self.distance < 0.00001 {
            self.distance = 0.00001;
        }

        if self.pitch < 0.01 {
            self.pitch = 0.01;
        }

        let _pi: f32 = f32::consts::PI;
        if self.pitch > _pi - 0.01 {
            self.pitch = _pi - 0.01
        }

        let ex = self.target.x + self.distance * self.yaw.cos() * self.pitch.sin();
        let ey = self.target.y + self.distance * self.pitch.cos();
        let ez = self.target.z + self.distance * self.yaw.sin() * self.pitch.sin();
        let eye = Point3::new(ex, ey, ez);
        let perspective = Perspective3::new(self.aspect, 1.0, 0.1, 1024.0);
        let view: Isometry3<f32> = Isometry3::look_at_rh(&eye, &self.target, &Vector3::y());
        self.projection = perspective.as_matrix() * view.to_homogeneous();
    }

    pub fn get_target(&self) -> (f32, f32, f32) {
        (self.target.x, self.target.y, self.target.z)
    }
}

impl Camera for ArcBall {
    /// Handles input to rotate the camera.
    /// TODO: Handle outside of camera and pass rotation in.
    fn handle_events(&mut self, event: &Event) {
        match event {
            Event::CursorMoved { x, y } => {
                // do stuff
                let mouse = Vector2::new(*x as f32, *y as f32);
                if self.is_pressed {
                    let dt = mouse - self.last_cursor_pos;
                    self.yaw += dt.x * 0.005;
                    self.pitch -= dt.y * 0.005;
                    self.recalculate_matrices();
                    self.idle = 0.0;
                }
                self.last_cursor_pos = mouse;
            }
            Event::CursorInput {
                button, pressed, ..
            } => {
                if *button == MouseButton::Left {
                    self.is_pressed = *pressed;
                }
            }
            Event::CursorScroll(_, dt) => {
                self.distance -= dt / 8.0;
                self.recalculate_matrices();
            }
            Event::Resized(w, h) => {
                self.aspect = w / h;
                self.recalculate_matrices();
            }
            _ => (),
        }
    }

    /// Gets the projection matrix for the camera.
    fn get_projection_matrix(&self) -> Matrix4<f32> {
        self.projection
    }

    /// Get the position of the camera.
    fn get_position(&self) -> (f32, f32, f32) {
        let ex = self.target.x + self.distance * self.yaw.cos() * self.pitch.sin();
        let ey = self.target.y + self.distance * self.pitch.cos();
        let ez = self.target.z + self.distance * self.yaw.sin() * self.pitch.sin();
        let eye = Point3::new(ex, ey, ez);
        (eye.x, eye.y, eye.z)
    }

    /// Set the target position of the camera.
    fn set_target_position(&mut self, (x, y, z): (f32, f32, f32)) {
        self.target.x = x;
        self.target.y = y;
        self.target.z = z;
        self.recalculate_matrices();
    }

    /// Move the camera by the specified amount.
    fn move_camera(&mut self, dx: f32, dy: f32, dz: f32) {
        let t = self.target;
        self.target = Point3::new(t.x + dx * 0.01, t.y + dy * 0.01, t.z + dz * 0.01);
        self.recalculate_matrices();
    }
}

impl Default for ArcBall {
    fn default() -> Self {
        Self::new()
    }
}
