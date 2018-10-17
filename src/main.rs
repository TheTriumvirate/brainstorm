extern crate particles;
extern crate gl_context;

#[cfg(not(target_arch = "wasm32"))]
#[macro_use]
extern crate structopt;

use gl_context::window::AbstractWindow;
use particles::App;

#[cfg(not(target_arch = "wasm32"))]
use structopt::StructOpt;
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

#[cfg(target_arch = "wasm32")]
fn main() {
    let mut app = App::new(None);
    gl_context::window::Window::run_loop(move |_| app.run());
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(StructOpt, Debug)]
struct Opt {
    /// File to process
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let opt = Opt::from_args();
    
    if std::fs::File::open(&opt.file).is_err() {
        println!("File not found");
        return;
    }

    let mut app = App::new(Some(opt.file));
    gl_context::window::Window::run_loop(move |_| app.run());
}
