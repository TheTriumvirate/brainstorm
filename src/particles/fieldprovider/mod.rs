//! Contains the providers of particle fields.

mod spherefieldprovider;
pub use self::spherefieldprovider::SphereFieldProvider;

/// Defines the basice interactions with a particle field provider.
pub trait FieldProvider {
    /// Initializes a new particle field.
    fn new() -> Self;

    /// Provides the velocity of a particle that finds itself at a
    /// given position.
    fn delta(&self, pos: (f32, f32, f32)) -> (f32, f32, f32);

    /// Provdes immutable access to the raw data of the field.
    fn data(&self) -> &[(f32, f32, f32)];
}
