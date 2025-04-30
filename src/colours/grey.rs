//! Monochrome colour representation.

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Add, Mul, Sub},
    str::FromStr,
};
use num_traits::Float;

use crate::{Channel, Colour, ColourParseError};

/// Monochrome.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct Grey<T: Float>(pub T);

impl<T: Float> Grey<T> {
    /// Create a new `Grey` instance.
    ///
    /// # Panics
    ///
    /// Panics if any of the components are not in the range [0, 1].
    #[inline]
    pub fn new(grey: T) -> Self {
        assert!(
            grey >= T::zero() && grey <= T::one(),
            "Grey component must be between 0 and 1"
        );
        Self(grey)
    }

    /// Get the grey component.
    #[inline]
    pub const fn g(&self) -> T {
        self.0
    }
}

impl<T: Float> Colour<T> for Grey<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    #[expect(clippy::min_ident_chars, reason = "The variable `t` is commonly used in lerp functions.")]
    #[inline]
    fn lerp(&self, other: &Self, t: T) -> Self {
        assert!(t >= T::zero() && t <= T::one(), "Lerp factor must be between 0 and 1");
        Self(self.0 + (other.0 - self.0) * t)
    }
}

impl<T: Float + Channel> FromStr for Grey<T> {
    type Err = ColourParseError;

    #[expect(
        clippy::min_ident_chars,
        reason = "The variable `s` is commonly used in string parsing functions."
    )]
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.trim().trim_start_matches('#');
        if hex.len() != 2 {
            return Err(ColourParseError::InvalidLength(hex.len()));
        }

        let grey = u8::from_str_radix(hex, 16)?;
        Ok(Self::new(T::from_u8(grey)))
    }
}

impl<T: Float + Channel> Display for Grey<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let g = self.g().to_u8().unwrap();
        write!(f, "#{:02X}", g)
    }
}
