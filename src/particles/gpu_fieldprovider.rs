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
    textures: Vec<Rc<Texture>>
}

impl GPUFieldProvider {
    pub fn new(raw_data: &[u8]) -> Self {
        let mut textures = Vec::new();
        let x: VectorField = deserialize(raw_data).expect("Failed to deserialize data.");
        
        let mut max : f32 = 0.0;
        for plane in x.vectors.iter() {
            for row in plane {
                for elem in row {
                    let (dx, dy, dz, da) = elem;
                    let (dx, dy, dz) = (dx * da, dy * da, dz *da);
                    max = max.max(dx);
                    max = max.max(dy);
                    max = max.max(dz);
                }
            }
        }
        
        // TODO: RGB only
        for plane in x.vectors {
            let mut data = Vec::new();
            for row in plane {
                for elem in row {
                    let (dx, dy, dz, da) = elem;
                    let (dx, dy, dz) = (dx * da, dy * da, dz *da);
                    data.push(dx / max);
                    data.push(dy / max);
                    data.push(dz / max);
                    data.push(1.0);
                }
            }
            textures.push(Rc::new(Texture::from_data(x.width as u32, x.height as u32, TextureFormat::RGBA, &data[..])));
        }
        GPUFieldProvider {
            textures
        }
    }

    pub fn len(&self) -> usize {
        self.textures.len()
    }

    pub fn get(&self, index: usize) -> Rc<Texture> {
        self.textures[index].clone()
    }
}
