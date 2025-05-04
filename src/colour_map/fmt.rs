//! Print a `ColourMap` to the terminal.

use core::fmt::{Display, Formatter, Result as FmtResult};
use num_traits::Float;
use terminal_size::{Width, terminal_size};

use crate::{Colour, ColourMap};

impl<C, T, const N: usize> Display for ColourMap<C, T, N>
where
    C: Display + Clone + Colour<T, N>,
    T: Float + Send + Sync,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let width = if let Some((Width(width), _)) = terminal_size() {
            width
        } else {
            60
        };
        for i in 0..width {
            let t = T::from(i).unwrap() / T::from((width - 1).max(1)).unwrap();
            let colour = self.sample(t);
            write!(f, "{colour}")?;
        }
        Ok(())
    }
}
