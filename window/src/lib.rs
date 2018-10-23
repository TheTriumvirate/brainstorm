#[cfg(not(target_arch = "wasm32"))]
extern crate glutin;

#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;

#[cfg(target_arch = "wasm32")]
extern crate gl_bindings;

pub mod abstract_window;
mod events;
#[cfg(not(target_arch = "wasm32"))]
mod opengl;
#[cfg(target_arch = "wasm32")]
mod webgl;

pub use self::abstract_window::AbstractWindow;
pub use self::events::*;

#[cfg(target_arch = "wasm32")]
pub use self::webgl::WebGLWindow as Window;

#[cfg(not(target_arch = "wasm32"))]
pub use self::opengl::GLWindow as Window;