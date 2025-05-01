//! ## `Colour` Module
//!
//! This module provides the `Colour` trait, which is implemented by all colour types.

use core::ops::AddAssign;
use num_traits::Float;

/// Common trait for all colour types.
pub trait Colour<T: AddAssign + Float, const N: usize> {
    /// Number of components in the colour.
    const NUM_COMPONENTS: usize = N;

    /// Create a new colour from components.
    #[must_use]
    fn from_components(components: [T; N]) -> Self;

    /// Get the components of the colour as a slice.
    #[must_use]
    fn components(&self) -> [T; N];

    /// Set the components of the colour from a slice.
    fn set_components(&mut self, components: [T; N]);

    /// Create a new colour from a byte array.
    #[must_use]
    fn from_bytes(bytes: [u8; N]) -> Self;

    /// Convert the colour to a byte array.
    #[must_use]
    fn to_bytes(self) -> [u8; N];

    /// Get the tolerance for comparing component values.
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[must_use]
    #[inline]
    fn tolerance() -> T {
        T::one() / T::from(256_i32).unwrap()
    }

    /// Linear interpolate between two greyalphas.
    #[must_use]
    fn lerp(lhs: &Self, rhs: &Self, t: T) -> Self;

    /// Mix N by folding lerp (assumes weights sum to 1).
    ///
    /// # Panics
    ///
    /// Panics if the list of colours is empty.
    /// Panics if the lengths of colours and weights do not match.
    /// Panics if any weight is negative.
    #[expect(
        clippy::min_ident_chars,
        reason = "The variable `t` for an interpolation factor is idiomatic."
    )]
    #[expect(
        clippy::missing_asserts_for_indexing,
        reason = "The indexing is safe due to the checks above."
    )]
    #[must_use]
    #[inline]
    fn mix(colours: &[Self], weights: &[T]) -> Self
    where
        Self: Sized + Clone,
    {
        assert!(!colours.is_empty(), "Cannot mix an empty list of colours.");
        assert_eq!(colours.len(), weights.len(), "Colours and weights must have the same length.");
        assert!(weights.iter().all(|&w| w >= T::zero()), "Weights must be non-negative.");

        // Handle the single colour case
        if colours.len() == 1 {
            return colours[0].clone();
        }

        // Create the accumulated result, starting with the first colour
        let mut result = colours[0].clone();
        let mut acc_weight = weights[0];

        // Progressively mix in each additional colour
        for i in 1..colours.len() {
            // Calculate the interpolation factor
            let t = weights[i] / (acc_weight + weights[i]);

            // Update the result with the interpolated value
            result = Self::lerp(&result, &colours[i], t);

            // Update the accumulated weight
            acc_weight += weights[i];
        }

        result
    }
}
