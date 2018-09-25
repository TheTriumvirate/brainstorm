use image::{self};

use AbstractContext;
use Context;
use NativeTexture;
use shaders::OurShader;

const DEFAULT_TEXTURE: &[u8] = include_bytes!("default.png");

lazy_static! {
    static ref DEFAULT: Texture = Texture::new(32, 32, DEFAULT_TEXTURE);
}

pub struct Texture {
    texture: NativeTexture,
}

impl Texture {
    // Assumes png format
    pub fn new(width: u32, height: u32, data: &[u8]) -> Self {
        let context = Context::get_context();

        let texture = context.create_texture().unwrap();
        context.bind_texture(Context::TEXTURE_2D, &texture);

        match image::load_from_memory(data).unwrap() {
            image::ImageRgba8(image) => {
                context.tex_image2d(
                    Context::TEXTURE_2D,
                    0,
                    Context::RGBA as i32,
                    width as i32,
                    height as i32,
                    0,
                    Context::RGBA,
                    &image.into_raw()[..]
                );
            }
            _ => {
                panic!("Failed to load texture, unsuported pixel format.");
            }
        }

        //context.tex_image2d(Context::TEXTURE_2D, 0, Context::RGBA as i32, width as i32, height as i32, 0, Context::RGBA, &data);
        context.generate_mipmap(Context::TEXTURE_2D);
        Texture {
            texture
        }
    }

    pub fn bind(&self) {
        let context = Context::get_context();
        context.bind_texture(Context::TEXTURE_2D, &self.texture);        
    }

    pub fn activate(&self, shader: &OurShader) {
        let context = Context::get_context();
        context.active_texture(Context::TEXTURE0);
        self.bind();
        shader.uniform1i("uSampler", 0);
    }

    pub fn default() -> &'static Self {
        &DEFAULT
    }
}