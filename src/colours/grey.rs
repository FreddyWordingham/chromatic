//! Monochrome colour representation.

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::AddAssign,
    str::FromStr,
};
use num_traits::{Float, ToPrimitive};

use crate::{Colour, GreyAlpha, LabRgb, LabRgba, ParseColourError, Rgb, Rgba};

/// Monochrome colour.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Grey<T: Float> {
    /// Grey component.
    grey: T,
}

impl<T: Display + AddAssign + Float> Grey<T> {
    /// Create a new `Grey` instance.
    ///
    /// # Panics
    ///
    /// Panics if the component is not in [0, 1].
    #[inline]
    pub fn new(grey: T) -> Self {
        assert!(!(grey < T::zero() || grey > T::one()), "Grey component {grey} out of [0, 1].");
        Self { grey }
    }

    /// Get the grey component.
    #[inline]
    pub const fn grey(&self) -> T {
        self.grey
    }

    /// Set the grey component.
    ///
    /// # Panics
    ///
    /// Panics if the value is not in [0, 1].
    #[inline]
    pub fn set_grey(&mut self, grey: T) {
        assert!(
            grey >= T::zero() && grey <= T::one(),
            "Grey component must be between 0 and 1."
        );
        self.grey = grey;
    }

    /// Convert to `GreyAlpha`.
    #[inline]
    pub fn to_grey_alpha(&self, alpha: T) -> GreyAlpha<T> {
        GreyAlpha::new(self.grey, alpha)
    }

    /// Convert to `Rgb`.
    #[inline]
    pub fn to_rgb(&self) -> Rgb<T> {
        Rgb::new(self.grey, self.grey, self.grey)
    }

    /// Convert to `Rgba`.
    #[inline]
    pub fn to_rgba(&self, alpha: T) -> Rgba<T> {
        Rgba::new(self.grey, self.grey, self.grey, alpha)
    }

    /// Convert to `LabRgb`.
    #[inline]
    pub fn to_lab_rgb(&self) -> LabRgb<T> {
        LabRgb::new(self.grey, self.grey, self.grey)
    }

    /// Convert to `LabRgba`.
    #[inline]
    pub fn to_lab_rgba(&self, alpha: T) -> LabRgba<T> {
        LabRgba::new(self.grey, self.grey, self.grey, alpha)
    }
}

impl<T: Display + AddAssign + Float> Colour<T, 1> for Grey<T> {
    #[inline]
    fn from_components(components: [T; 1]) -> Self {
        Self::new(components[0])
    }

    #[inline]
    fn components(&self) -> [T; 1] {
        [self.grey]
    }

    #[inline]
    fn set_components(&mut self, components: [T; 1]) {
        self.set_grey(components[0]);
    }

    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn from_bytes(bytes: [u8; 1]) -> Self {
        let max = T::from(255_u8).unwrap();
        let value = T::from(bytes[0]).unwrap() / max;
        Self::new(value)
    }

    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn to_bytes(self) -> [u8; 1] {
        let max = T::from(255_u8).unwrap();
        let value = (self.grey * max).round().to_u8().unwrap();
        [value]
    }

    /// Linear interpolate between two greys.
    ///
    /// # Panics
    ///
    /// Panics if the interpolation factor is not in [0, 1].
    #[expect(
        clippy::min_ident_chars,
        reason = "The variable `t` for an interpolation factor is idiomatic."
    )]
    #[inline]
    fn lerp(lhs: &Self, rhs: &Self, t: T) -> Self {
        assert!(t >= T::zero() && t <= T::one(), "Interpolation factor {t} out of [0, 1].");
        Self::new(lhs.grey() * (T::one() - t) + rhs.grey() * t)
    }
}

impl<T: Display + AddAssign + Float> PartialEq for Grey<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (self.grey - other.grey).abs() <= Self::tolerance()
    }
}

impl<T> FromStr for Grey<T>
where
    T: Display + AddAssign + Float + FromStr + ToPrimitive,
{
    type Err = ParseColourError<<T as FromStr>::Err>;

    #[expect(clippy::min_ident_chars, reason = "The variable `s` for a string is idiomatic.")]
    #[expect(clippy::unwrap_in_result, reason = "Unwrap will not fail here.")]
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(hex) = s.trim().strip_prefix('#') {
            match hex.len() {
                // Short form: #G
                1 => {
                    let value = u8::from_str_radix(hex, 16).map_err(ParseColourError::ParseHex)?;
                    // Expand short form (e.g., #F becomes #FF)
                    let grey = T::from(value * 17).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                    Ok(Self::new(grey))
                }
                // Long form: #GG
                2 => {
                    let value = u8::from_str_radix(hex, 16).map_err(ParseColourError::ParseHex)?;
                    let grey = T::from(value).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                    Ok(Self::new(grey))
                }
                _ => Err(ParseColourError::InvalidFormat),
            }
        } else {
            // Parse as comma-separated float values
            let parts: Vec<&str> = s.split(',').collect();
            if parts.len() != 1 {
                return Err(ParseColourError::InvalidFormat);
            }

            let grey = parts[0].trim().parse::<T>().map_err(ParseColourError::ParseFloat)?;
            Ok(Self::new(grey))
        }
    }
}

impl<T> Display for Grey<T>
where
    T: Float + ToPrimitive,
{
    #[expect(clippy::min_ident_chars, reason = "The variable `f` for a formatter is idiomatic.")]
    #[expect(clippy::unwrap_in_result, reason = "Unwrap will not fail here.")]
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let max = T::from(15_i32).unwrap();
        let grey = (self.grey * max).round().to_u8().unwrap();
        write!(f, "#{grey:X}")
    }
}
