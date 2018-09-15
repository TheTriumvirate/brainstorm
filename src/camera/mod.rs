//! Contains various cameras and their projection matrices.
mod arcball;

pub use self::arcball::ArcBall;

use na::Matrix4;
use std::f32;
use window::Event;

/// Represents a camera in 3D space.
pub trait Camera {
    fn update(&mut self);
    fn handle_events(&mut self, event: &Event);
    fn get_projection_matrix(&self) -> Matrix4<f32>;
}
