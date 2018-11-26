#[cfg(not(target_arch = "wasm32"))]
extern crate structopt;

extern crate brainstorm;
extern crate gl_bindings;
extern crate window;

#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;
#[cfg(not(target_arch = "wasm32"))]
use structopt::StructOpt;
use window::AbstractWindow;

/// Main entry point for the Web application.
#[cfg(target_arch = "wasm32")]
fn main() {
    let mut app = brainstorm::App::new(None, false);
    window::Window::run_loop(move |_| app.run());
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(StructOpt, Debug)]
struct Opt {
    /// File to process
    #[structopt(name = "FILE", parse(from_os_str))]
    file: Option<PathBuf>,

    /// Start with GPU particles instead of CPU particles.
    #[structopt(short = "g", long = "gpu")]
    gpu: bool,
}

/// Main entry point for the native application.
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let opt = Opt::from_args();

    let mut app = brainstorm::App::new(opt.file, opt.gpu);
    window::Window::run_loop(move |_| app.run());
}
