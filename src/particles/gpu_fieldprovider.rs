use bincode::deserialize;
use std::f32;

use gl_bindings::{Texture, TextureFormat};
use std::rc::Rc;

type Vector4 = (f32, f32, f32, f32);

#[derive(Serialize, Deserialize)]
struct VectorField {
    width: usize,
    height: usize,
    depth: usize,
    vectors: Vec<Vec<Vec<Vector4>>>,
}

pub struct GPUFieldProvider {
    texture: Rc<Texture>,
    size: usize
}

impl GPUFieldProvider {
    pub fn new(raw_data: &[u8]) -> Result<Self, &'static str> {
        let x: VectorField = deserialize(raw_data).map_err(|_| "Failed to deserialize data.")?;
        
        let mut max : f32 = 0.0;
        let mut min : f32 = 0.0;
        for plane in x.vectors.iter() {
            for row in plane {
                for elem in row {
                    let (dx, dy, dz, da) = elem;
                    let (dx, dy, dz) = (dx * da, dy * da, dz *da);
                    max = max.max(dx);
                    max = max.max(dy);
                    max = max.max(dz);
                    min = min.min(dx);
                    min = min.min(dy);
                    min = min.min(dz);
                }
            }
        }
        
        // TODO: RGB only
        let mut data = Vec::new();
        for plane in x.vectors {
            for row in plane {
                for elem in row {
                    let (dx, dy, dz, da) = elem;
                    let (dx, dy, dz) = (dx * da, dy * da, dz *da);
                    
                    let dx = (dx - min) / (max - min);
                    let dy = (dy - min) / (max - min);
                    let dz = (dz - min) / (max - min);

                    data.push(dx);
                    data.push(dy);
                    data.push(dz);
                    data.push(1.0);
                }
            }
        }
        Ok(GPUFieldProvider {
            texture: Rc::new(Texture::from_3d_data(x.width as u32, x.height as u32, x.depth as u32, TextureFormat::RGBA, &data[..], false)),
            size: x.depth,
        })
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn get_texture(&self) -> Rc<Texture> {
        self.texture.clone()
    }
}
