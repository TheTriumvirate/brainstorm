
mod spherefieldprovider;
pub use self::spherefieldprovider::SphereFieldProvider;

pub type Field = Vec<Vec<Vec<(f32, f32, f32)>>>;

pub trait FieldProvider {
    fn new() -> Self;
    fn delta(&self, pos: (f32, f32, f32)) -> (f32, f32, f32);
}