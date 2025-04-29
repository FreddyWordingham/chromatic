use core::{
    ops::{Add, Mul, Sub},
    str::FromStr,
};

use crate::{Channel, Colour, ColourParseError};

/// Monochrome with alpha.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct GreyAlpha<T>(pub T, pub T);

impl<T> GreyAlpha<T> {
    /// Create a new `GreyAlpha` instance.
    #[inline]
    pub const fn new(grey: T, alpha: T) -> Self {
        Self(grey, alpha)
    }

    /// Get the grey component.
    #[inline]
    pub fn g(&self) -> T
    where
        T: Clone,
    {
        self.0.clone()
    }

    /// Get the alpha component.
    #[inline]
    pub fn a(&self) -> T
    where
        T: Clone,
    {
        self.1.clone()
    }
}

impl<T> Colour<T> for GreyAlpha<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    #[inline]
    fn lerp(&self, other: &Self, t: T) -> Self {
        Self(
            self.0 + (other.0 - self.0) * t,
            self.1 + (other.1 - self.1) * t,
        )
    }
}

impl<T: Channel> FromStr for GreyAlpha<T> {
    type Err = ColourParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.trim().strip_prefix('#').unwrap_or(s);
        if hex.len() != 4 {
            return Err(ColourParseError::InvalidLength(hex.len()));
        }
        let grey = u8::from_str_radix(&hex[0..2], 16).map_err(|_| ColourParseError::InvalidHex)?;
        let alpha = u8::from_str_radix(&hex[2..4], 16).map_err(|_| ColourParseError::InvalidHex)?;
        Ok(Self(T::from_u8(grey), T::from_u8(alpha)))
    }
}
