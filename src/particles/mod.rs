//! Module containing all particle-system related code.

use serde_derive::{Deserialize, Serialize};

pub mod fieldprovider;
pub mod gpu_fieldprovider;
pub mod gpu_particles;
mod marching_cubes;
mod particle_engine;

pub type Vector4 = (f32, f32, f32, f32);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VectorField {
    width: usize,
    height: usize,
    depth: usize,
    vectors: Vec<Vec<Vec<Vector4>>>,
    directional: Vec<(f32, f32, f32)>,
}

pub use self::marching_cubes::MarchingCubes;
pub use self::particle_engine::ParticleEngine;
