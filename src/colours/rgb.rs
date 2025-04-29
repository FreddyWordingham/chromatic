use core::{
    ops::{Add, Mul, Sub},
    str::FromStr,
};
use palette::{
    LinSrgb, Mix as _,
    num::{Arithmetics, Clamp, One, Real, Zero},
};

use crate::{Channel, Colour, ColourParseError};

/// RGB colour.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct Rgb<T>(pub LinSrgb<T>);

impl<T> Rgb<T> {
    /// Create a new `Rgb` instance.
    #[inline]
    pub const fn new(red: T, green: T, blue: T) -> Self {
        Self(LinSrgb::new(red, green, blue))
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
}

impl<T> Colour<T> for Rgb<T>
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
    #[expect(
        clippy::min_ident_chars,
        reason = "The variable `t` is commonly used in lerp functions."
    )]
    #[inline]
    fn lerp(&self, other: &Self, t: T) -> Self {
        Self(self.0.mix(other.0, t))
    }
}

impl<T: Channel> FromStr for Rgb<T> {
    type Err = ColourParseError;

    #[expect(
        clippy::min_ident_chars,
        reason = "The variable `s` is commonly used in string parsing functions."
    )]
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.trim_start_matches('#');
        if hex.len() != 6 {
            return Err(ColourParseError::InvalidLength(hex.len()));
        }

        let rgb = u32::from_str_radix(hex, 16)?;
        let red = u8::try_from((rgb >> 16i32) & 0xFF)?;
        let green = u8::try_from((rgb >> 8i32) & 0xFF)?;
        let blue = u8::try_from(rgb & 0xFF)?;

        Ok(Self::new(
            T::from_u8(red),
            T::from_u8(green),
            T::from_u8(blue),
        ))
    }
}
