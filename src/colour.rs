//! Defines common traits for color types.

use num_traits::Float;

/// Common trait for all color types
pub trait Colour<T: Float> {
    /// Linearly interpolate between two colors
    #[must_use]
    fn lerp(&self, other: &Self, t: T) -> Self;
}
