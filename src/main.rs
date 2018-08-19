extern crate noise;
extern crate particles;
extern crate rand;
extern crate nalgebra as na;

use noise::RidgedMulti;
use particles::window::*;
use particles::*;
use rand::rngs::SmallRng;
use rand::{FromEntropy, Rng};
use std::f32;

use na::{Isometry3, Matrix4, Perspective3, Point3, Vector2, Vector3};

const PARTICLE_COUNT: usize = 100_000;

fn main() {
    // Set up window and shaders.
    let mut window = Window::new("Particles!", 1000, 1000);
    let program = configure_shaders(&mut window);

    // Set up particles.
    let mut data = Vec::new();
    for i in 0..PARTICLE_COUNT {
        let x: f32 = (i as f32) / PARTICLE_COUNT as f32 - 0.5;
        data.push(x);
        data.push(-0.5);
        data.push(0.0);
    }

    // Bind the window buffer.
    let vb = window
        .create_buffer()
        .expect("Failed to create window buffer.");
    window.bind_buffer(Window::ARRAY_BUFFER, &vb);
    window.buffer_data(Window::ARRAY_BUFFER, &data, Window::DYNAMIC_DRAW);

    // Bind the vertex array.
    let vao = window
        .create_vertex_array()
        .expect("Failed to create vertex array.");
    window.bind_vertex_array(&vao);

    // Enable the attribute arrays.
    let pos_attrib = window.get_attrib_location(&program, "position");
    window.vertex_attrib_pointer(&pos_attrib, 3, Window::FLOAT, false, 3, 0);
    window.enable_vertex_attrib_array(&pos_attrib);

    let mut mvp_uniform = window.get_uniform_location(&program, "MVP");

    // Run main loop.
    let mut noise = RidgedMulti::new();
    let mut time: f32 = 0.0;
    let mut rng = SmallRng::from_entropy();
    let mut running = true;

    Window::run_loop(move |_| run(&mut window, &mut data, &mut noise, &mut time, &mut rng, &mut running, &mut mvp_uniform));
    /* TODO: cleanup resources... not easy bcs of window move into loop
    window.delete_vertex_array(&vao);
    window.delete_buffer(&vb);
    window.delete_program(&program);
    window.delete_shader(&vs);
    window.delete_shader(&fs);
    */
}

fn run(
    window: &mut Window,
    data: &mut Vec<f32>,
    noise: &mut RidgedMulti,
    time: &mut f32,
    rng: &mut SmallRng,
    running: &mut bool,
    mvp_uniform: &mut UniformLocation
) -> bool {
    window.handle_events(|event| {
        //println!("Event: {:?}", event);
        match event {
            Event::KeyboardInput{pressed: true, key: Key::W, modifiers: ModifierKeys{ctrl: true, ..}} 
                | Event::Quit => *running = false,
            _ => ()
        }
    });
    for i in 0..PARTICLE_COUNT {
        if rng.gen_bool(0.02) {
            data[i * 3] = rng.gen_range::<f32>(-0.5, 0.5);
            data[i * 3 + 1] = rng.gen_range::<f32>(-0.5, -0.47);
            data[i * 3 + 2] = 0.0;
        }
        let (dx, dy, dz) = field((data[i * 3], data[i * 3 + 1], data[i * 3 + 2]), &noise, *time);
        data[i * 3] += dx * 0.001;
        data[i * 3 + 1] += dy * 0.001;
        data[i * 3 + 2] += dz * 0.001;
    }

    
    let perspective = Perspective3::new(1.0, f32::consts::PI / 4.0, 0.1, 1024.0);
    let eye = Point3::new(time.sin() * 2.0, 0.75, time.cos() * 2.0);
    let at = Point3::new(0.0, 0.0, 0.0);
    let view: Isometry3<f32> = Isometry3::look_at_rh(&eye, &at, &Vector3::y());
    let projection_matrix = perspective.as_matrix() * view.to_homogeneous();
    window.uniform_matrix_4fv(&mvp_uniform, 1, false, &projection_matrix);

    window.buffer_data(Window::ARRAY_BUFFER, &data, Window::DYNAMIC_DRAW);
    window.clear_color(0.0, 0.0, 0.0, 1.0);
    window.clear(Window::COLOR_BUFFER_BIT);
    window.draw_arrays(Window::POINTS, 0, PARTICLE_COUNT as i32);
    window.swap_buffers();
    *time += 0.01;
    *running
}

fn configure_shaders(window: &mut Window) -> Program {
    // Compile vertex shader
    let vertex_shader =
        std::str::from_utf8(shaders::VERTEX_SHADER).expect("Failed to load vertex shader.");
    let vs = window
        .create_shader(ShaderType::Vertex)
        .expect("Failed to create vertex shader.");
    window.shader_source(&vs, vertex_shader);
    window.compile_shader(&vs);

    if let Some(log) = window.get_shader_info_log(&vs) {
        println!("vertex shader log: {}", log);
    }

    // Compile fragment shader
    let fragment_shader =
        std::str::from_utf8(shaders::FRAGMENT_SHADER).expect("Failed to load fragment shader.");
    let fs = window
        .create_shader(ShaderType::Fragment)
        .expect("Failed to create fragment shader.");
    window.shader_source(&fs, fragment_shader);
    window.compile_shader(&fs);
    if let Some(log) = window.get_shader_info_log(&fs) {
        println!("fragment shader log: {}", log);
    }

    // Link program
    let program = window
        .create_program()
        .expect("Failed to create shader program.");
    window.attach_shader(&program, &vs);
    window.attach_shader(&program, &fs);
    window.link_program(&program);
    window.use_program(&program);

    program
}
