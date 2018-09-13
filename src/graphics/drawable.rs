use super::RenderTarget;

pub trait Drawable {
    fn draw(&self, target: impl RenderTarget);
}