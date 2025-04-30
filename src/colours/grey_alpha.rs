//! Monochrome with alpha channel colour representation.

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Add, Mul, Sub},
    str::FromStr,
};
use num_traits::{Float, FromPrimitive};

use crate::{Colour, ColourParseError};

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

impl<T> FromStr for GreyAlpha<T>
where
    T: Float + FromPrimitive,
{
    type Err = ColourParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.trim().trim_start_matches('#');
        if hex.len() != 4 {
            return Err(ColourParseError::InvalidLength(hex.len()));
        }

        // parse two hex bytes
        let g = u8::from_str_radix(&hex[0..2], 16)?;
        let a = u8::from_str_radix(&hex[2..4], 16)?;

        // cast into T
        let scale = T::from_u8(255).ok_or(ColourParseError::ConversionFailed)?;
        let gt = T::from_u8(g).ok_or(ColourParseError::ConversionFailed)? / scale;
        let at = T::from_u8(a).ok_or(ColourParseError::ConversionFailed)? / scale;

        Ok(GreyAlpha(gt, at))
    }
}

impl<T: Float> Display for GreyAlpha<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let scale = T::from(255).unwrap();
        let g = (scale * self.g()).to_u8().unwrap();
        let a = (scale * self.a()).to_u8().unwrap();
        write!(f, "#{:02X}{:02X}", g, a)
    }
}
