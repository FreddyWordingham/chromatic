//! Monochrome colour representation.

use num_traits::Float;

/// Monochrome.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Grey<T: Float>(T);

impl<T: Float> Grey<T> {
    /// Create a new `Grey` instance.
    ///
    /// # Panics
    ///
    /// Panics if any of the components are not in the range [0, 1].
    #[inline]
    pub fn new(grey: T) -> Self {
        assert!(
            grey >= T::zero() && grey <= T::one(),
            "Grey component must be between 0 and 1"
        );
        Self(grey)
    }
}
