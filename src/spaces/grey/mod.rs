//! Monochrome colour representation.

use num_traits::Float;

mod colour;
mod convert;
mod fmt;

/// Monochrome colour.
#[derive(Debug, Clone, Copy)]
pub struct Grey<T: Float + Send + Sync> {
    /// Grey component.
    grey: T,
}

impl<T: Float + Send + Sync> Grey<T> {
    /// Create a new `Grey` instance.
    #[inline]
    pub fn new(grey: T) -> Self {
        debug_assert!(
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
    #[inline]
    pub fn set_grey(&mut self, grey: T) {
        debug_assert!(
            grey >= T::zero() && grey <= T::one(),
            "Grey component must be between 0 and 1."
        );
        self.grey = grey;
    }
}
