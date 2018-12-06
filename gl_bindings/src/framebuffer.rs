use crate::AbstractContext;
use crate::Context;
use crate::NativeFrameBuffer;
use crate::Texture;


/// Holds a GL buffer and lets you upload it to the GPU.
pub struct FrameBuffer {
    buffer: NativeFrameBuffer
}

impl FrameBuffer {
    /// Creates a new buffor of the selected type.
    pub fn new() -> Self {
        let context = Context::get_context();
        
        let buffer = context.create_framebuffer().expect("Failed to create frame buffer");

        FrameBuffer {buffer}
    }

    /// Binds the buffer to the GPU.
    pub fn bind(&self) {
        let context = Context::get_context();
        context.bind_framebuffer(Context::FRAMEBUFFER, Some(&self.buffer));
    }

    pub fn unbind(&self) {
        let context = Context::get_context();
        context.bind_framebuffer(Context::FRAMEBUFFER, None);

    }

    pub fn buffer_texture_layer(&self, texture: &Texture, layer: i32) {
        let context = Context::get_context();
        texture.bind();
        context.framebuffer_texture_layer(Context::FRAMEBUFFER, Context::COLOR_ATTACHMENT0, &texture.get_native(), 0, layer);
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        let context = Context::get_context();
        context.delete_framebuffer(&self.buffer);
    }
}
