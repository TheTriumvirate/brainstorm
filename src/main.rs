extern crate particles;

use particles::window::AbstractWindow;
use particles::*;

fn main() {
    let mut app = App::new();
    particles::window::Window::run_loop(move |_| app.run());
}
