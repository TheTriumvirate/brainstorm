//! Provides a seamless wrapper around OpenGL and WebGL, so that the rest of
//! the code doesn't need to know which of the two it's running on.

#![cfg_attr(target_arch = "wasm32", feature(extern_prelude))]
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
extern crate image;

#[cfg(target_arch = "wasm32")]
pub mod webgl;
#[cfg(not(target_arch = "wasm32"))]
pub mod opengl;
pub mod shaders;
mod context;
mod buffer;
mod texture;

#[cfg(not(target_arch = "wasm32"))]
pub use opengl::GLContext as Context;
#[cfg(target_arch = "wasm32")]
pub use webgl::WebGLContext as Context;

pub use context::Buffer as NativeBuffer;
pub use context::Texture as NativeTexture;
pub use context::*;
pub use buffer::Buffer;
pub use buffer::BufferType;
pub use texture::Texture;
pub use texture::TextureFormat;