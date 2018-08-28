/*
 * Very inspired from kiss3d's implementation of window and context
 * link: https://github.com/sebcrozet/kiss3d
 */

#[cfg(not(target_arch = "wasm32"))]
use window::opengl as Window;

use window::Event as EventWrapper;

pub trait AbstractWindow {

    fn new(title: &str, width: u32, height: u32) -> Self;
    fn run_loop(callback: impl FnMut(f64) -> bool + 'static);
    fn get_events(&mut self) -> Vec<EventWrapper>;
    fn swap_buffers(&mut self);
}
