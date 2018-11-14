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

mod particle_engine;
pub use self::particle_engine::ParticleEngine;

mod marching_cubes;
pub use self::marching_cubes::MarchingCubes;

mod streamlines;
pub use self::streamlines::Streamlines;