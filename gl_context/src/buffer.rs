use std::ops::{Index,IndexMut};

use context::GlPrimitive;
use AbstractContext;
use Context;
use NativeBuffer;

/// Represents the two GL buffer types.
pub enum BufferType {
    Array,
    IndexArray,
}

impl BufferType {
    /// Returns the GL value of the type.
    fn gl_type(&self) -> u32 {
        match *self {
            BufferType::Array => Context::ARRAY_BUFFER,
            BufferType::IndexArray => Context::ELEMENT_ARRAY_BUFFER,
        } 
    }
}

/// Holds a GL buffer and lets you upload it to the GPU.
pub struct Buffer<T: Clone+GlPrimitive> {
    buffer: NativeBuffer,
    buffer_type: BufferType,
    data: Vec<T>,
}

impl<T: Clone+GlPrimitive> Buffer<T> {
    /// Creates a new buffor of the selected type.
    pub fn new(buffer_type: BufferType) -> Self {
        let context = Context::get_context();
        
        let buffer = context.create_buffer().expect("Failed to create buffer");

        let data : Vec<T> = Vec::new();
        // assume static until update
        Buffer {buffer, buffer_type, data}
    }

    /// Resizes the buffer to the requested size.
    pub fn resize(&mut self, size: usize, default: T) {
        self.data.resize(size, default)
    }

    /// Gets the size of the buffer.
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    /// Returns whether or not the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Sets the data of the buffer.
    pub fn set_data(&mut self, data: &[T]) {
        self.data = data.to_vec()
    }

    /// Uploads the data to the GPU.
    pub fn upload_data(&mut self, offset: usize, length: usize, is_static: bool) {
        let alloc_type = if is_static {
            Context::STATIC_DRAW
        } else {
            Context::DYNAMIC_DRAW
        };

        let context = Context::get_context();
        context.buffer_data(self.buffer_type.gl_type(), &self.data[offset..length], alloc_type);
    }

    /// Binds the buffer to the GPU.
    pub fn bind(&self) {
        let context = Context::get_context();
        context.bind_buffer(self.buffer_type.gl_type(), &self.buffer);
    }
}

impl<T: Clone+GlPrimitive> Index<usize> for Buffer<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.data[index]
    }
}

impl<T: Clone+GlPrimitive> IndexMut<usize> for Buffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.data[index]
    }
}

impl<T: Clone+GlPrimitive> Drop for Buffer<T> {
    fn drop(&mut self) {
        let context = Context::get_context();
        context.delete_buffer(&self.buffer);
    }
}
