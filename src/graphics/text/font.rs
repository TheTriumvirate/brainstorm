use rusttype::gpu_cache::Cache;
use rusttype::{point, vector, Font as TFont, PositionedGlyph, Rect, Scale};

use gl_context::{shaders::OurShader, shaders::ShaderAttribute, Buffer, Texture, TextureFormat};

use std::rc::Rc;
use std::str;

use resources::shaders::{TEXT_FRAGMENT_SHADER, TEXT_VERTEX_SHADER};

lazy_static! {
    static ref SHADER: OurShader = OurShader::new(
        str::from_utf8(TEXT_VERTEX_SHADER).expect("Failed to read vertex shader"),
        str::from_utf8(TEXT_FRAGMENT_SHADER).expect("Failed to read fragment shader"),
        &[
            ShaderAttribute {
                name: "a_position".to_string(),
                size: 2
            },
            ShaderAttribute {
                name: "a_color".to_string(),
                size: 3
            },
            ShaderAttribute {
                name: "a_texture".to_string(),
                size: 2
            },
        ]
    );
}

pub struct Font<'a> {
    font: TFont<'a>,
    texture: Rc<Texture>,
    cache: Cache<'a>,
}

impl<'a> Font<'a> {
    pub fn from_bytes(data: &'a [u8]) -> Self {
        let font = TFont::from_bytes(data).expect("Could not load font from bytes");

        let cache = Cache::builder().dimensions(512, 512).build();

        Font {
            font,
            texture: Rc::from(Texture::new(512, 512, TextureFormat::LUMINANCE, None)),
            cache,
        }
    }

    pub fn get_texture(&self) -> Rc<Texture> {
        self.texture.clone()
    }

    pub fn get_shader(&self) -> &'static OurShader {
        &SHADER
    }

    pub fn update_texture(
        &mut self,
        text: &str,
        x: f32,
        y: f32,
        vertices: &mut Buffer<f32>,
        indices: &mut Buffer<u16>,
        aspect_ratio: f32,
    ) {
        let glyphs = self.layout_paragraph(Scale::uniform(24.0), 512, text);

        for glyph in &glyphs {
            // TODO: font-ids
            self.cache.queue_glyph(0, glyph.clone());
        }

        let texture: &Texture = self.texture.as_ref();
        self.cache
            .cache_queued(|rect, data| {
                texture.update_sub_rect(
                    rect.min.x as i32,
                    rect.min.y as i32,
                    rect.width() as i32,
                    rect.height() as i32,
                    &data,
                );
            }).expect("Could not construct cache texture");

        let origin = point(x, y);
        let mut idx = 0;

        vertices.clear();
        indices.clear();

        let v_metrics = self.font.v_metrics(Scale::uniform(24.0));
        let advance_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;

        for g in glyphs {
            if let Ok(Some((uv_rect, screen_rect))) = self.cache.rect_for(0, &g) {
                // TODO: font-ids
                let gl_rect = Rect {
                    min: origin
                        + (vector(
                            screen_rect.min.x as f32 / 1024.0 / aspect_ratio - 0.5,
                            1.0 - screen_rect.min.y as f32 / 1024.0 - 0.5,
                        )) * 2.0
                        + vector(1.0, -1.0 + advance_height / 512.0),
                    max: origin
                        + (vector(
                            screen_rect.max.x as f32 / 1024.0 / aspect_ratio - 0.5,
                            1.0 - screen_rect.max.y as f32 / 1024.0 - 0.5,
                        )) * 2.0
                        + vector(1.0, -1.0 + advance_height / 512.0),
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

                indices.push(&[idx, idx + 1, idx + 2, idx, idx + 2, idx + 3]);

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
                    '\n' => caret = point(0.0, caret.y + advance_height),
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
