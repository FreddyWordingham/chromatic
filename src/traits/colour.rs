//! Trait implemented by all colour types.

use num_traits::Float;
use std::ops::AddAssign;

use crate::error::ChromaticError;

/// Common trait for all colour types.
pub trait Colour<T: Float + Send + Sync, const N: usize> {
    /// Number of components in the colour.
    const NUM_COMPONENTS: usize = N;

    /// Create a new colour from a hex string.
    ///
    /// # Errors
    ///
    /// Returns an error if the hex string is invalid or out of range.
    fn from_hex(hex: &str) -> Result<Self, ChromaticError>
    where
        Self: Sized;

    /// Convert the colour to a hex string.
    #[must_use]
    fn to_hex(&self) -> String;

    /// Create a new colour from a byte array.
    #[must_use]
    fn from_bytes(bytes: [u8; N]) -> Self;

    /// Convert the colour to a byte array.
    #[must_use]
    fn to_bytes(self) -> [u8; N];

    /// Linear interpolate between two colours of the same type.
    #[must_use]
    fn lerp(lhs: &Self, rhs: &Self, t: T) -> Self;

    /// Mix N by folding lerp (assumes weights sum to 1).
    ///
    /// # Panics
    ///
    /// Panics if the list of colours is empty.
    /// Panics if the lengths of colours and weights do not match.
    /// Panics if any weight is negative.
    #[must_use]
    fn mix(colours: &[Self], weights: &[T]) -> Self
    where
        Self: Clone,
        T: AddAssign,
    {
        debug_assert!(!colours.is_empty(), "Cannot mix an empty list of colours.");
        debug_assert_eq!(colours.len(), weights.len(), "Colours and weights must have the same length.");
        debug_assert!(weights.iter().all(|&w| w >= T::zero()), "Weights must be non-negative.");

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
