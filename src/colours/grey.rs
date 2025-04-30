//! Monochrome colour representation.

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Add, Mul, Sub},
    str::FromStr,
};
use num_traits::{Float, FromPrimitive};

use crate::{Colour, ColourParseError};

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

impl<T> FromStr for Grey<T>
where
    T: Float + FromPrimitive,
{
    type Err = ColourParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.trim().trim_start_matches('#');
        // support "#G" or "#GG"
        let byte = match hex.len() {
            1 => {
                let v = u8::from_str_radix(hex, 16)?;
                v.saturating_mul(17)
            }
            2 => u8::from_str_radix(hex, 16)?,
            len => return Err(ColourParseError::InvalidLength(len)),
        };
        let scale = T::from_u8(255).unwrap();
        let gt = T::from_u8(byte).unwrap() / scale;
        Ok(Grey::new(gt))
    }
}

impl<T: Float> Display for Grey<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let scale = T::from(255).unwrap();
        let g = (scale * self.g()).to_u8().unwrap();
        write!(f, "#{:02X}", g)
    }
}
