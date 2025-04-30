//! Monochrome with alpha channel colour representation.

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Add, Mul, Sub},
    str::FromStr,
};
use num_traits::Float;

use crate::{Channel, Colour, ColourParseError};

/// Monochrome with alpha.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct GreyAlpha<T: Float>(pub T, pub T);

impl<T: Float> GreyAlpha<T> {
    /// Create a new `GreyAlpha` instance.
    ///
    /// # Panics
    ///
    /// Panics if any of the components are not in the range [0, 1].
    #[inline]
    pub fn new(grey: T, alpha: T) -> Self {
        assert!(
            grey >= T::zero() && grey <= T::one(),
            "Grey component must be between 0 and 1"
        );
        assert!(
            alpha >= T::zero() && alpha <= T::one(),
            "Alpha component must be between 0 and 1"
        );
        Self(grey, alpha)
    }

    /// Get the grey component.
    #[inline]
    pub const fn g(&self) -> T {
        self.0
    }

    /// Get the alpha component.
    #[inline]
    pub const fn a(&self) -> T {
        self.1
    }
}

impl<T: Float> Colour<T> for GreyAlpha<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    #[expect(clippy::min_ident_chars, reason = "The variable `t` is commonly used in lerp functions.")]
    #[inline]
    fn lerp(&self, other: &Self, t: T) -> Self {
        assert!(t >= T::zero() && t <= T::one(), "Lerp factor must be between 0 and 1");
        Self(self.0 + (other.0 - self.0) * t, self.1 + (other.1 - self.1) * t)
    }
}

impl<T: Float + Channel> FromStr for GreyAlpha<T> {
    type Err = ColourParseError;

    #[expect(
        clippy::min_ident_chars,
        reason = "The variable `s` is commonly used in string parsing functions."
    )]
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.trim().trim_start_matches('#');
        if hex.len() != 4 {
            return Err(ColourParseError::InvalidLength(hex.len()));
        }

        let ga = u16::from_str_radix(hex, 16)?;
        let grey = u8::try_from(ga >> 8i32)?;
        let alpha = u8::try_from(ga & 0xFF)?;

        Ok(Self(T::from_u8(grey), T::from_u8(alpha)))
    }
}

impl<T: Float + Channel> Display for GreyAlpha<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let g = self.g().to_u8().unwrap();
        let a = self.a().to_u8().unwrap();
        write!(f, "#{:02X}{:02X}", g, a)
    }
}
