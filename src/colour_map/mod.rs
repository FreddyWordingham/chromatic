//! ## `ColourMap` Module
//!
//! This module provides the `ColourMap` struct, which allows for interpolation between colours.

use num_traits::Float;

use crate::Colour;

mod fmt;

/// A map of colours at specific positions, with interpolation between them.
#[derive(Debug, Clone)]
pub struct ColourMap<C, T, const N: usize>
where
    C: Colour<T, N>,
    T: Float + Send + Sync,
{
    /// The colours in the map.
    colours: Vec<C>,
    /// The positions of the colours in the map, in range [0, 1].
    positions: Vec<T>,
}

impl<C, T, const N: usize> ColourMap<C, T, N>
where
    C: Clone + Colour<T, N>,
    T: Float + Send + Sync,
{
    /// Create a new colour map from a list of colours and positions.
    #[must_use]
    #[inline]
    pub fn new(colours: &[C], positions: &[T]) -> Self {
        debug_assert!(!colours.is_empty(), "Colour map must have at least one colour.");
        debug_assert_eq!(
            colours.len(),
            positions.len(),
            "Colour map must have the same number of colours and positions."
        );

        // Validate positions are in range [0, 1] using all()
        debug_assert!(
            positions
                .iter()
                .all(|position| *position >= T::zero() && *position <= T::one()),
            "Positions must be in range [0, 1]."
        );

        // Check positions are in ascending order using windows() and all()
        debug_assert!(
            positions.windows(2).all(|window| window[1] > window[0]),
            "Positions must be in ascending order."
        );

        Self {
            colours: colours.to_vec(),
            positions: positions.to_vec(),
        }
    }

    /// Create a new colour map with uniformly spaced positions.
    ///
    /// # Panics
    ///
    /// Panics if the colours slice is empty.
    #[must_use]
    #[inline]
    pub fn new_uniform(colours: &[C]) -> Self {
        debug_assert!(!colours.is_empty(), "Colour map must have at least one colour.");
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
    /// Panics if the position is not in the range [0, 1].
    #[inline]
    pub fn sample(&self, position: T) -> C {
        debug_assert!(
            (T::zero()..=T::one()).contains(&position),
            "Position must be in range [0, 1]."
        );

        // fast-edge
        if position <= self.positions[0] {
            return self.colours[0].clone();
        }
        if position >= *self.positions.last().unwrap() {
            return self.colours.last().unwrap().clone();
        }

        // find first i where positions[i] > position
        let hi = self
            .positions
            .binary_search_by(|p| p.partial_cmp(&position).unwrap())
            .unwrap_or_else(|i| i);
        let lo = hi - 1;

        let (start, end) = (self.positions[lo], self.positions[hi]);
        let t = (position - start) / (end - start);
        C::lerp(&self.colours[lo], &self.colours[hi], t)
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

    /// Get a reference to the positions in the map.
    #[must_use]
    #[inline]
    pub fn positions(&self) -> &[T] {
        &self.positions
    }
}
