#![cfg_attr(feature = "cargo-clippy", allow(cast_lossless))]
extern crate noise;

#[macro_use]
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
extern crate stdweb;
#[macro_use]
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
extern crate stdweb_derive;
#[macro_use]
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
extern crate serde_derive;
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
extern crate serde;

pub mod shaders;
pub mod window;

use noise::{NoiseFn, RidgedMulti};
use std::f32;

/// Gets the new position for a particle at a given position.
/// Assumes normalized vector return.
pub fn field((x, y): (f32, f32), noise: &RidgedMulti, time: f32) -> (f32, f32) {
    let angle = noise.get([x as f64, y as f64, time as f64]) as f32 * f32::consts::PI * 2.0;
    let strength = noise
        .get([x as f64 + 4000.0, y as f64 + 2000.0, time as f64])
        .abs() as f32 * 9.0 + 1.0;
    (
        angle.cos() * strength,
        angle.sin() * strength + 20.0 * (y.abs() + 0.5),
    )
}
