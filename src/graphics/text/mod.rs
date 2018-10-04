use graphics::Drawable;

pub mod font;
use self::font::Font;

use std::cell::RefCell;
use std::rc::Rc;

use gl_context::{shaders::OurShader, Buffer, BufferType, Texture};

use graphics::*;

pub struct Text<'a> {
    text: String,
    font: Rc<RefCell<Font<'a>>>,
    vertices: Buffer<f32>,
    indices: Buffer<u16>,
}

impl<'a> Drawable for Text<'a> {
    fn get_texture(&self) -> Option<Rc<Texture>> {
        let texture = self.font.borrow().get_texture();
        Some(texture)
        //None
    }

    fn get_shader(&self) -> Option<&OurShader> {
        Some(self.font.borrow().get_shader())
    }

    fn draw_transformed(&self, view_matrix: &Matrix4<f32>) {
        render_target::draw_indices(DrawMode::TRIANGLES, &self.vertices, &self.indices, RenderStates::from(self), view_matrix);
    }
}

impl<'a> Text<'a> {
    pub fn new(text: String, font: Rc<RefCell<Font<'a>>>, x: f32, y: f32) -> Self {
        let vertices: Buffer<f32> = Buffer::new(BufferType::Array);
        let indices: Buffer<u16> = Buffer::new(BufferType::IndexArray);

        let mut t = Text {
            text,
            font: font.clone(),
            vertices,
            indices,
        };

        t.font
            .borrow_mut()
            .update_texture(&t.text, x, y, &mut t.vertices, &mut t.indices);

        t
    }

    // TODO: DIRTY
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.font.borrow_mut().update_texture(
            &self.text,
            x,
            y,
            &mut self.vertices,
            &mut self.indices,
        );
    }
}
