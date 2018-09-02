// #![windows_subsystem = "windows"]

extern crate particles;
use particles::window::AbstractWindow;
use particles::*;

fn main() {
    let mut app = App::new();
    particles::window::Window::run_loop(move |_| app.run());
    /* TODO: cleanup resources... not easy bcs of window move into loop
    window.delete_vertex_array(&vao);
    window.delete_buffer(&vb);    
    */
}
