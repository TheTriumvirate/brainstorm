extern crate particles;
use particles::*;
use particles::window::AbstractWindow;

fn main() {
    let mut app = App::new();
    particles::window::Window::run_loop(move |_| app.run());
    /* TODO: cleanup resources... not easy bcs of window move into loop
    window.delete_vertex_array(&vao);
    window.delete_buffer(&vb);
    window.delete_program(&program);
    window.delete_shader(&vs);
    window.delete_shader(&fs);
    */
}
