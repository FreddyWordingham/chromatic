//! Monochrome colour representation.

use num_traits::Float;

/// Grey.
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

    /// Get the grey component.
    #[inline]
    pub fn grey(&self) -> T {
        self.0
    }

    /// Set the grey component.
    #[inline]
    pub fn set_grey(&self) -> T {
        assert!(
            self.0 >= T::zero() && self.0 <= T::one(),
            "Grey component must be between 0 and 1"
        );
        self.0
    }
}
