extern crate noise;
extern crate particles;
extern crate rand;

use noise::RidgedMulti;
use particles::window::*;
use particles::*;
use rand::rngs::SmallRng;
use rand::{FromEntropy, Rng};
use std::f32;

const PARTICLE_COUNT: usize = 50_000;

fn main() {
    // Set up window and shaders.
    let mut window = Window::new("Particles!", 1000, 1000);
    let program = configure_shaders(&mut window);

    // Set up particles.
    let mut data = Vec::new();
    let countsqrt: f32 = (PARTICLE_COUNT as f32).sqrt();
    for i in 0..PARTICLE_COUNT {
        let x: f32 = ((i as f32) % countsqrt) / countsqrt * 2.0 - 1.0;
        let y: f32 = ((i as f32) / countsqrt).floor() / countsqrt * 2.0 - 1.0;
        data.push(x);
        data.push(0.0);
        data.push(y.atan2(x).abs() / f32::consts::PI * 2.0 + 0.5);
        data.push(x / 4.0 + 0.25);
        data.push(y / 4.0 + 0.25);
        data.push(0.2);
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
    let color_attrib = window.get_attrib_location(&program, "color");
    window.vertex_attrib_pointer(&pos_attrib, 2, Window::FLOAT, false, 6, 0);
    window.vertex_attrib_pointer(&color_attrib, 4, Window::FLOAT, false, 6, 2);
    window.enable_vertex_attrib_array(&pos_attrib);
    window.enable_vertex_attrib_array(&color_attrib);

    // Run main loop.
    let mut noise = RidgedMulti::new();
    let mut time: f32 = 0.0;
    let mut rng = SmallRng::from_entropy();
    let mut running = true;

    Window::run_loop(move |_| run(&mut window, &mut data, &mut noise, &mut time, &mut rng, &mut running));
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
        if rng.gen_bool(0.01) {
            let angle = rng.gen_range::<f32>(0.0, 2.0 * f32::consts::PI);
            let dist = rng.gen_range::<f32>(-0.1, 0.1);
            data[i * 6] = angle.cos() * dist;
            data[i * 6 + 1] = angle.sin() * dist;
        }
        let (dx, dy) = field((data[i * 6], data[i * 6 + 1]), &noise, *time);
        data[i * 6] += dx * 0.001;
        data[i * 6 + 1] += dy * 0.001;
    }
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
