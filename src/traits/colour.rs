//! Trait implemented by all colour types.

use num_traits::Float;
use std::ops::AddAssign;

use crate::error::{InterpolationError, Result};

/// Common trait for all colour types.
pub trait Colour<T: Float + Send + Sync, const N: usize> {
    /// Number of components in the colour.
    const NUM_COMPONENTS: usize = N;

    /// Create a new colour from a hex string.
    ///
    /// # Errors
    ///
    /// Returns an error if the hex string is invalid or out of range.
    fn from_hex(hex: &str) -> Result<Self>
    where
        Self: Sized;

    /// Convert the colour to a hex string.
    ///
    /// # Errors
    ///
    /// Returns an error if conversion fails or components are out of range.
    fn to_hex(&self) -> Result<String>;

    /// Create a new colour from a byte array.
    ///
    /// # Errors
    ///
    /// Returns an error if byte conversion fails or values are out of range.
    fn from_bytes(bytes: [u8; N]) -> Result<Self>
    where
        Self: Sized;

    /// Convert the colour to a byte array.
    ///
    /// # Errors
    ///
    /// Returns an error if conversion fails or components are out of range.
    fn to_bytes(self) -> Result<[u8; N]>;

    /// Linear interpolate between two colours of the same type.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left-hand side colour (at t=0)
    /// * `rhs` - The right-hand side colour (at t=1)
    /// * `t` - The interpolation factor, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if the interpolation factor is outside [0, 1] or if
    /// the interpolation calculation fails.
    fn lerp(lhs: &Self, rhs: &Self, t: T) -> Result<Self>
    where
        Self: Sized;

    /// Mix multiple colours using weighted interpolation.
    ///
    /// This method combines multiple colours using their associated weights.
    /// The weights do not need to sum to 1.0 - they will be normalized internally.
    ///
    /// # Arguments
    ///
    /// * `colours` - A slice of colours to mix, must not be empty
    /// * `weights` - A slice of weights corresponding to each colour, must be non-negative
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The colour list is empty
    /// - The lengths of colours and weights do not match
    /// - Any weight is negative
    /// - The sum of weights is zero or invalid
    /// - Interpolation calculations fail
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // Mix red and blue with equal weights
    /// let red = Rgb::new(1.0, 0.0, 0.0)?;
    /// let blue = Rgb::new(0.0, 0.0, 1.0)?;
    /// let purple = Rgb::mix(&[red, blue], &[1.0, 1.0])?;
    ///
    /// // Mix three colours with different weights
    /// let result = Rgb::mix(&[red, green, blue], &[0.5, 0.3, 0.2])?;
    /// ```
    fn mix(colours: &[Self], weights: &[T]) -> Result<Self>
    where
        Self: Clone,
        T: AddAssign,
    {
        // Validate inputs
        if colours.is_empty() {
            return Err(InterpolationError::EmptyColourList.into());
        }

        if colours.len() != weights.len() {
            return Err(InterpolationError::MismatchedArrayLengths {
                colours: colours.len(),
                weights: weights.len(),
            }
            .into());
        }

        // Validate all weights are non-negative
        for (i, &weight) in weights.iter().enumerate() {
            if weight < T::zero() {
                return Err(InterpolationError::NegativeWeight {
                    weight: weight.to_f64().unwrap_or(f64::NAN),
                    index: i,
                }
                .into());
            }
        }

        // Handle the single colour case
        if colours.len() == 1 {
            return Ok(colours[0].clone());
        }

        // Calculate total weight and validate it's not zero
        let total_weight = weights.iter().fold(T::zero(), |acc, &w| acc + w);
        if total_weight <= T::zero() {
            return Err(InterpolationError::InvalidWeightSum.into());
        }

        // Normalize weights and perform weighted mixing
        let mut result = colours[0].clone();
        let mut accumulated_normalized_weight = weights[0] / total_weight;

        for i in 1..colours.len() {
            let normalized_weight = weights[i] / total_weight;

            // Calculate interpolation factor for this step
            // t = current_weight / (accumulated_weight + current_weight)
            let denominator = accumulated_normalized_weight + normalized_weight;
            if denominator <= T::zero() {
                return Err(InterpolationError::InvalidWeightSum.into());
            }

            let t = normalized_weight / denominator;

            // Interpolate between current result and next colour
            result = Self::lerp(&result, &colours[i], t)?;

            // Update accumulated weight
            accumulated_normalized_weight += normalized_weight;
        }

        Ok(result)
    }

    /// Mix two colours with specified weights.
    ///
    /// This is a convenience method for mixing exactly two colours.
    ///
    /// # Arguments
    ///
    /// * `colour1` - The first colour
    /// * `weight1` - Weight for the first colour, must be non-negative
    /// * `colour2` - The second colour  
    /// * `weight2` - Weight for the second colour, must be non-negative
    ///
    /// # Errors
    ///
    /// Returns an error if weights are negative, sum to zero, or if interpolation fails.
    fn mix_two(colour1: &Self, weight1: T, colour2: &Self, weight2: T) -> Result<Self>
    where
        Self: Clone,
        T: AddAssign,
    {
        Self::mix(&[colour1.clone(), colour2.clone()], &[weight1, weight2])
    }

    /// Create a gradient between two colours with a specified number of steps.
    ///
    /// # Arguments
    ///
    /// * `start` - The starting colour
    /// * `end` - The ending colour
    /// * `steps` - Number of colours in the gradient (including start and end)
    ///
    /// # Errors
    ///
    /// Returns an error if steps is less than 2 or if interpolation fails.
    fn gradient(start: &Self, end: &Self, steps: usize) -> Result<Vec<Self>>
    where
        Self: Clone,
    {
        if steps < 2 {
            return Err(InterpolationError::InvalidGradientSteps { steps }.into());
        }

        if steps == 2 {
            return Ok(vec![start.clone(), end.clone()]);
        }

        let mut gradient = Vec::with_capacity(steps);
        let denominator = T::from(steps - 1).ok_or_else(|| InterpolationError::Math {
            operation: format!("Converting steps {steps} to target type"),
        })?;

        for i in 0..steps {
            let t = T::from(i).ok_or_else(|| InterpolationError::Math {
                operation: format!("Converting step index {i} to target type"),
            })? / denominator;

            gradient.push(Self::lerp(start, end, t)?);
        }

        Ok(gradient)
    }
}
