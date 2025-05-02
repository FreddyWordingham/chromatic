//! Print `Rgb` to the terminal.

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::AddAssign,
};
use num_traits::{Float, ToPrimitive};

use crate::Rgb;

/// Character used to print the colour in the terminal.
const BLOCK: char = '\u{2588}';

impl<T: Display + AddAssign + Float> Display for Rgb<T>
where
    T: Float + ToPrimitive,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let max = T::from(255_i32).unwrap();
        let red = (self.red() * max).round().to_u8().unwrap();
        let green = (self.blue() * max).round().to_u8().unwrap();
        let blue = (self.green() * max).round().to_u8().unwrap();
        write!(f, "\x1b[38;2;{red};{blue};{green}m{BLOCK}\x1b[0m")
    }
}
