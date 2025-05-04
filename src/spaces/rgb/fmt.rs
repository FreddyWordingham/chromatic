//! Print `Rgb` to the terminal.

use num_traits::Float;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{Rgb, config::PRINT_BLOCK};

impl<T: Float + Send + Sync> Display for Rgb<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let max = T::from(255_i32).unwrap();
        let red = (self.red * max).round().to_u8().unwrap();
        let green = (self.green * max).round().to_u8().unwrap();
        let blue = (self.blue * max).round().to_u8().unwrap();
        write!(f, "\x1b[38;2;{red};{green};{blue}m{PRINT_BLOCK}\x1b[0m")
    }
}
