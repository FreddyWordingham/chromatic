//! Print `Hsla` to the terminal.

use core::fmt::{Display, Formatter, Result as FmtResult};
use num_traits::Float;

use crate::Hsla;

/// Character used to print the colour in the terminal.
const BLOCK: char = '\u{2588}';

impl<T: Float> Display for Hsla<T> {
    #[expect(clippy::min_ident_chars, reason = "Variable `f` for `Formatter` is idiomatic.")]
    #[expect(clippy::unwrap_in_result, reason = "Unwrap will not fail here.")]
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let (red, green, blue, _) = self.rgba_components();
        let max = T::from(255_i32).unwrap();
        let rounded_red = (red * max).round().to_u8().unwrap();
        let rounded_green = (green * max).round().to_u8().unwrap();
        let rounded_blue = (blue * max).round().to_u8().unwrap();
        write!(f, "\x1b[38;2;{rounded_red};{rounded_green};{rounded_blue}m{BLOCK}\x1b[0m")
    }
}
