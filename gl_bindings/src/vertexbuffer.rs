use AbstractContext;
use Context;
use NativeVertexBuffer;
use Texture;


/// Holds a GL buffer and lets you upload it to the GPU.
pub struct VertexBuffer {
    buffer: NativeVertexBuffer
}

impl VertexBuffer {
    /// Creates a new buffor of the selected type.
    pub fn new() -> Self {
        let context = Context::get_context();
        
        let buffer = context.create_vertexbuffer().expect("Failed to create frame buffer");

        VertexBuffer {buffer}
    }

    /// Binds the buffer to the GPU.
    pub fn bind(&self) {
        let context = Context::get_context();
        context.bind_vertexbuffer(Some(&self.buffer));
    }

    pub fn unbind(&self) {
        let context = Context::get_context();
        context.bind_vertexbuffer(None);

    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        let context = Context::get_context();
        context.delete_vertexbuffer(&self.buffer);
    }
}
