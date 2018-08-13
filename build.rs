#[cfg(target_arch = "wasm32")]
extern crate webgl_generator;

#[cfg(target_arch = "wasm32")]
use std::env;
#[cfg(target_arch = "wasm32")]
use std::fs::File;
#[cfg(target_arch = "wasm32")]
use std::path::*;
#[cfg(target_arch = "wasm32")]
use webgl_generator::*;

#[cfg(target_arch = "wasm32")]
fn main() {
    let dest = env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&dest).join("webgl_rendering_context.rs")).unwrap();

    Registry::new(Api::WebGl2, Exts::NONE)
        .write_bindings(StdwebGenerator, &mut file)
        .unwrap();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() { }
