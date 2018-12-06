//! Provides a seamless wrapper around OpenGL and WebGL, so that the rest of
//! the code doesn't need to know which of the two it's running on.

#[cfg(target_arch = "wasm32")]
pub mod webgl;
#[cfg(not(target_arch = "wasm32"))]
pub mod opengl;
pub mod shaders;
mod context;
mod buffer;
mod texture;
mod framebuffer;
mod vertexbuffer;

#[cfg(not(target_arch = "wasm32"))]
pub use crate::opengl::GLContext as Context;
#[cfg(target_arch = "wasm32")]
pub use crate::webgl::WebGLContext as Context;

pub use crate::context::Buffer as NativeBuffer;
pub use crate::context::Texture as NativeTexture;
pub use crate::context::FrameBuffer as NativeFrameBuffer;
pub use crate::context::VertexArray as NativeVertexBuffer;
pub use crate::context::{Program, Shader, AbstractContext, GlPrimitive, UniformLocation};
pub use crate::buffer::Buffer;
pub use crate::buffer::BufferType;
pub use crate::texture::Texture;
pub use crate::texture::TextureFormat;
pub use crate::framebuffer::FrameBuffer;
pub use crate::vertexbuffer::VertexBuffer;