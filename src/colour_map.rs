//! ## `ColourMap` Module
//!
//! This module provides the `ColourMap` struct, which allows for interpolation between colours.

use core::fmt::{Display, Formatter, Result as FmtResult};
use num_traits::Float;
use terminal_size::{Width, terminal_size};

use crate::Colour;

/// A map of colours at specific positions, with interpolation between them.
#[derive(Debug, Clone)]
pub struct ColourMap<C, T, const N: usize>
where
    C: Colour<T, N>,
    T: Float,
{
    /// The colours in the map.
    colours: Vec<C>,
    /// The positions of the colours in the map, in range [0, 1].
    positions: Vec<T>,
}

impl<C, T, const N: usize> ColourMap<C, T, N>
where
    C: Clone + Colour<T, N>,
    T: Float,
{
    /// Create a new colour map from a list of colours and positions.
    ///
    /// # Panics
    ///
    /// Panics if the lists are empty.
    /// Panics if the lists have different lengths.
    /// Panics if the positions are not in ascending order.
    /// Panics if any position is outside the range [0, 1].
    #[must_use]
    #[inline]
    pub fn new(colours: &[C], positions: &[T]) -> Self {
        assert!(!colours.is_empty(), "Colour map must have at least one colour.");
        assert_eq!(
            colours.len(),
            positions.len(),
            "Colour map must have the same number of colours and positions."
        );

        // Validate positions are in range [0, 1] and ascending
        for position in positions {
            assert!(
                *position >= T::zero() && *position <= T::one(),
                "Positions must be in range [0, 1]."
            );
        }

        for i in 1..positions.len() {
            assert!(positions[i] > positions[i - 1], "Positions must be in ascending order.");
        }

        Self {
            colours: colours.to_vec(),
            positions: positions.to_vec(),
        }
    }

    /// Create a new colour map with uniformly spaced positions.
    ///
    /// # Panics
    ///
    /// Panics if the list of colours is empty.
    #[must_use]
    #[inline]
    pub fn new_uniform(colours: &[C]) -> Self {
        assert!(!colours.is_empty(), "Colour map must have at least one colour.");
        if colours.len() == 1 {
            return Self::new(colours, &[T::zero()]);
        }
        let positions = (0..colours.len())
            .map(|i| T::from(i).unwrap() / T::from(colours.len() - 1).unwrap())
            .collect::<Vec<_>>();
        Self::new(colours, &positions)
    }

    /// Sample the colour map at a given position.
    ///
    /// # Panics
    ///
    /// Panics if the position is outside the range [0, 1].
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[must_use]
    #[inline]
    pub fn sample(&self, position: T) -> C {
        assert!(
            position >= T::zero() && position <= T::one(),
            "Sample position must be in range [0, 1]."
        );

        // Handle edge cases
        if position <= self.positions[0] {
            return self.colours[0].clone();
        }

        if position >= *self.positions.last().unwrap() {
            return self.colours.last().unwrap().clone();
        }

        // Find the segment containing the position
        let mut segment_index = 0;
        for i in 1..self.positions.len() {
            if position <= self.positions[i] {
                segment_index = i - 1;
                break;
            }
        }

        // Calculate interpolation factor within segment
        let segment_start = self.positions[segment_index];
        let segment_end = self.positions[segment_index + 1];
        let segment_t = (position - segment_start) / (segment_end - segment_start);

        // Use the Colour trait's lerp method for interpolation
        C::lerp(&self.colours[segment_index], &self.colours[segment_index + 1], segment_t)
    }

    /// Get the number of control points in the `ColourMap`.
    #[expect(clippy::len_without_is_empty, reason = "A colour map can never be empty.")]
    #[expect(
        clippy::missing_const_for_fn,
        reason = "`Vec::<T, A>::len` is not yet stable as a const fn."
    )]
    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.colours.len()
    }

    /// Get a reference to the colours in the map.
    #[must_use]
    #[inline]
    pub fn colours(&self) -> &[C] {
        &self.colours
    }

    /// Get a reference to the positions in the map.
    #[must_use]
    #[inline]
    pub fn positions(&self) -> &[T] {
        &self.positions
    }
}

impl<C, T, const N: usize> Display for ColourMap<C, T, N>
where
    C: Display + Clone + Colour<T, N>,
    T: Float,
{
    #[expect(
        clippy::min_ident_chars,
        reason = "Variable `f` for `Formatter` is idiomatic. Variable `t` for an interpolation factor is idiomatic."
    )]
    #[expect(clippy::unwrap_in_result, reason = "Unwrap will not fail here.")]
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some((Width(mut width), _)) = terminal_size() {
            width = width.max(1);
            for i in 0..width {
                let t = T::from(i).unwrap() / T::from((width - 1).max(1)).unwrap();
                let colour = self.sample(t);
                write!(f, "{colour}")?;
            }
            Ok(())
        } else {
            write!(f, "cmap")
        }
    }
}
