#![cfg_attr(feature = "cargo-clippy", allow(cast_lossless))]
extern crate noise;

#[macro_use]
#[cfg(target_arch = "wasm32")]
extern crate stdweb;
#[macro_use]
#[cfg(target_arch = "wasm32")]
extern crate stdweb_derive;
#[macro_use]
#[cfg(target_arch = "wasm32")]
extern crate serde_derive;
#[cfg(target_arch = "wasm32")]
extern crate serde;

extern crate nalgebra as na;

pub mod shaders;
pub mod window;

use noise::{NoiseFn, RidgedMulti};
use std::f32;

/// Gets the new position for a particle at a given position.
/// Assumes normalized vector return.
pub fn field((x, y, z): (f32, f32, f32), noise: &RidgedMulti, time: f32) -> (f32, f32, f32) {
   /* let angle = noise.get([x as f64, y as f64, z as f64, time as f64]) as f32 * f32::consts::PI * 2.0;
    let strength = noise
        .get([x as f64 + 4000.0, y as f64 + 2000.0, z as f64 + 8000.0, time as f64])
        .abs() as f32 * 9.0 + 1.0;*/
        let sqrt = (z*z + x*x).sqrt();
        let strength = 4.0;
    (
        -z * strength * 5.0,
        strength * (y.abs() + 1.0),
        x * strength * 5.0
    )
}
