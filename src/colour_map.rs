use enterpolation::{Generator, Identity, Merge, Sorted, linear::Linear};
use num_traits::{Float, FromPrimitive};
use std::fmt::Debug;

use crate::Colour;

/// A collection of colours that can be sampled to produce a gradient.
#[derive(Debug, Clone)]
pub struct ColourMap<C, T>
where
    C: Colour<T> + Clone + Copy,
    T: Float + FromPrimitive,
{
    inner: ColourMapInner<C, T>,
}

/// Internal representation of a colour map.
#[derive(Debug, Clone)]
enum ColourMapInner<C, T> {
    /// Efficient gradient using `enterpolate` library.
    Gradient(Linear<Sorted<Vec<T>>, Vec<C>, Identity>),

    /// Raw colors and positions when gradient can't be used.
    Raw {
        /// The colours that make up the map.
        colours: Vec<C>,
        /// The positions of each colour in the range [0, 1].
        positions: Vec<T>,
    },
}

impl<C, T> ColourMap<C, T>
where
    C: Clone + Copy + Colour<T> + Merge<T>, // Added Copy trait bound here
    T: Float + FromPrimitive,
{
    /// Create a new colour map with specified colours and positions.
    ///
    /// # Panics
    ///
    /// Panics if the number of colours and positions differ.
    /// Panics if any position is outside the range [0, 1].
    /// Panics if positions are not monotonically increasing.
    #[inline]
    pub fn new(colours: &[C], positions: &[T]) -> Self {
        assert!(
            colours.len() == positions.len(),
            "Colours and positions must have the same length"
        );
        assert!(
            positions.iter().all(|&p| p >= T::zero() && p <= T::one()),
            "Positions must be in range [0, 1]"
        );
        assert!(!positions.is_empty(), "At least one position is required");

        for i in 1..positions.len() {
            assert!(positions[i] > positions[i - 1], "Positions must be strictly increasing");
        }

        let colours_vec = colours.to_vec();
        let positions_vec = positions.to_vec();

        match Self::create_gradient(&colours_vec, &positions_vec) {
            Some(gradient) => Self {
                inner: ColourMapInner::Gradient(gradient),
            },
            None => Self {
                inner: ColourMapInner::Raw {
                    colours: colours_vec,
                    positions: positions_vec,
                },
            },
        }
    }

    /// Build a colour map from a list of uniformly distributed colours.
    ///
    /// # Panics
    ///
    /// Panics if no colours are provided.
    #[inline]
    pub fn new_uniform(colours: &[C]) -> Self {
        assert!(!colours.is_empty(), "Cannot create a colour map with no colours");

        let count = colours.len();
        let mut positions = Vec::with_capacity(count);

        if count == 1 {
            positions.push(T::zero());
        } else {
            for i in 0..count {
                let t = if i == 0 {
                    T::zero()
                } else if i == count - 1 {
                    T::one()
                } else {
                    T::from_usize(i).unwrap() / T::from_usize(count - 1).unwrap()
                };
                positions.push(t);
            }
        }

        Self::new(colours, &positions)
    }

    /// Create a new empty colour map.
    #[inline]
    pub fn empty() -> Self {
        Self {
            inner: ColourMapInner::Raw {
                colours: Vec::new(),
                positions: Vec::new(),
            },
        }
    }

    /// Helper method to create a gradient using enterpolate
    #[inline]
    fn create_gradient(colours: &[C], positions: &[T]) -> Option<Linear<Sorted<Vec<T>>, Vec<C>, Identity>>
    where
        C: Copy, // Added Copy trait bound here
    {
        // We need at least two colours to create a gradient
        if colours.len() < 2 {
            return None;
        }

        // Build the gradient using enterpolate
        Linear::builder()
            .elements(colours.to_vec())
            .knots(positions.to_vec())
            .build()
            .ok()
    }

    /// Add a colour to the map at the specified position.
    ///
    /// # Panics
    ///
    /// Panics if the position is not in the range [0, 1].
    /// Panics if the position is already occupied in the map.
    #[inline]
    pub fn insert_colour(&mut self, colour: C, position: T) -> &mut Self
    where
        C: Debug,
        T: Debug,
    {
        assert!(
            position >= T::zero() && position <= T::one(),
            "Position must be in range [0, 1]"
        );

        // Extract current colours and positions or get them from the Raw variant
        let (mut colours, mut positions) = match &self.inner {
            ColourMapInner::Gradient(_) => {
                // Since we can't access the internal fields directly and to avoid a circular
                // dependency with sample(), we'll convert to a Raw representation using
                // a reasonable number of sample points

                // We'll sample at a reasonably high resolution
                const SAMPLE_COUNT: usize = 20;
                let mut extracted_colours = Vec::with_capacity(SAMPLE_COUNT + 1);
                let mut extracted_positions = Vec::with_capacity(SAMPLE_COUNT + 1);

                for i in 0..=SAMPLE_COUNT {
                    let t = T::from_usize(i).unwrap() / T::from_usize(SAMPLE_COUNT).unwrap();
                    // Instead of calling self.sample which might cause recursion,
                    // we manually match on the inner again
                    let color = match &self.inner {
                        ColourMapInner::Gradient(g) => {
                            // Use fully qualified syntax to call the Generator trait method
                            <Linear<Sorted<Vec<T>>, Vec<C>, Identity> as Generator<T>>::r#gen(g, t)
                        }
                        ColourMapInner::Raw { colours, positions } => {
                            // This branch shouldn't be reached, but just in case
                            // we include the raw interpolation logic
                            if colours.is_empty() {
                                panic!("Cannot sample an empty colour map");
                            }
                            if colours.len() == 1 {
                                colours[0]
                            } else if t <= positions[0] {
                                colours[0]
                            } else if t >= *positions.last().unwrap() {
                                *colours.last().unwrap()
                            } else {
                                let idx = positions.iter().position(|&p| p > t).unwrap_or(positions.len() - 1);
                                let low_idx = idx - 1;
                                let high_idx = idx;
                                let low_pos = positions[low_idx];
                                let high_pos = positions[high_idx];
                                let low_colour = &colours[low_idx];
                                let high_colour = &colours[high_idx];
                                let segment_t = (t - low_pos) / (high_pos - low_pos);
                                low_colour.lerp(high_colour, segment_t)
                            }
                        }
                    };
                    extracted_positions.push(t);
                    extracted_colours.push(color);
                }

                (extracted_colours, extracted_positions)
            }
            ColourMapInner::Raw { colours, positions } => (colours.clone(), positions.clone()),
        };

        // Check for duplicate position
        assert!(
            !positions.contains(&position),
            "Cannot insert a colour at an existing position"
        );

        // Find the insertion index to maintain sorted order
        let index = positions.iter().position(|&p| p > position).unwrap_or(positions.len());

        // Insert the new colour and position
        positions.insert(index, position);
        colours.insert(index, colour);

        // Create a new inner representation
        match Self::create_gradient(&colours, &positions) {
            Some(gradient) => {
                self.inner = ColourMapInner::Gradient(gradient);
            }
            None => {
                self.inner = ColourMapInner::Raw { colours, positions };
            }
        }

        self
    }

    /// Sample the colour map at a given position.
    ///
    /// Returns the interpolated colour at the specified position.
    /// If the position is outside the range [0, 1], it will be clamped.
    ///
    /// # Panics
    ///
    /// Panics if the colour map is empty.
    #[inline]
    pub fn sample(&self, mut t: T) -> C
    where
        C: Debug,
        T: Debug,
    {
        // Clamp t to [0, 1]
        if t < T::zero() {
            t = T::zero();
        } else if t > T::one() {
            t = T::one();
        }

        match &self.inner {
            ColourMapInner::Gradient(gradient) => {
                // Use fully qualified syntax to call the Generator trait method
                <Linear<Sorted<Vec<T>>, Vec<C>, Identity> as Generator<T>>::r#gen(gradient, t)
            }
            ColourMapInner::Raw { colours, positions } => {
                assert!(!colours.is_empty(), "Cannot sample an empty colour map");

                // If we only have one colour, return it
                if colours.len() == 1 {
                    return colours[0];
                }

                // Handle edge cases
                if t <= positions[0] {
                    return colours[0];
                }

                if t >= *positions.last().unwrap() {
                    return colours.last().unwrap().clone();
                }

                // Perform manual interpolation
                let idx = positions.iter().position(|&p| p > t).unwrap_or(positions.len() - 1);

                let low_idx = idx - 1;
                let high_idx = idx;

                let low_pos = positions[low_idx];
                let high_pos = positions[high_idx];

                let low_colour = &colours[low_idx];
                let high_colour = &colours[high_idx];

                // Normalize t for the segment
                let segment_t = (t - low_pos) / (high_pos - low_pos);

                low_colour.lerp(high_colour, segment_t)
            }
        }
    }
}
