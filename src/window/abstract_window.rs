use window::Event as EventWrapper;

/// The "abstract interface" for a "window" in both web and desktop
/// environments.
pub trait AbstractWindow {
    /// Creates the "window".
    fn new(title: &str, width: u32, height: u32) -> Self;

    /// Starts the main loop. Only returns if the window is closed.
    /// Unless it's web, in which case it's unlikely to ever return.
    fn run_loop(callback: impl FnMut(f64) -> bool + 'static);

    /// Gets the queued events for the window.
    fn get_events(&mut self) -> Vec<EventWrapper>;

    /// Swaps the rendering buffers.
    fn swap_buffers(&mut self);

    /// Sets the size of the window.
    fn set_size(&mut self, width: u32, height: u32);

    /// Gets the size of the window.
    fn get_size(&self) -> (u32, u32);

    /// Enables depth testing
    fn enable_depth(&self);

    /// Disables depth testing
    fn disable_depth(&self);
}
