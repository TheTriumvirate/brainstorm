use graphics::Drawable;

use resources::fonts::DEFAULT;

use image::{DynamicImage, Rgba};

pub mod font;
use self::font::Font;

use std::rc::Rc;

use gl_context::{Texture, TextureFormat, shaders::OurShader, Buffer, BufferType};

use rusttype::gpu_cache::{Cache, CacheBuilder};

use graphics::Rectangle;
use graphics::position::*;
use graphics::*;

pub struct Text<'a> {
    text: String,
    font: Rc<Font<'a>>,
    texture: Texture,
    cache: Cache<'a>,
    vertices: Buffer<f32>,
    indices: Buffer<u16>,
}

impl<'a> Drawable for Text<'a> {
    fn get_texture(&self) -> Option<&Texture> {
        Some(&self.texture)
    }

    fn draw(&self) {
        render_target::draw_indices(&self.vertices, &self.indices, RenderStates::from(self));
    }
}

impl<'a> Text<'a> {
    pub fn new(text: String, font: Rc<Font<'a>>) -> Self {
        let mut cache = Cache::builder()
            .dimensions(512, 512)
            .build();

        let mut vertices: Buffer<f32> = Buffer::new(BufferType::Array);
        let mut indices: Buffer<u16> = Buffer::new(BufferType::IndexArray);

        let mut t = Text {
            text,
            font: font.clone(),
            texture: Texture::new(512, 512, TextureFormat::LUMINANCE, None),
            cache,
            vertices,
            indices,
        };

        font.as_ref().update_texture(&mut t.cache, &t.texture, &t.text, &mut t.vertices, &mut t.indices);

        t
    }
}