extern crate particles;
extern crate gl_context;

use gl_context::window::AbstractWindow;
use particles::App;

fn main() {
    let mut app = App::new();
    gl_context::window::Window::run_loop(move |_| app.run());
}
