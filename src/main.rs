extern crate brainstorm;
extern crate gl_bindings;
extern crate window;

#[cfg(not(target_arch = "wasm32"))]
extern crate structopt;

use window::AbstractWindow;
use brainstorm::App;

#[cfg(not(target_arch = "wasm32"))]
use structopt::StructOpt;
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

#[cfg(target_arch = "wasm32")]
fn main() {
    let mut app = App::new(None);
    window::Window::run_loop(move |_| app.run());
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
    window::Window::run_loop(move |_| app.run());
}
