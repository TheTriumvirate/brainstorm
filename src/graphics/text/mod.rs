pub mod font;
use self::font::Font;

use std::cell::RefCell;
use std::rc::Rc;

use gl_bindings::{shaders::OurShader, Buffer, BufferType, Texture};

use graphics::{render_target, DrawMode, Drawable};
use na::Matrix4;

pub struct Text<'a> {
    text: String,
    font: Rc<RefCell<Font<'a>>>,
    vertices: Buffer<f32>,
    indices: Buffer<u16>,
    x: f32,
    y: f32,
    z: f32,
    width: f32,
    height: f32,
    screen_size: (f32, f32),
}

impl<'a> Drawable for Text<'a> {
    fn get_texture(&self) -> Option<Rc<Texture>> {
        let texture = self.font.borrow().get_texture();
        Some(texture)
        //None
    }

    fn get_shader(&self) -> Option<Rc<OurShader>> {
        Some(self.font.borrow().get_shader())
    }

    fn draw_transformed(&self, view_matrix: &Matrix4<f32>) {
        render_target::draw_indices(DrawMode::TRIANGLES, &self.vertices, &self.indices, self.render_states(), view_matrix);
    }
}

impl<'a> Text<'a> {
    pub fn new(text: String, font: Rc<RefCell<Font<'a>>>, x: f32, y: f32, z: f32, screen_size: (f32, f32)) -> Self {
        let mut vertices: Buffer<f32> = Buffer::new(BufferType::Array);
        let mut indices: Buffer<u16> = Buffer::new(BufferType::IndexArray);

        let font = font.clone();
        let (width, height) = font.borrow_mut().update_texture(&text, x, y, z, &mut vertices, &mut indices, screen_size);
        Text {
            text,
            font,
            vertices,
            indices,
            x,
            y,
            z,
            width,
            height,
            screen_size,
        }
    }

    // TODO: DIRTY
    pub fn set_position(&mut self, x: f32, y: f32, z: f32, screen_size: (f32, f32)) {
        let (width, height) = self.font.borrow_mut().update_texture(
            &self.text,
            x,
            y,
            z,
            &mut self.vertices,
            &mut self.indices,
            screen_size,
        );

        self.width = width;
        self.height = height;
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
        let (width, height) = self.font.borrow_mut().update_texture(
            &self.text,
            self.x,
            self.y,
            self.z,
            &mut self.vertices,
            &mut self.indices,
            self.screen_size,
        );
        self.width = width;
        self.height = height;
    }

    pub fn get_position(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }

    pub fn get_center(&self) -> (f32, f32, f32) {
        (self.x + self.width, self.y + self.height, self.z)
    }
}
