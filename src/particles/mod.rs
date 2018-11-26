//! Module containing all particle-system related code.

pub mod fieldprovider;
pub mod gpu_fieldprovider;
pub mod gpu_particles;

pub enum EngineType {
    CPU,
    GPU,
}

pub enum FieldProviderType {
    CPU(fieldprovider::FieldProvider),
    GPU(gpu_fieldprovider::GPUFieldProvider),
}

pub type Vector4 = (f32, f32, f32, f32);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VectorField {
    width: usize,
    height: usize,
    depth: usize,
    vectors: Vec<Vec<Vec<Vector4>>>,
    directional: Vec<(f32, f32, f32)>,
}

mod particle_engine;
pub use self::particle_engine::ParticleEngine;

mod marching_cubes;
pub use self::marching_cubes::MarchingCubes;

mod streamlines;
pub use self::streamlines::Streamlines;
