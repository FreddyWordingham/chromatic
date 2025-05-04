//! Print `Lab` to the terminal.

use core::fmt::{Display, Formatter, Result as FmtResult};
use num_traits::Float;

use crate::{Convert, Lab, config::PRINT_BLOCK};

impl<T: Float + Send + Sync> Display for Lab<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // Convert to RGB for terminal display
        let rgb = self.to_rgb();
        let max = T::from(255_i32).unwrap();
        let red = (rgb.red() * max).round().to_u8().unwrap();
        let green = (rgb.green() * max).round().to_u8().unwrap();
        let blue = (rgb.blue() * max).round().to_u8().unwrap();
        write!(f, "\x1b[38;2;{red};{green};{blue}m{PRINT_BLOCK}\x1b[0m")
    }
}
