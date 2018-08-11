//! Contains the shaders for rendering.

/// Vertex shader
pub const VERTEX_SHADER: &[u8] = include_bytes!("vertex.glslv");

/// Fragment shader
pub const FRAGMENT_SHADER: &[u8] = include_bytes!("fragment.glslf");
