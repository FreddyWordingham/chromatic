//! ## `ColourMap` Module
//!
//! This module provides the `ColourMap` struct, which allows for interpolation between colours.

use num_traits::{Float, NumCast};
use std::{
    any::type_name,
    fmt::{Display, Formatter, Result as FmtResult},
    marker::PhantomData,
};
use terminal_size::{Width, terminal_size};

use crate::{
    Colour,
    error::{ColourMapError, NumericError, Result},
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
    /// # Errors
    ///
    /// Returns an error if the sampled position is outside the range [0, 1],
    /// or if numeric conversion fails during interpolation calculations.
    pub fn sample(&self, position: T) -> Result<C> {
        if position < T::zero() || position > T::one() {
            return Err(ColourMapError::InvalidSamplingPosition {
                position: NumCast::from(position).ok_or_else(|| NumericError::TypeConversionFailed {
                    from: type_name::<T>().to_string(),
                    to: "f64".to_string(),
                    reason: "Failed to convert position for error reporting".to_string(),
                })?,
            }
            .into());
        }

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

        // Calculate which segment we're in
        let segments = T::from(self.colours.len() - 1).ok_or_else(|| NumericError::TypeConversionFailed {
            from: "usize".to_string(),
            to: type_name::<T>().to_string(),
            reason: "Failed to convert segment count".to_string(),
        })?;

        let scaled_pos = position * segments;
        let segment_idx = scaled_pos
            .floor()
            .to_usize()
            .ok_or_else(|| NumericError::TypeConversionFailed {
                from: type_name::<T>().to_string(),
                to: "usize".to_string(),
                reason: "Failed to convert segment index".to_string(),
            })?
            .min(self.colours.len() - 2);

        // Calculate interpolation parameter within the segment
        let segment_start = T::from(segment_idx).ok_or_else(|| NumericError::TypeConversionFailed {
            from: "usize".to_string(),
            to: type_name::<T>().to_string(),
            reason: "Failed to convert segment start".to_string(),
        })? / segments;

        let segment_width = T::one() / segments;
        let t = (position - segment_start) / segment_width;

        C::lerp(&self.colours[segment_idx], &self.colours[segment_idx + 1], t)
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
}

impl<C, T, const N: usize> Display for ColourMap<C, T, N>
where
    C: Display + Clone + Colour<T, N>,
    T: Float + Send + Sync,
{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        let width = terminal_size().map_or(60, |(Width(w), _)| w);
        let denom = width.saturating_sub(1).max(1);
        for i in 0..width {
            // Handle numeric conversion error in Display - convert to fmt::Error
            let t = T::from(i).ok_or(std::fmt::Error)? / T::from(denom).ok_or(std::fmt::Error)?;

            match self.sample(t) {
                Ok(colour) => write!(fmt, "{colour}")?,
                Err(_) => return Err(std::fmt::Error),
            }
        }
        Ok(())
    }
}
