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
    textures: Vec<Rc<Texture>>,
    size: usize
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
        let mut data = Vec::new();
        for plane in x.vectors {
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
        }
        textures.push(Rc::new(Texture::from_3d_data(x.width as u32, x.height as u32, x.depth as u32, TextureFormat::RGBA, &data[..])));
        GPUFieldProvider {
            textures,
            size: x.depth,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn get(&self, index: usize) -> Rc<Texture> {
        self.textures[0].clone()
    }
}