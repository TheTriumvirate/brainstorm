use super::{FieldProvider};

use std::f32;

type Vector3 = (f32, f32, f32);

fn lerpf(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

fn lerp((ax, ay, az): Vector3, (bx, by, bz): Vector3, t: f32) -> Vector3 {
    (lerpf(ax, bx, t), lerpf(ay, by, t), lerpf(az, bz, t))
}

fn lerp2d(lxly: Vector3, lxuy: Vector3, uxly: Vector3, uxuy: Vector3, t1: f32, t2: f32) -> Vector3 {
    let s = lerp(uxly, lxuy, t1);
    let v = lerp(lxly, uxuy, t2);
    lerp(s, v, t2)
}

fn lerp3d(
    v1: Vector3,
    v2: Vector3,
    v3: Vector3,
    v4: Vector3,
    v5: Vector3,
    v6: Vector3,
    v7: Vector3,
    v8: Vector3,
    t1: f32,
    t2: f32,
    t3: f32,
) -> Vector3 {
    let s = lerp2d(v1, v2, v3, v4, t1, t2);
    let v = lerp2d(v5, v6, v7, v8, t1, t2);
    lerp(s, v, t3)
}

pub struct SphereFieldProvider {
    data: Vec<(f32, f32, f32)>,
}

impl SphereFieldProvider {
    fn get_vec(&self, (fx, fy, fz): (usize, usize, usize)) -> (f32, f32, f32) {
        let fx = fx.min(99);
        let fy = fy.min(99);
        let fz = fz.min(99);
        let index = fz + fy * 100 + fx * 100 * 100;
        self.data[index]
        //(0.0, 0.0, 0.0)
    }
}

impl FieldProvider for SphereFieldProvider {
    fn new() -> Self {
        let mut data = Vec::new();
        
        for i in 0..100 {
            for j in 0..100 {
                for k in 0..100 {
                    let mut fx = i as f32 / 100.0 - 0.5;
                    let mut fy = j as f32 / 100.0 - 0.5;
                    let mut fz = k as f32 / 100.0 - 0.5;

                    if fx == 0.0 && fy == 0.0 && fz == 0.0 {
                        fx = 0.1;
                        fy = 0.1;
                        fz = 0.1;
                    }

                    let max = (fx * fx + fy * fy + fz * fz).sqrt();

                    if max < 0.4 {
                        data.push((fx, fy, fz));
                    } else {
                        data.push((-fx, -fy, -fz));
                    }
                    /*
                    let vx = -(fx - 0.2).abs();
                    let vy = -(fy - 0.2).abs();
                    let vz = -(fz - 0.2).abs();
                    z.push((vx, vy, vz));*/
                }
            }
        }
        SphereFieldProvider { data }
    }

    fn delta(&self, (x, y, z): (f32, f32, f32)) -> (f32, f32, f32) {
        let x = x * 100.0 + 50.0;
        let y = y * 100.0 + 50.0;
        let z = z * 100.0 + 50.0;
        let lx = x.floor() as usize;
        let ly = y.floor() as usize;
        let lz = z.floor() as usize;
        let ux = x.ceil() as usize;
        let uy = y.ceil() as usize;
        let uz = z.ceil() as usize;
        let v1 = self.get_vec((lx, ly, lz));
        let v2 = self.get_vec((lx, ly, uz));
        let v3 = self.get_vec((lx, uy, lz));
        let v4 = self.get_vec((lx, uy, uz));
        let v5 = self.get_vec((ux, ly, lz));
        let v6 = self.get_vec((ux, ly, uz));
        let v7 = self.get_vec((ux, uy, lz));
        let v8 = self.get_vec((ux, uy, uz));

        let t1 = x - x.floor();
        let t2 = y - y.floor();
        let t3 = z - z.floor();
        let res = lerp3d(v1, v2, v3, v4, v5, v6, v7, v8, t1, t2, t3);

        res
    }
}
