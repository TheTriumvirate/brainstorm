/*
 * Very inspired from kiss3d's implementation of window and context
 * link: https://github.com/sebcrozet/kiss3d
 */
//pub use self::window::Window;

pub use self::abstract_window::*;
pub use self::events::*;

#[cfg(target_arch = "wasm32")]
pub use self::webgl::WebGLWindow as Window;

#[cfg(not(target_arch = "wasm32"))]
pub use self::opengl::GLWindow as Window;

pub mod abstract_window;

#[cfg(target_arch = "wasm32")]
mod webgl;

#[cfg(not(target_arch = "wasm32"))]
mod opengl;

mod events;
