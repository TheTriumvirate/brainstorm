//! Contains the providers of particle fields.

mod datafieldprovider;
pub use self::datafieldprovider::DataFieldProvider;

/// Defines the basice interactions with a particle field provider.
pub trait FieldProvider {
    /// Initializes a new particle field.
    fn new() -> Self;

    /// Provides the velocity of a particle that finds itself at a
    /// given position.
    /// NOTE: fourth value is FA value
    fn delta(&self, pos: (f32, f32, f32)) -> (f32, f32, f32, f32);

    /// Provdes immutable access to the raw data of the field.
    fn data(&self) -> &[(f32, f32, f32, f32)];

    /// Return the width of the vector field
    fn width(&self) -> usize;

    /// Return the height of the vector field
    fn height(&self) -> usize;

    /// Return the depth of the vector field
    fn depth(&self) -> usize;

    /// Return the vector at data[x][y][z]
    /// NOTE: fourth value is FA value
    fn get(&self, x: usize, y: usize, z: usize) -> (f32, f32, f32, f32) {
        self.data()[z + y * self.width() + x * self.width() * self.height()]
    }
}
