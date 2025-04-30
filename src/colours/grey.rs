//! Monochrome colour representation.

use core::fmt::Display;
use num_traits::Float;

/// Grey.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Grey<T: Float>(T);

impl<T: Float + Display> Grey<T> {
    /// Create a new `Grey` instance.
    ///
    /// # Panics
    ///
    /// Panics if the component is not in [0, 1].
    #[inline]
    pub fn new(mut grey: T) -> Self {
        let tol = T::epsilon();
        if grey < T::zero() - tol || grey > T::one() + tol {
            panic!("Grey component {} out of [0, 1]±{}", grey, tol);
        }
        grey = grey.max(T::zero()).min(T::one());
        Grey(grey)
    }

    /// Get the grey component.
    #[inline]
    pub const fn grey(&self) -> T {
        self.0
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
            "Grey component must be between 0 and 1"
        );
        self.0 = grey;
    }

    /// Get the tolerance for comparing grey values, which is 1/256.
    #[inline]
    fn tolerance() -> T {
        T::one() / T::from(256).unwrap()
    }
}

impl<T: Float + Display> PartialEq for Grey<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() <= Self::tolerance()
    }
}
