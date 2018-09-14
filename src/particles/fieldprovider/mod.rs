mod spherefieldprovider;
pub use self::spherefieldprovider::SphereFieldProvider;

pub trait FieldProvider {
    fn new() -> Self;
    fn delta(&self, pos: (f32, f32, f32)) -> (f32, f32, f32);
    fn data(&self) -> &[(f32, f32, f32)];
}
