use core::{
    ops::{Add, Mul, Sub},
    str::FromStr,
};
use palette::{
    LinSrgba, Mix as _,
    num::{Arithmetics, Clamp, One, Real, Zero},
};

use crate::{Channel, Colour, ColourParseError};

/// Colour with alpha channel.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct Rgba<T>(pub LinSrgba<T>);

impl<T> Rgba<T> {
    /// Create a new `Rgba` instance.
    #[inline]
    pub const fn new(red: T, green: T, blue: T, alpha: T) -> Self {
        Self(LinSrgba::new(red, green, blue, alpha))
    }

    /// Get the red component.
    #[inline]
    pub fn r(&self) -> T
    where
        T: Clone,
    {
        self.0.red.clone()
    }

    /// Get the green component.
    #[inline]
    pub fn g(&self) -> T
    where
        T: Clone,
    {
        self.0.green.clone()
    }

    /// Get the blue component.
    #[inline]
    pub fn b(&self) -> T
    where
        T: Clone,
    {
        self.0.blue.clone()
    }

    /// Get the alpha component.
    #[inline]
    pub fn a(&self) -> T
    where
        T: Clone,
    {
        self.0.alpha.clone()
    }
}

impl<T> Colour<T> for Rgba<T>
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Clone
        + Real
        + Zero
        + One
        + Arithmetics
        + Clamp,
{
    #[inline]
    fn lerp(&self, other: &Self, t: T) -> Self {
        Self(self.0.mix(other.0, t))
    }
}

impl<T: Channel> FromStr for Rgba<T> {
    type Err = ColourParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.trim().strip_prefix('#').unwrap_or(s);
        if hex.len() != 8 {
            return Err(ColourParseError::InvalidLength(hex.len()));
        }

        let red = u8::from_str_radix(&hex[0..2], 16).map_err(|_| ColourParseError::InvalidHex)?;
        let green = u8::from_str_radix(&hex[2..4], 16).map_err(|_| ColourParseError::InvalidHex)?;
        let blue = u8::from_str_radix(&hex[4..6], 16).map_err(|_| ColourParseError::InvalidHex)?;
        let alpha = u8::from_str_radix(&hex[6..8], 16).map_err(|_| ColourParseError::InvalidHex)?;

        Ok(Self::new(
            T::from_u8(red),
            T::from_u8(green),
            T::from_u8(blue),
            T::from_u8(alpha),
        ))
    }
}
