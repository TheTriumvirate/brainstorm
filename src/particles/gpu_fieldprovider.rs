use super::VectorField;
use gl_bindings::{Texture, TextureFormat};
use std::{f32, rc::Rc};

pub struct GPUFieldProvider {
    texture: Rc<Texture>,
    size: usize,
}

impl GPUFieldProvider {
    pub fn new(x: &VectorField) -> Self {
        let mut max: f32 = 0.0;
        let mut min: f32 = 0.0;
        for plane in x.vectors.iter() {
            for row in plane {
                for elem in row {
                    let (dx, dy, dz, _) = elem;
                    max = max.max(*dy);
                    max = max.max(*dx);
                    max = max.max(*dz);
                    min = min.min(*dx);
                    min = min.min(*dy);
                    min = min.min(*dz);
                }
            }
        }

        let mut data = Vec::new();
        for plane in &x.vectors {
            for row in plane {
                for elem in row {
                    let (dx, dy, dz, da) = elem;

                    let dx = (dx - min) / (max - min);
                    let dy = (dy - min) / (max - min);
                    let dz = (dz - min) / (max - min);

                    data.push((dx * 255.0).round() as u8);
                    data.push((dy * 255.0).round() as u8);
                    data.push((dz * 255.0).round() as u8);
                    data.push((da * 255.0).round() as u8);
                }
            }
        }
        GPUFieldProvider {
            texture: Rc::new(Texture::from_3d_data(
                x.width as u32,
                x.height as u32,
                x.depth as u32,
                TextureFormat::RGBA,
                &data[..],
                false,
            )),
            size: x.depth,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn get_texture(&self) -> Rc<Texture> {
        self.texture.clone()
    }
}
