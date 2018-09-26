use rusttype::gpu_cache::{Cache};
use rusttype::{point, Font as TFont, Scale, Rect, PositionedGlyph, vector};

use gl_context::{Texture, TextureFormat, Buffer};

pub struct Font<'a> {
    font: TFont<'a>,
    texture: Texture,
    cache: Cache<'a>,
}

impl<'a> Font<'a> {
    pub fn from_bytes(data: &'a[u8]) -> Self {
        let font = TFont::from_bytes(data).expect("Could not load font from bytes");
        
        let cache = Cache::builder()
            .dimensions(512, 512)
            .build();

        Font {
            font,
            texture: Texture::new(512, 512, TextureFormat::LUMINANCE, None),
            cache,
        }
    }

    pub fn get_texture(&self) -> &Texture {
        return &self.texture;
    }
    
    pub fn update_texture<'b>(&mut self, text: &str, vertices: &mut Buffer<f32>, indices: &mut Buffer<u16>) {
        let glyphs = self.layout_paragraph(Scale::uniform(24.0), 512, text);

        for glyph in &glyphs {
            // TODO: font-ids
            self.cache.queue_glyph(0, glyph.clone());
        }
        
        let texture : &Texture = &self.texture;
        self.cache.cache_queued(|rect, data| {
            texture.update_sub_rect(
                rect.min.x as i32,
                rect.min.y as i32,
                rect.width() as i32,
                rect.height() as i32,
                &data,
            );
        }).expect("Could not construct cache texture");

    let origin = point(0.0, 0.0);
    let mut idx = 0;

        for g in glyphs {
            if let Ok(Some((uv_rect, screen_rect))) = self.cache.rect_for(0, &g) { // TODO: font-ids
                let gl_rect = Rect {
                    min: origin
                        + (vector(
                            screen_rect.min.x as f32 / 1000.0 - 0.5,
                            1.0 - screen_rect.min.y as f32 / 1000.0 - 0.5,
                        )) * 2.0,
                    max: origin
                        + (vector(
                            screen_rect.max.x as f32 / 1000.0 - 0.5,
                            1.0 - screen_rect.max.y as f32 / 1000.0 - 0.5,
                        )) * 2.0,
                };
                vertices.push(&[
                    gl_rect.min.x,
                    gl_rect.min.y,
                    1.0,
                    1.0,
                    1.0,
                    uv_rect.min.x,
                    uv_rect.min.y,
                    gl_rect.max.x,
                    gl_rect.min.y,
                    1.0,
                    1.0,
                    1.0,
                    uv_rect.max.x,
                    uv_rect.min.y,
                    gl_rect.max.x,
                    gl_rect.max.y,
                    1.0,
                    1.0,
                    1.0,
                    uv_rect.max.x,
                    uv_rect.max.y,
                    gl_rect.min.x,
                    gl_rect.max.y,
                    1.0,
                    1.0,
                    1.0,
                    uv_rect.min.x,
                    uv_rect.max.y,
                ]);

                indices.push(&[
                   idx, 
                   idx+1, 
                   idx+2, 
                   idx, 
                   idx+2, 
                   idx+3, 
                ]);

                idx += 4;
            }
        }
        
        vertices.bind();
        let len = vertices.len();
        vertices.upload_data(0, len, true);
        indices.bind();
        let len = indices.len();
        indices.upload_data(0, len, true);
    }

    fn layout_paragraph(&self, scale: Scale, width: u32, text: &str) -> Vec<PositionedGlyph<'a>> {
        use unicode_normalization::UnicodeNormalization;
        let mut result = Vec::new();
        let v_metrics = self.font.v_metrics(scale);
        let advance_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
        let mut caret = point(0.0, v_metrics.ascent);
        let mut last_glyph_id = None;
        for c in text.nfc() {
            if c.is_control() {
                match c {
                    '\n' => {
                        caret = point(0.0, caret.y + advance_height)
                    }
                    _ => {}
                }
                continue;
            }

            let base_glyph = self.font.glyph(c);
            if let Some(id) = last_glyph_id.take() {
                caret.x += self.font.pair_kerning(scale, id, base_glyph.id());
            }
            last_glyph_id = Some(base_glyph.id());
            let mut glyph = base_glyph.scaled(scale).positioned(caret);
            if let Some(bb) = glyph.pixel_bounding_box() {
                if bb.max.x > width as i32 {
                    caret = point(0.0, caret.y + advance_height);
                    glyph = glyph.into_unpositioned().positioned(caret);
                    last_glyph_id = None;
                }
            }
            caret.x += glyph.unpositioned().h_metrics().advance_width;
            result.push(glyph);
        }
        result
    }
}