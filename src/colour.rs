/// Common trait for all color types
pub trait Colour<T> {
    /// Linearly interpolate between two colors
    fn lerp(&self, other: &Self, t: T) -> Self;
}
