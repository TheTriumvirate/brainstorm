
extern crate particles;
extern crate noise;

use particles::window::*;
use std::f32;
use noise::{NoiseFn, Fbm, Perlin, RidgedMulti };

const PARTICLE_COUNT: usize = 50_000; 

// Assumes normalized vector return
fn field((x, y):(f32, f32), noise: &RidgedMulti , time: &f32) -> (f32, f32) {
    let angle = noise.get([x as f64, y as f64, *time as f64]) as f32 * f32::consts::PI * 2.0;
    let strength = noise.get([x as f64 + 4000.0, y as f64 + 2000.0, *time as f64]).abs() as f32 * 9.0 + 1.0;
    (angle.cos() * strength, angle.sin() * strength + 20.0 * (y.abs() + 0.5))
    /*let dx:f32 = (y*10.0 + *time).cos();
    let dy:f32 = 0.0;
    let angle = dy.atan2(dx);
    let strength = 1.0;
    (angle.cos() /strength, angle.sin() / strength)*/
}

// psudorandom function because rand does not work in web D:
fn random(x: f32) -> f32 {
    (x.sin() * 43758.5453123).abs() % 1.0
}

fn run(window: &mut Window, data: &mut Vec<f32>, noise: &mut RidgedMulti , time: &mut f32) -> bool {
    for i in 0..PARTICLE_COUNT {
        if random(i as f32 * *time) < 0.01 {
            let angle = random(i as f32 * 123.29843 + 182937.12983) as f32 * f32::consts::PI * 2.0;
            let dist = random(i as f32 * 298347.9832 + 89475.9823) as f32 * 0.2 - 0.1;
            data[i*6] = angle.cos() * dist;
            data[i*6 + 1] = angle.sin() * dist;
        }
        let (dx, dy) = field((data[i*6], data[i*6+1]), &noise, time);
        data[i*6] += dx * 0.001;
        data[i*6+1] += dy * 0.001;
    }
    window.buffer_data(Window::ARRAY_BUFFER, &data, Window::DYNAMIC_DRAW);
    window.clear_color(0.0,0.0,0.0,1.0);
    window.clear(Window::COLOR_BUFFER_BIT);
    window.draw_arrays(Window::POINTS, 0, PARTICLE_COUNT as i32);
    window.swap_buffers();
    *time += 0.01;
    true
}

fn main() {
    let mut window = Window::new("test", 1000, 1000);

    let vs = window.create_shader(ShaderType::Vertex).unwrap();
    window.shader_source(&vs, VS_SRC);
    window.compile_shader(&vs);

    if window.get_shader_parameter(&vs, Window::COMPILE_STATUS) == Some(0) {
        let log = window.get_shader_info_log(&vs).unwrap();
        println!("vertex shader log: {}", log);
    }

    let fs = window.create_shader(ShaderType::Fragment).unwrap();
    window.shader_source(&fs, FS_SRC);
    window.compile_shader(&fs);
    if window.get_shader_parameter(&fs, Window::COMPILE_STATUS) == Some(0) {
        let log = window.get_shader_info_log(&fs).unwrap();
        println!("fragment shader log: {}", log);
    }

    let program = window.create_program().unwrap();
    window.attach_shader(&program, &vs);
    window.attach_shader(&program, &fs);
    window.link_program(&program);
    window.use_program(&program);

    let mut data = Vec::new();
    let countsqrt: f32 = (PARTICLE_COUNT as f32).sqrt();
    for i in 0..PARTICLE_COUNT {
        let x: f32 = ((i as f32) % countsqrt) / countsqrt * 2.0 - 1.0;
        let y: f32 = ((i as f32) / countsqrt).floor() / countsqrt * 2.0 - 1.0;
        data.push(x);
        data.push(0.0);
        data.push(y.atan2(x).abs()  / f32::consts::PI*2.0 + 0.5);
        data.push(x / 4.0 + 0.25);
        data.push(y / 4.0 + 0.25);
        data.push(0.2);
    }

    let vb = window.create_buffer().unwrap();
    window.bind_buffer(Window::ARRAY_BUFFER, &vb);
    window.buffer_data(Window::ARRAY_BUFFER, &data, Window::DYNAMIC_DRAW);

    let vao = window.create_vertex_array().unwrap();
    window.bind_vertex_array(&vao);

    let pos_attrib = window.get_attrib_location(&program, "position");
    let color_attrib = window.get_attrib_location(&program, "color");

    window.vertex_attrib_pointer(&pos_attrib, 2, Window::FLOAT, false, 6, 0);
    window.vertex_attrib_pointer(&color_attrib, 4, Window::FLOAT, false, 6, 2);

    window.enable_vertex_attrib_array(&pos_attrib);
    window.enable_vertex_attrib_array(&color_attrib);

    let mut noise = RidgedMulti::new();
    let mut time: f32 = 0.0;

    Window::run_loop(move |_| run(&mut window, &mut data, &mut noise, &mut time));
    /* TODO: cleanup resources... not easy bcs of window move into loop
    window.delete_vertex_array(&vao);
    window.delete_buffer(&vb);
    window.delete_program(&program);
    window.delete_shader(&vs);
    window.delete_shader(&fs);
    */
}

const VS_SRC: &'static str = "#version 300 es
precision mediump float;
in vec2 position;
in vec4 color;
out vec4 v_color;
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    gl_PointSize = 4.0;
    v_color = color;
}
";

const FS_SRC: &'static str = "#version 300 es
precision mediump float;
in vec4 v_color;
out vec4 o_color;
void main() {
    o_color = v_color;
}
";