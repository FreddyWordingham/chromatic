//! Monochrome colour representation with transparency.

use num_traits::Float;

mod colour;
mod convert;
mod eq;
mod fmt;

/// Monochrome colour with transparency.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GreyAlpha<T: Float> {
    /// Grey component.
    grey: T,
    /// Alpha component.
    alpha: T,
}

impl<T: Float> GreyAlpha<T> {
    /// Create a new `GreyAlpha` instance.
    ///
    /// # Panics
    ///
    /// Panics if any component is not in [0, 1].
    #[inline]
    pub fn new(grey: T, alpha: T) -> Self {
        assert!(
            !(grey < T::zero() || grey > T::one()),
            "Grey component must be between 0 and 1."
        );
        assert!(
            !(alpha < T::zero() || alpha > T::one()),
            "Alpha component must be between 0 and 1."
        );
        Self { grey, alpha }
    }

    /// Get the grey component.
    #[inline]
    pub const fn grey(&self) -> T {
        self.grey
    }

    /// Get the alpha component.
    #[inline]
    pub const fn alpha(&self) -> T {
        self.alpha
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

    /// Set the alpha component.
    ///
    /// # Panics
    ///
    /// Panics if the value is not in [0, 1].
    #[inline]
    pub fn set_alpha(&mut self, alpha: T) {
        assert!(
            alpha >= T::zero() && alpha <= T::one(),
            "Alpha component must be between 0 and 1."
        );
        self.alpha = alpha;
    }
}
