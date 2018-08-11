/*
 * Very inspired from kiss3d's implementation of window and context
 * link: https://github.com/sebcrozet/kiss3d
 */
//pub use self::window::Window;

pub use self::abstract_window::*;

#[cfg(any(target_arch = "wasm32"))]
pub use self::webgl::WebGLWindow as Window;

#[cfg(not(any(target_arch = "wasm32")))]
pub use self::opengl::GLWindow as Window;
//use self::gl_window::GLWindow;

mod abstract_window;
//mod gl_window;

#[cfg(any(target_arch = "wasm32"))]
mod webgl;


#[cfg(not(any(target_arch = "wasm32")))]
mod opengl;
