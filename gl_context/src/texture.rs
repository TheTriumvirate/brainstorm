use image::{self};

use AbstractContext;
use Context;
use NativeTexture;
use shaders::OurShader;

#[derive(Copy, Clone)]
pub enum TextureFormat {
    RGBA,
    LUMINANCE,
}

pub struct Texture {
    texture: NativeTexture,
    _format: TextureFormat,
}

impl Texture {
    // Assumes png format
    pub fn new(width: u32, height: u32, format: TextureFormat, data: Option<&[u8]>) -> Self {
        let context = Context::get_context();

        let texture = context.create_texture().unwrap();
        context.bind_texture(Context::TEXTURE_2D, &texture);
        
        let formatv : u32 = format.into();

        match data {
            Some(data) => {
                match image::load_from_memory(data).unwrap() {
                    image::ImageRgba8(image) => {
                        context.tex_image2d(
                            Context::TEXTURE_2D,
                            0,
                            formatv as i32,
                            width as i32,
                            height as i32,
                            0,
                            formatv,
                            Some(&image.into_raw()[..])
                        );
                    }
                    _ => {
                        panic!("Failed to load texture, unsuported pixel format.");
                    }
                }
            },
            _ => {
                context.tex_image2d(
                    Context::TEXTURE_2D,
                    0,
                    formatv as i32,
                    width as i32,
                    height as i32,
                    0,
                    formatv,
                    None
                )
            }
        }

        //context.tex_image2d(Context::TEXTURE_2D, 0, Context::RGBA as i32, width as i32, height as i32, 0, Context::RGBA, &data);
        //context.generate_mipmap(Context::TEXTURE_2D);
        
        context.tex_parameteri(
            Context::TEXTURE_2D,
            Context::TEXTURE_WRAP_S,
            Context::CLAMP_TO_EDGE as i32
        );

        context.tex_parameteri(
            Context::TEXTURE_2D,
            Context::TEXTURE_WRAP_T,
            Context::CLAMP_TO_EDGE as i32
        );

        /* Linear filtering usually looks best for text. */
        context.tex_parameteri(
            Context::TEXTURE_2D,
            Context::TEXTURE_MIN_FILTER,
            Context::LINEAR as i32
        );
        context.tex_parameteri(
            Context::TEXTURE_2D,
            Context::TEXTURE_MAG_FILTER,
            Context::LINEAR as i32
        );

        context.pixel_storei(Context::UNPACK_ALIGNMENT, 1);

        Texture {
            texture,
            format,
        }
    }

    pub fn bind(&self) {
        let context = Context::get_context();
        context.bind_texture(Context::TEXTURE_2D, &self.texture);
    }

    pub fn activate(&self, shader: Option<&OurShader>) {
        let context = Context::get_context();
        context.active_texture(Context::TEXTURE0);
        
        if let Some(shader) = shader {
            self.bind();
            shader.uniform1i("uSampler", 0);
        }
    }

    pub fn unbind(&self) {
        let context = Context::get_context();
        context.unbind_texture(Context::TEXTURE_2D);   
    }

    pub fn update_sub_rect(&self, x: i32, y: i32, w: i32, h: i32, data: &[u8]) {
        self.bind();
        self.activate(None);
        let context = Context::get_context();
        //let format = self.format.into();
        context.tex_sub_image2d(Context::TEXTURE_2D, 0, x, y, w, h, Context::LUMINANCE, Some(&data));
        self.unbind();
    }
}

impl Into<u32> for TextureFormat {
    fn into(self) -> u32 {
        match self {
            TextureFormat::RGBA => Context::RGBA,
            TextureFormat::LUMINANCE => Context::LUMINANCE,
        }
    }
}