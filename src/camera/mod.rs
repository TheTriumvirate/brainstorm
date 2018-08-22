use window::Event;
use window::MouseButton;
use na::{Matrix4, Point3, Vector3, Perspective3, Isometry3, Vector2};
use std::f32;

pub trait Camera {
    fn update(&mut self);
    fn handle_events(&mut self, event: &Event);
    fn get_projection_matrix(&self) -> Matrix4<f32>;
}

pub struct ArcBallCamera {
    target: Point3<f32>,
    yaw: f32,
    pitch: f32,
    distance: f32,
    projection: Matrix4<f32>,
    last_cursor_pos: Vector2<f32>,
    is_pressed: bool
}

impl ArcBallCamera {
    pub fn new() -> ArcBallCamera {
        let mut res = ArcBallCamera {
            target: Point3::new(0.0, 0.0, 0.0),
            yaw: f32::consts::PI / 2.0,
            pitch: f32::consts::PI / 2.0,
            distance: 5.0,
            projection: Matrix4::identity(),
            last_cursor_pos: Vector2::new(0.0, 0.0),
            is_pressed: false
        };
        res.recalculate_matrices();
        res
    }
    
    fn recalculate_matrices(&mut self) {
        let ex = self.target.x + self.distance * self.yaw.cos() * self.pitch.sin();
        let ey = self.target.y + self.distance * self.pitch.cos();
        let ez = self.target.z + self.distance * self.yaw.sin() * self.pitch.sin();
        let eye = Point3::new(ex, ey, ez);
        let perspective = Perspective3::new(1.0, f32::consts::PI / 4.0, 0.1, 1024.0);
        let view: Isometry3<f32> = Isometry3::look_at_rh(&eye, &self.target, &Vector3::y());
        self.projection = perspective.as_matrix() * view.to_homogeneous();
    }
}

impl Camera for ArcBallCamera {

    fn update(&mut self) {

    }

    fn handle_events(&mut self, event: &Event) {
        match event {
            Event::CursorMoved {x, y} => {
                // do stuff
                let mouse = Vector2::new(*x as f32, *y as f32);
                if self.is_pressed {
                    let dt = mouse - self.last_cursor_pos;
                    self.yaw = self.yaw + dt.x * 0.005;
                    self.pitch = self.pitch - dt.y * 0.005;
                    self.recalculate_matrices();
                }
                self.last_cursor_pos = mouse;
            },
            Event::CursorInput {button, pressed, ..} => {
                if *button == MouseButton::Left {
                    self.is_pressed = *pressed;
                }
            },
            Event::CursorScroll(_, dt) => {
                self.distance += dt;
                self.recalculate_matrices();
            }
            _ => ()
        }
    }

    fn get_projection_matrix(&self) -> Matrix4<f32> {
        self.projection
    }
}

/*

        
        let perspective = Perspective3::new(1.0, f32::consts::PI / 4.0, 0.1, 1024.0);
        let eye = Point3::new(self.time.sin() * 2.0, 0.75, self.time.cos() * 2.0);
        let at = Point3::new(0.0, 0.0, 0.0);
        let view: Isometry3<f32> = Isometry3::look_at_rh(&eye, &at, &Vector3::y());
        let projection_matrix = perspective.as_matrix() * view.to_homogeneous();

        */