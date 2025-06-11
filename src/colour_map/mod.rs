//! ## `ColourMap` Module
//!
//! This module provides the `ColourMap` struct, which allows for interpolation between colours
//! with uniformly spaced positions.

mod fmt;

use num_traits::Float;
use std::marker::PhantomData;

use crate::Colour;

/// A map of colours with uniform spacing, allowing interpolation between them.
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
    /// Create a new colour map from a list of colours with uniform spacing.
    ///
    /// # Panics
    ///
    /// Panics if the colours slice is empty.
    #[must_use]
    #[inline]
    pub fn new(colours: &[C]) -> Self {
        assert!(!colours.is_empty(), "Colour map must have at least one colour.");
        Self {
            colours: colours.to_vec(),
            _phantom: PhantomData,
        }
    }

    /// Sample the colour map at a given position.
    ///
    /// # Panics
    ///
    /// Panics if the position is not in the range [0, 1].
    #[inline]
    pub fn sample(&self, position: T) -> C {
        // Single colour case
        if self.colours.len() == 1 {
            return self.colours[0].clone();
        }

        // Edge cases
        if position <= T::zero() {
            return self.colours[0].clone();
        }
        if position >= T::one() {
            return self.colours.last().unwrap().clone();
        }

        // Calculate which segment we're in
        let segments = T::from(self.colours.len() - 1).unwrap();
        let scaled_pos = position * segments;
        let segment_idx = scaled_pos.floor().to_usize().unwrap().min(self.colours.len() - 2);

        // Calculate interpolation parameter within the segment
        let segment_start = T::from(segment_idx).unwrap() / segments;
        let segment_width = T::one() / segments;
        let t = (position - segment_start) / segment_width;

        C::lerp(&self.colours[segment_idx], &self.colours[segment_idx + 1], t)
    }

    /// Get the number of control points in the `ColourMap`.
    #[expect(clippy::len_without_is_empty, reason = "ColourMaps should never be empty.")]
    #[must_use]
    #[inline]
    pub const fn len(&self) -> usize {
        self.colours.len()
    }

    /// Get a reference to the colours in the map.
    #[must_use]
    #[inline]
    pub fn colours(&self) -> &[C] {
        &self.colours
    }
}
