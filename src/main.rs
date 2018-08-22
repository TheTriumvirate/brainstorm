extern crate particles;
use particles::*;

fn main() {
    App::new().run();
    /* TODO: cleanup resources... not easy bcs of window move into loop
    window.delete_vertex_array(&vao);
    window.delete_buffer(&vb);    
    */
}
