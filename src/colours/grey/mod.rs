//! Monochrome colour representation.

use num_traits::Float;

mod colour;
mod convert;
mod eq;
mod fmt;

/// Monochrome colour.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Grey<T: Float> {
    /// Grey component.
    grey: T,
}

impl<T: Float> Grey<T> {
    /// Create a new `Grey` instance.
    ///
    /// # Panics
    ///
    /// Panics if the component is not in [0, 1].
    #[inline]
    pub fn new(grey: T) -> Self {
        assert!(
            !(grey < T::zero() || grey > T::one()),
            "Grey component must be between 0 and 1."
        );
        Self { grey }
    }

    /// Get the grey component.
    #[inline]
    pub const fn grey(&self) -> T {
        self.grey
    }

    /// Set the grey component.
    ///
    /// # Panics
    ///
    /// Panics if the value is not in [0, 1].
    #[inline]
    pub fn set_grey(&mut self, grey: T) {
        assert!(
            grey >= T::zero() && grey <= T::one(),
            "Grey component must be between 0 and 1."
        );
        self.grey = grey;
    }
}
