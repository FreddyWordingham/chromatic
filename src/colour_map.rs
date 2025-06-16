//! ## `ColourMap` Module
//!
//! This module provides the `ColourMap` struct, which allows for interpolation between colours.

use num_traits::Float;
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    marker::PhantomData,
};
use terminal_size::{Width, terminal_size};

use crate::{
    Colour,
    error::{ColourMapError, Result, safe_constant, validate_interpolation_factor},
};

/// A map of colours at specific positions, with interpolation between them.
#[derive(Debug, Clone)]
pub struct ColourMap<C, T, const N: usize>
where
    C: Colour<T, N>,
    T: Float + Send + Sync,
{
    /// The colours in the map.
    colours: Vec<C>,
    /// Phantom type for the colour space.
    _phantom: PhantomData<T>,
}

impl<C, T, const N: usize> ColourMap<C, T, N>
where
    C: Clone + Colour<T, N>,
    T: Float + Send + Sync,
{
    /// Create a new colour map with uniformly spaced positions.
    ///
    /// # Arguments
    ///
    /// * `colours` - A slice of colours to create the map from. Must not be empty.
    ///
    /// # Errors
    ///
    /// Returns an error if the colour map is empty.
    pub fn new(colours: &[C]) -> Result<Self> {
        if colours.is_empty() {
            return Err(ColourMapError::EmptyColourMap.into());
        }

        Ok(Self {
            colours: colours.to_vec(),
            _phantom: PhantomData,
        })
    }

    /// Sample the colour map at a given position.
    ///
    /// # Arguments
    ///
    /// * `position` - The position to sample at, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if the sampled position is outside the range [0, 1],
    /// or if numeric conversion fails during interpolation calculations.
    pub fn sample(&self, position: T) -> Result<C> {
        // Validate sampling position using our standard helper
        validate_interpolation_factor(position)?;

        // Single colour case
        if self.colours.len() == 1 {
            return Ok(self.colours[0].clone());
        }

        // Edge cases - use exact comparisons since we've already validated the range
        if position <= T::zero() {
            return Ok(self.colours[0].clone());
        }
        if position >= T::one() {
            return Ok(self.colours[self.colours.len() - 1].clone());
        }

        // Calculate which segment we're in
        let segments = safe_constant::<usize, T>(self.colours.len() - 1)?;
        let scaled_pos = position * segments;

        // Get segment index, ensuring it's within bounds
        let segment_idx = scaled_pos
            .floor()
            .to_usize()
            .ok_or_else(|| ColourMapError::InvalidSamplingPosition {
                position: position.to_f64().unwrap_or(f64::NAN),
            })?
            .min(self.colours.len() - 2);

        // Calculate interpolation parameter within the segment
        let segment_start = safe_constant::<usize, T>(segment_idx)? / segments;
        let segment_width = T::one() / segments;
        let t = (position - segment_start) / segment_width;

        // Perform the interpolation
        C::lerp(&self.colours[segment_idx], &self.colours[segment_idx + 1], t)
    }

    /// Create a colour map from colours with explicit positions.
    ///
    /// # Arguments
    ///
    /// * `colours_and_positions` - A slice of (colour, position) tuples where positions must be in [0, 1] and sorted
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The input is empty
    /// - Any position is outside [0, 1]
    /// - Positions are not in ascending order
    pub fn from_positions(colours_and_positions: &[(C, T)]) -> Result<Self> {
        if colours_and_positions.is_empty() {
            return Err(ColourMapError::EmptyColourMap.into());
        }

        // Validate positions
        for (i, (_, position)) in colours_and_positions.iter().enumerate() {
            validate_interpolation_factor(*position)?;

            if i > 0 {
                let prev_position = colours_and_positions[i - 1].1;
                if *position <= prev_position {
                    return Err(ColourMapError::NonAscendingPositions {
                        pos1: prev_position.to_f64().unwrap_or(f64::NAN),
                        idx1: i - 1,
                        pos2: position.to_f64().unwrap_or(f64::NAN),
                        idx2: i,
                    }
                    .into());
                }
            }
        }

        // For now, we'll store just the colours and use uniform spacing
        // A future enhancement could store the positions as well
        let colours: Vec<C> = colours_and_positions.iter().map(|(c, _)| c.clone()).collect();
        Ok(Self {
            colours,
            _phantom: PhantomData,
        })
    }

    /// Sample the colour map at a given position with custom interpolation.
    ///
    /// # Arguments
    ///
    /// * `position` - The position to sample at, must be in range [0, 1]
    /// * `interpolation_fn` - Custom interpolation function
    ///
    /// # Errors
    ///
    /// Returns an error if the position is invalid or interpolation fails.
    pub fn sample_with<F>(&self, position: T, interpolation_fn: F) -> Result<C>
    where
        F: Fn(&C, &C, T) -> Result<C>,
    {
        validate_interpolation_factor(position)?;

        // Single colour case
        if self.colours.len() == 1 {
            return Ok(self.colours[0].clone());
        }

        // Edge cases
        if position <= T::zero() {
            return Ok(self.colours[0].clone());
        }
        if position >= T::one() {
            return Ok(self.colours[self.colours.len() - 1].clone());
        }

        // Calculate segment and interpolate using custom function
        let segments = safe_constant::<usize, T>(self.colours.len() - 1)?;
        let scaled_pos = position * segments;

        let segment_idx = scaled_pos
            .floor()
            .to_usize()
            .ok_or_else(|| ColourMapError::InvalidSamplingPosition {
                position: position.to_f64().unwrap_or(f64::NAN),
            })?
            .min(self.colours.len() - 2);

        let segment_start = safe_constant::<usize, T>(segment_idx)? / segments;
        let segment_width = T::one() / segments;
        let t = (position - segment_start) / segment_width;

        interpolation_fn(&self.colours[segment_idx], &self.colours[segment_idx + 1], t)
    }

    /// Get the number of control points in the `ColourMap`.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.colours.len()
    }

    /// Check if the `ColourMap` is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.colours.is_empty()
    }

    /// Get a reference to the colours in the map.
    #[must_use]
    pub fn colours(&self) -> &[C] {
        &self.colours
    }

    /// Get an iterator over the colours in the map.
    pub fn iter(&self) -> std::slice::Iter<'_, C> {
        self.colours.iter()
    }

    /// Generate a vector of sampled colours across the entire map.
    ///
    /// # Arguments
    ///
    /// * `num_samples` - Number of samples to generate
    ///
    /// # Errors
    ///
    /// Returns an error if sampling fails or if num_samples is 0.
    pub fn sample_n(&self, num_samples: usize) -> Result<Vec<C>> {
        if num_samples == 0 {
            return Err(ColourMapError::InvalidSamplingPosition { position: 0.0 }.into());
        }

        if num_samples == 1 {
            return Ok(vec![self.sample(T::zero())?]);
        }

        let mut samples = Vec::with_capacity(num_samples);
        let denominator = safe_constant::<usize, T>(num_samples - 1)?;

        for i in 0..num_samples {
            let position = safe_constant::<usize, T>(i)? / denominator;
            samples.push(self.sample(position)?);
        }

        Ok(samples)
    }
}

impl<C, T, const N: usize> Display for ColourMap<C, T, N>
where
    C: Display + Clone + Colour<T, N>,
    T: Float + Send + Sync,
{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        let width = terminal_size().map_or(60, |(Width(w), _)| w).min(200); // Cap at reasonable width

        let denom = width.saturating_sub(1).max(1);

        for i in 0..width {
            // Use our safe conversion helpers
            let position = match (safe_constant::<u16, T>(i), safe_constant::<u16, T>(denom)) {
                (Ok(i_t), Ok(denom_t)) => i_t / denom_t,
                _ => return Err(std::fmt::Error),
            };

            match self.sample(position) {
                Ok(colour) => write!(fmt, "{colour}")?,
                Err(_) => return Err(std::fmt::Error),
            }
        }
        Ok(())
    }
}

// Implement IntoIterator for easier usage
impl<C, T, const N: usize> IntoIterator for ColourMap<C, T, N>
where
    C: Colour<T, N>,
    T: Float + Send + Sync,
{
    type Item = C;
    type IntoIter = std::vec::IntoIter<C>;

    fn into_iter(self) -> Self::IntoIter {
        self.colours.into_iter()
    }
}

impl<'a, C, T, const N: usize> IntoIterator for &'a ColourMap<C, T, N>
where
    C: Colour<T, N>,
    T: Float + Send + Sync,
{
    type Item = &'a C;
    type IntoIter = std::slice::Iter<'a, C>;

    fn into_iter(self) -> Self::IntoIter {
        self.colours.iter()
    }
}
