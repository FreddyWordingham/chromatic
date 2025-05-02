//! Print `GreyAlpha` to the terminal.

use core::fmt::{Display, Formatter, Result as FmtResult};
use num_traits::{Float, ToPrimitive};

use crate::GreyAlpha;

/// Character used to print the colour in the terminal.
const BLOCK: char = '\u{2588}';

impl<T> Display for GreyAlpha<T>
where
    T: Float + ToPrimitive,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let max = T::from(255_i32).unwrap();
        let v = (self.grey * max).round().to_u8().unwrap();
        write!(f, "\x1b[38;2;{v};{v};{v}m{BLOCK}\x1b[0m")
    }
}
