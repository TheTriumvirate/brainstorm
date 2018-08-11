//! Very inspired from kiss3d's implementation of window and context
//! link: https://github.com/sebcrozet/kiss3d

pub use self::webgl_window::WebGLWindow;

#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
mod webgl_bindings;

mod webgl_window;
