#![cfg_attr(feature = "cargo-clippy", allow(cast_lossless))]

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
#[macro_use]
extern crate lazy_static;
extern crate nalgebra as na;

#[cfg(target_arch = "wasm32")]
pub mod webgl;
#[cfg(not(target_arch = "wasm32"))]
pub mod opengl;
pub mod shaders;
mod context;

#[cfg(not(target_arch = "wasm32"))]
pub use opengl::GLContext as Context;
#[cfg(target_arch = "wasm32")]
pub use webgl::WebGLContext as Context;

pub use context::*;
