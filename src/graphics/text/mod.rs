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
    rectangle: Rectangle,
    cache: Cache<'a>,
    vertices: Buffer<f32>,
    indices: Buffer<u16>,
}

impl<'a> Drawable for Text<'a> {
    fn draw(&self) {
        self.texture.activate(Some(OurShader::default()));
        self.rectangle.draw();

        render_target::test(&self.vertices, &self.indices, &self.texture);
    }
}

impl<'a> Text<'a> {
    pub fn new(text: String, font: Rc<Font<'a>>) -> Self {
        let coords = Coordinates {
            x1: -0.4,
            x2: 0.4,
            y1: -0.4,
            y2: 0.4,
        };

        let mut cache = Cache::builder()
            .dimensions(512, 512)
            .build();

        let mut vertices: Buffer<f32> = Buffer::new(BufferType::Array);
        let mut indices: Buffer<u16> = Buffer::new(BufferType::IndexArray);


        let mut t = Text {
            text,
            font: font.clone(),
            texture: Texture::new(512, 512, TextureFormat::LUMINANCE, None),
            rectangle: Rectangle::new(coords, (1.0, 1.0, 1.0)),
            cache,
            vertices,
            indices,
        };

        font.as_ref().update_texture(&mut t.cache, &t.texture, &t.text, &mut t.vertices, &mut t.indices);
        //font.as_ref().

        t
    }
    /*
    // Deprecated
    pub fn render_to_image(&self) {
        let font_data = DEFAULT;

        let font = Font::from_bytes(font_data as &[u8]).expect("Error constructing font!");

        // font size (TODO: custom property)
        let scale = Scale::uniform(32.0);

        let text = &self.text[..];

        // font color (TODO: custom property)
        let colour = (150, 0, 0);

        let v_metrics = font.v_metrics(scale);

        
        // layout the glyphs in a line with 20 pixels padding
        let glyphs: Vec<_> = font
            .layout(text, scale, point(20.0, 20.0 + v_metrics.ascent))
            .collect();

        // work out the layout size
        let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        let glyphs_width = {
            let min_x = glyphs
                .first()
                .map(|g| g.pixel_bounding_box().unwrap().min.x)
                .unwrap();
            let max_x = glyphs
                .last()
                .map(|g| g.pixel_bounding_box().unwrap().max.x)
                .unwrap();
            (max_x - min_x) as u32
        };

        // Create a new rgba image with some padding
        let mut image = DynamicImage::new_rgba8(glyphs_width + 40, glyphs_height + 40).to_rgba();

        // Loop through the glyphs in the text, positing each one on a line
        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                // Draw the glyph into the image per-pixel by using the draw closure
                glyph.draw(|x, y, v| {
                    image.put_pixel(
                        // Offset the position by the glyph bounding box
                        x + bounding_box.min.x as u32,
                        y + bounding_box.min.y as u32,
                        // Turn the coverage into an alpha value
                        Rgba {
                            data: [colour.0, colour.1, colour.2, (v * 255.0) as u8],
                        },
                    )
                });
            }
        }

        // Save the image to a png file
        image.save("image_example.png").unwrap();
        println!("Generated: image_example.png");
    }*/
}