//! Print a `ColourMap` to the terminal.

use num_traits::Float;
use std::fmt::{Display, Formatter, Result as FmtResult};
use terminal_size::{Width, terminal_size};

use crate::{Colour, ColourMap};

impl<C, T, const N: usize> Display for ColourMap<C, T, N>
where
    C: Display + Clone + Colour<T, N>,
    T: Float + Send + Sync,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let width = terminal_size().map_or(60, |(Width(w), _)| w);
        let denom = width.saturating_sub(1).max(1);
        for i in 0..width {
            let t = T::from(i).unwrap() / T::from(denom).unwrap();
            let colour = self.sample(t);
            write!(f, "{colour}")?;
        }
        Ok(())
    }
}
