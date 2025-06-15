//! ## `ColourMap` Module
//!
//! This module provides the `ColourMap` struct, which allows for interpolation between colours.

use num_traits::{Float, NumCast};
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    marker::PhantomData,
};
use terminal_size::{Width, terminal_size};

use crate::{Colour, error::ColourMapError};

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
    pub fn new(colours: &[C]) -> Result<Self, ColourMapError> {
        if colours.is_empty() {
            return Err(ColourMapError::EmptyColourMap);
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
    /// Returns an error if the sampled position is outside the range [0, 1].
    pub fn sample(&self, position: T) -> Result<C, ColourMapError> {
        if position < T::zero() || position > T::one() {
            return Err(ColourMapError::InvalidSamplingPosition {
                position: NumCast::from(position).expect("Conversion to f64 failed"),
            });
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
            return Ok(self.colours.last().unwrap().clone());
        }

        // Calculate which segment we're in
        let segments = T::from(self.colours.len() - 1).unwrap();
        let scaled_pos = position * segments;
        let segment_idx = scaled_pos.floor().to_usize().unwrap().min(self.colours.len() - 2);

        // Calculate interpolation parameter within the segment
        let segment_start = T::from(segment_idx).unwrap() / segments;
        let segment_width = T::one() / segments;
        let t = (position - segment_start) / segment_width;

        Ok(C::lerp(&self.colours[segment_idx], &self.colours[segment_idx + 1], t))
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
            let t = T::from(i).unwrap() / T::from(denom).unwrap();
            // Handle the error gracefully instead of using ?
            match self.sample(t) {
                Ok(colour) => write!(fmt, "{colour}")?,
                Err(_) => return Err(std::fmt::Error), // Convert to fmt::Error
            }
        }
        Ok(())
    }
}
