// Largely translated from this: http://paulbourke.net/geometry/polygonise/

use crate::graphics::{render_target, DrawMode, Drawable};
use crate::particles::{consts, fieldprovider::FieldProvider};
use gl_bindings::{shaders::OurShader, shaders::ShaderAttribute, Buffer, BufferType};
use na::Matrix4;
use resources::shaders::{OBJ_FRAGMENT_SHADER, OBJ_VERTEX_SHADER};
use std::{rc::Rc, str};

pub struct MarchingCubes {
    vertices: Buffer<f32>,
    shader: Rc<OurShader>,
}

type Vector3 = (f32, f32, f32);

impl Drawable for MarchingCubes {
    fn get_shader(&self) -> Option<Rc<OurShader>> {
        Some(self.shader.clone())
    }

    fn draw_transformed(&self, view_matrix: &Matrix4<f32>) {
        let len = self.vertices.len() as i32 / 6;
        render_target::draw_vertex_array(
            DrawMode::TRIANGLES,
            0,
            len,
            &self.vertices,
            &self.render_states(),
            view_matrix,
        )
    }
}

impl MarchingCubes {
    /// Sets the direction of the light illuminating the mesh.
    pub fn set_light_dir(&self, (x, y, z): Vector3) {
        let dist = (x * x + y * y + z * z).sqrt();
        self.shader
            .uniform3f("lightDir", x / dist, y / dist, z / dist);
    }

    /// Sets the transparency of the mesh.
    /// Argument should be 0.0 <= x <= 1.0.
    pub fn set_transparency(&self, transparency: f32) {
        self.shader.uniform1f("u_transparency", transparency);
    }

    pub fn marching_cubes(field: &FieldProvider) -> MarchingCubes {
        let mut vertices = Buffer::<f32>::new(BufferType::Array);

        const EPSILON: f32 = 0.1; // NOTE: 0.1 to reduce noise in data.
        const S: usize = 1; // step size

        let mut verts: [Vector3; 12] = [(0.0, 0.0, 0.0); 12];

        // Equivalent to `for x in... for y in... for z in...
        let iterator = (0..field.width)
            .step_by(S)
            .flat_map(|x| (0..field.height).step_by(S).map(move |y| (x, y)))
            .flat_map(|(x, y)| (0..field.depth).step_by(S).map(move |z| (x, y, z)));

        for (x, y, z) in iterator {
            let xs = x + S;
            let ys = y + S;
            let zs = z + S;
            let v1m = field.get_len((x, y, z));
            let v2m = field.get_len((xs, y, z));
            let v3m = field.get_len((xs, y, zs));
            let v4m = field.get_len((x, y, zs));
            let v5m = field.get_len((x, ys, z));
            let v6m = field.get_len((xs, ys, z));
            let v7m = field.get_len((xs, ys, zs));
            let v8m = field.get_len((x, ys, zs));

            let mut cidx: usize = 0;
            cidx |= if v1m > EPSILON { 1 } else { 0 };
            cidx |= if v2m > EPSILON { 2 } else { 0 };
            cidx |= if v3m > EPSILON { 4 } else { 0 };
            cidx |= if v4m > EPSILON { 8 } else { 0 };
            cidx |= if v5m > EPSILON { 16 } else { 0 };
            cidx |= if v6m > EPSILON { 32 } else { 0 };
            cidx |= if v7m > EPSILON { 64 } else { 0 };
            cidx |= if v8m > EPSILON { 128 } else { 0 };

            let edges = consts::MARCHING_CUBES_EDGE_TABLE[cidx];
            // This voxel is not on an edge
            if edges == 0 {
                continue;
            }

            let fx1 = x as f32 / field.width as f32;
            let fy1 = y as f32 / field.height as f32;
            let fz1 = z as f32 / field.depth as f32;

            let dx = S as f32 / field.width as f32;
            let dy = S as f32 / field.height as f32;
            let dz = S as f32 / field.depth as f32;

            let fx2 = fx1 + dx;
            let fy2 = fy1 + dy;
            let fz2 = fz1 + dz;

            let v = &mut verts;
            let push = &MarchingCubes::push_edge;
            let ep = EPSILON;
            push(edges, 00, v, ep, (fx1, fy1, fz1), (fx2, fy1, fz1), v1m, v2m);
            push(edges, 01, v, ep, (fx2, fy1, fz1), (fx2, fy1, fz2), v2m, v3m);
            push(edges, 02, v, ep, (fx2, fy1, fz2), (fx1, fy1, fz2), v3m, v4m);
            push(edges, 03, v, ep, (fx1, fy1, fz2), (fx1, fy1, fz1), v4m, v1m);
            push(edges, 04, v, ep, (fx1, fy2, fz1), (fx2, fy2, fz1), v5m, v6m);
            push(edges, 05, v, ep, (fx2, fy2, fz1), (fx2, fy2, fz2), v6m, v7m);
            push(edges, 06, v, ep, (fx2, fy2, fz2), (fx1, fy2, fz2), v7m, v8m);
            push(edges, 07, v, ep, (fx1, fy2, fz2), (fx1, fy2, fz1), v8m, v5m);
            push(edges, 08, v, ep, (fx1, fy1, fz1), (fx1, fy2, fz1), v1m, v5m);
            push(edges, 09, v, ep, (fx2, fy1, fz1), (fx2, fy2, fz1), v2m, v6m);
            push(edges, 10, v, ep, (fx2, fy1, fz2), (fx2, fy2, fz2), v3m, v7m);
            push(edges, 11, v, ep, (fx1, fy1, fz2), (fx1, fy2, fz2), v4m, v8m);

            let mut id = 0;

            let triangle_table = &consts::MARCHING_CUBES_TRIANGLE_TABLE;
            while triangle_table[cidx][id] != -1 {
                let v1 = verts[triangle_table[cidx][id] as usize];
                let v2 = verts[triangle_table[cidx][id + 1] as usize];
                let v3 = verts[triangle_table[cidx][id + 2] as usize];

                let ab = (v2.0 - v1.0, v2.1 - v1.1, v2.2 - v1.2);
                let cb = (v3.0 - v1.0, v3.1 - v1.1, v3.2 - v1.2);

                let cross = (
                    cb.1 * ab.2 - cb.2 * ab.1,
                    cb.2 * ab.0 - cb.0 * ab.2,
                    cb.0 * ab.1 - cb.1 * ab.0,
                );
                let dist = (cross.0 * cross.0 + cross.1 * cross.1 + cross.2 * cross.2).sqrt();
                let norm = (cross.0 / dist, cross.1 / dist, cross.2 / dist);

                MarchingCubes::push_vert(&mut vertices, v1, norm);
                MarchingCubes::push_vert(&mut vertices, v2, norm);
                MarchingCubes::push_vert(&mut vertices, v3, norm);
                id += 3;
            }
        }

        vertices.bind();
        let len = vertices.len();
        vertices.upload_data(0, len, true);

        let shader: OurShader = OurShader::new(
            str::from_utf8(OBJ_VERTEX_SHADER).expect("Failed to read vertex shader"),
            str::from_utf8(OBJ_FRAGMENT_SHADER).expect("Failed to read fragment shader"),
            &[
                ShaderAttribute {
                    name: "a_position".to_string(),
                    size: 3,
                },
                ShaderAttribute {
                    name: "a_normal".to_string(),
                    size: 3,
                },
            ],
        );
        MarchingCubes {
            vertices,
            shader: Rc::new(shader),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn push_edge(
        edges: u32,
        edge: usize,
        verts: &mut [Vector3],
        epsilon: f32,
        v1: Vector3,
        v2: Vector3,
        m1: f32,
        m2: f32,
    ) {
        if edges & (1 << edge) != 0 {
            verts[edge] = MarchingCubes::interp(epsilon, v1, v2, m1, m2);
        }
    }

    fn push_vert(vertices: &mut Buffer<f32>, (x, y, z): Vector3, (nx, ny, nz): Vector3) {
        vertices.push(&[
            x - 0.5,
            y - 0.5,
            z - 0.5, // position
            nx,
            ny,
            nz, // normals
        ])
    }

    fn interp(epsilon: f32, v1: Vector3, v2: Vector3, m1: f32, m2: f32) -> Vector3 {
        if (epsilon - m1).abs() < 0.00001 {
            return v1;
        }

        if (epsilon - m2).abs() < 0.00001 {
            return v2;
        }

        if (m1 - m2).abs() < 0.00001 {
            return v1;
        }

        let mu = (epsilon - m1) / (m2 - m1);
        let x = v1.0 + mu * (v2.0 - v1.0);
        let y = v1.1 + mu * (v2.1 - v1.1);
        let z = v1.2 + mu * (v2.2 - v1.2);
        (x, y, z)
    }
}
