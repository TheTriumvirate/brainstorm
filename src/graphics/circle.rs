#![allow(dead_code)] // Not yet in use.
use gl_bindings::{Buffer, BufferType};
use crate::graphics::{render_target, DrawMode, Drawable};
use na::Matrix4;
use std::f32;

const POINTS: usize = 40;

/// Represents a drawable circle.
pub struct Circle {
    pos: (f32, f32, f32),
    rotation: f32,
    rotation_axis: (f32, f32, f32),
    radius: f32,
    filled: bool,
    color: (f32, f32, f32),
    vertices: Buffer<f32>,
    indices: Buffer<u16>,
    draw_mode: DrawMode,
}

impl Circle {
    /// Creates a new circle with the chosen parameters.
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        radius: f32,
        rotation: f32,
        rotation_axis: (f32, f32, f32),
        filled: bool,
    ) -> Self {
        let vertices: Buffer<f32> = Buffer::new(BufferType::Array);
        let indices: Buffer<u16> = Buffer::new(BufferType::IndexArray);

        let mut result = Circle {
            pos: (x, y, z),
            rotation,
            rotation_axis,
            radius,
            filled,
            color: (1.0, 1.0, 1.0),
            vertices,
            indices,
            draw_mode: DrawMode::LINES,
        };

        result.rebuild_data();

        result
    }

    pub fn set_rotation(&mut self, rotation: f32, rotation_axis: (f32, f32, f32)) {
        self.rotation = rotation;
        self.rotation_axis = rotation_axis;
    }

    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.color = (r, g, b);
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }

    pub fn set_center(&mut self, center: (f32, f32, f32)) {
        self.pos = center;
    }

    pub fn get_center(&self) -> (f32, f32, f32) {
        self.pos
    }

    pub fn rebuild_data(&mut self) {
        let mut vertex_data = Vec::new();
        let mut index_data = Vec::new();

        let (x, y, z) = self.pos;
        let rot = self.rotation;
        let (ax, ay, az) = self.rotation_axis;
        let radius = self.radius;
        let (r, g, b) = self.color;

        if self.filled {
            vertex_data.extend_from_slice(&[x, y, z, 1.0, 1.0, 1.0, 0.5, 0.5]);
            for i in 0..POINTS {
                let progress = i as f32 / POINTS as f32;
                let dt = progress * f32::consts::PI * 2.0;
                let dx = x + radius * dt.cos();
                let dy = y + radius * dt.sin();
                let dz = z;
                vertex_data.extend_from_slice(&[
                    dx,
                    dy,
                    dz,
                    1.0,
                    1.0,
                    1.0,
                    dx / 2.0 + 0.5,
                    dy / 2.0 + 0.5,
                ]);

                if i > 0 {
                    let idx = i as u16;
                    index_data.extend_from_slice(&[0, idx + 1, idx]);
                }
            }
            index_data.extend_from_slice(&[0, 1, POINTS as u16]);
            self.draw_mode = DrawMode::TRIANGLES;
        //TODO: implement
        } else {
            for i in 0..POINTS {
                let progress = i as f32 / POINTS as f32;
                let dt = progress * f32::consts::PI * 2.0;

                let dts = radius * dt.sin();
                let dtc = radius * dt.cos();

                let dx = x + dtc * ax + dts * ay * rot.sin() + dts * az * rot.cos();
                let dy = y + dts * ax * rot.cos() + dtc * ay + dts * az * rot.sin();
                let dz = z + dts * ax * rot.sin() + dts * ay * rot.cos() + dtc * az;
                vertex_data.extend_from_slice(&[
                    dx,
                    dy,
                    dz,
                    r,
                    g,
                    b,
                    dx / 2.0 + 0.5,
                    dy / 2.0 + 0.5,
                ]);

                if i > 0 {
                    let idx = i as u16;
                    index_data.extend_from_slice(&[idx - 1, idx]);
                }
            }
            index_data.extend_from_slice(&[0, POINTS as u16 - 1]);
            self.draw_mode = DrawMode::LINES;
        }

        self.vertices.set_data(&vertex_data[..]);
        self.indices.set_data(&index_data[..]);

        self.vertices.bind();
        let len = self.vertices.len();
        self.vertices.upload_data(0, len, true);

        self.indices.bind();
        let len = self.indices.len();
        self.indices.upload_data(0, len, true);
    }
}

impl Drawable for Circle {
    fn draw_transformed(&self, view_matrix: &Matrix4<f32>) {
        render_target::draw_indices(
            self.draw_mode.clone(),
            &self.vertices,
            &self.indices,
            self.render_states(),
            view_matrix,
        );
    }
}
