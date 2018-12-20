//! Contains various cameras and their projection matrices.
mod arcball;

pub use self::arcball::ArcBall;

use na::Matrix4;
use std::f32;
use window::Event;

/// Represents a camera in 3D space.
pub trait Camera {
    fn get_position(&self) -> (f32, f32, f32);
    fn handle_events(&mut self, event: &Event);
    fn get_projection_matrix(&self) -> Matrix4<f32>;
    fn move_camera(&mut self, dx: f32, dy: f32, dz: f32);
    fn set_target_position(&mut self, p: (f32, f32, f32));
}
