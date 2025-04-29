use core::{
    ops::{Add, Mul, Sub},
    str::FromStr,
};

use crate::{Channel, Colour, ColourParseError};

/// Monochrome.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct Grey<T>(pub T);

impl<T> Grey<T> {
    /// Create a new `Grey` instance.
    #[inline]
    pub const fn new(grey: T) -> Self {
        Self(grey)
    }

    /// Get the grey component.
    #[inline]
    pub fn g(&self) -> T
    where
        T: Clone,
    {
        self.0.clone()
    }
}

impl<T> Colour<T> for Grey<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    #[inline]
    fn lerp(&self, other: &Self, t: T) -> Self {
        Self(self.0 + (other.0 - self.0) * t)
    }
}

impl<T: Channel> FromStr for Grey<T> {
    type Err = ColourParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.trim().strip_prefix('#').unwrap_or(s);
        if hex.len() != 2 {
            return Err(ColourParseError::InvalidLength(hex.len()));
        }
        let grey = u8::from_str_radix(hex, 16).map_err(|_| ColourParseError::InvalidHex)?;
        Ok(Self::new(T::from_u8(grey)))
    }
}
