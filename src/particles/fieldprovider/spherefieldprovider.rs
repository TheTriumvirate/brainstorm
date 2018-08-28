
use super::{FieldProvider, Field};

use std::f32;

type Vector3 = (f32, f32, f32);

fn lerpf(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

fn lerp((ax, ay, az): Vector3, (bx, by, bz): Vector3, t: f32) -> Vector3 {
    (
        lerpf(ax, bx, t),
        lerpf(ay, by, t),
        lerpf(az, bz, t)
    )
}

fn lerp2d(lxly: Vector3, lxuy: Vector3, uxly: Vector3, uxuy: Vector3, t1: f32, t2: f32) -> Vector3 {
    let s = lerp(lxly, lxuy, t1);
    let v = lerp(uxly, uxuy, t1);
    lerp(s, v, t2)
}

fn lerp3d(v1: Vector3, v2: Vector3, v3: Vector3, v4: Vector3, v5: Vector3, v6: Vector3, v7: Vector3, v8: Vector3, t1: f32, t2: f32, t3: f32) -> Vector3 {
    let s = lerp2d(v1, v2, v3, v4, t1, t2);
    let v = lerp2d(v5, v6, v7, v8, t1, t2);
    lerp(s, v, t3)
}

pub struct SphereFieldProvider {
    data: Field
}

impl SphereFieldProvider {
        
    fn get_vec(&self, (fx, fy, fz): (usize, usize, usize)) -> Option<(f32, f32, f32)> {
        let data_x = self.data.get(fx);
        if data_x == None {return None;}
        let data_x = data_x.unwrap();
        let data_y = data_x.get(fy);
        if data_y == None {return None;}
        let data_y = data_y.unwrap();
        let vec = data_y.get(fz);
        if vec == None {return None;}
        Some (*vec.unwrap())
    }
}

impl FieldProvider for SphereFieldProvider {
    fn new() -> Self {
        let mut data: Field = Vec::new();
        for i in 0..100 {
            let mut y = Vec::new();
            for j in 0..100 {
                let mut z = Vec::new();
                for k in 0..100 {
                    let fx = i as f32 / 100.0 - 0.5;
                    let fy = j as f32 / 100.0 - 0.5;
                    let fz = k as f32 / 100.0 - 0.5;

                    let max = (fx * fx + fy * fy + fz * fz).sqrt();

                    if max < 0.4 {
                        z.push((fx, fy, fz));
                    } else {
                        z.push((-fx, -fy, -fz));
                    }
                    /*
                    let vx = -(fx - 0.2).abs();
                    let vy = -(fy - 0.2).abs();
                    let vz = -(fz - 0.2).abs();
                    z.push((vx, vy, vz));*/
                }
                y.push(z);
            }
            data.push(y);
        }
        SphereFieldProvider {
            data
        }
    }

    fn delta(&self, (x, y, z): (f32, f32, f32)) -> (f32, f32, f32) {
        let lx = x.floor() as usize;
        let ly = y.floor() as usize;
        let lz = z.floor() as usize;
        let ux = x.ceil() as usize;
        let uy = y.ceil() as usize;
        let uz = z.ceil() as usize;
        let v1 = self.get_vec((lx, ly, lz)).unwrap_or((0.0,0.0,0.0));
        let v2 = self.get_vec((lx, ly, uz)).unwrap_or((0.0,0.0,0.0));
        let v3 = self.get_vec((lx, uy, lz)).unwrap_or((0.0,0.0,0.0));
        let v4 = self.get_vec((lx, uy, uz)).unwrap_or((0.0,0.0,0.0));
        let v5 = self.get_vec((ux, ly, lz)).unwrap_or((0.0,0.0,0.0));
        let v6 = self.get_vec((ux, ly, uz)).unwrap_or((0.0,0.0,0.0));
        let v7 = self.get_vec((ux, uy, lz)).unwrap_or((0.0,0.0,0.0));
        let v8 = self.get_vec((ux, uy, uz)).unwrap_or((0.0,0.0,0.0));

        let t1 = x - x.floor();
        let t2 = y - y.floor();
        let t3 = z - z.floor();
        let res = lerp3d(v1, v2, v3, v4, v5, v6, v7, v8, t1, t2, t3);

        res
    }
}