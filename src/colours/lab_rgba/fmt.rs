//! Print `LabRgba` to the terminal.

use core::fmt::{Display, Formatter, Result as FmtResult};
use num_traits::Float;

use crate::LabRgba;

/// Character used to print the colour in the terminal.
const BLOCK: char = '\u{2588}';

impl<T: Float> Display for LabRgba<T> {
    #[expect(clippy::min_ident_chars, reason = "Variable `f` for `Formatter` is idiomatic.")]
    #[expect(clippy::unwrap_in_result, reason = "Unwrap will not fail here.")]
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let max = T::from(255_i32).unwrap();
        let [red, green, blue] = self.rgb_components();
        let red_rounded = (red * max).round().to_u8().unwrap();
        let green_rounded = (green * max).round().to_u8().unwrap();
        let blue_rounded = (blue * max).round().to_u8().unwrap();
        write!(f, "\x1b[38;2;{red_rounded};{green_rounded};{blue_rounded}m{BLOCK}\x1b[0m")
    }
}
