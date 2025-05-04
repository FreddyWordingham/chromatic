//! Print `Grey` to the terminal.

use core::fmt::{Display, Formatter, Result as FmtResult};
use num_traits::Float;

use crate::{Grey, config::PRINT_BLOCK};

impl<T: Float + Send + Sync> Display for Grey<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let max = T::from(255_i32).unwrap();
        let value = (self.grey * max).round().to_u8().unwrap();
        write!(f, "\x1b[38;2;{value};{value};{value}m{PRINT_BLOCK}\x1b[0m")
    }
}
