#![allow(unused_parens, non_camel_case_types)]

#[allow(unused_imports)]
#[macro_use]
extern crate serde_derive;
extern crate serde;

#[macro_use]
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
extern crate stdweb;
#[macro_use]
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
extern crate stdweb_derive;

pub mod shaders;
pub mod window;
