//! Monochrome colour representation with transparency.

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    num::ParseIntError,
    ops::AddAssign,
    str::FromStr,
};
use num_traits::{Float, ToPrimitive};

use crate::Colour;

/// Error parsing `GreyAlpha` from string.
#[derive(Debug)]
#[non_exhaustive]
pub enum ParseGreyAlphaError<E> {
    /// Error parsing float.
    ParseFloat(E),
    /// Error parsing hex string.
    ParseHex(ParseIntError),
    /// Value out of range.
    OutOfRange,
    /// Invalid format.
    InvalidFormat,
}

/// Monochrome colour with transparency.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GreyAlpha<T: Float> {
    /// Grey component.
    grey: T,
    /// Alpha component.
    alpha: T,
}

impl<T: Display + AddAssign + Float> GreyAlpha<T> {
    /// Create a new `GreyAlpha` instance.
    ///
    /// # Panics
    ///
    /// Panics if any component is not in [0, 1].
    #[inline]
    pub fn new(mut grey: T, mut alpha: T) -> Self {
        let tolerance = Self::tolerance();
        if grey < T::zero() - tolerance || grey > T::one() + tolerance {
            assert!(
                !(grey < T::zero() - tolerance || grey > T::one() + tolerance),
                "Grey component {grey} out of [0, 1]\u{b1}{tolerance}."
            );
        }
        if alpha < T::zero() - tolerance || alpha > T::one() + tolerance {
            assert!(
                !(alpha < T::zero() - tolerance || alpha > T::one() + tolerance),
                "Alpha component {alpha} out of [0, 1]\u{b1}{tolerance}."
            );
        }
        grey = grey.clamp(T::zero(), T::one());
        alpha = alpha.clamp(T::zero(), T::one());
        Self { grey, alpha }
    }

    /// Get the grey component.
    #[inline]
    pub const fn grey(&self) -> T {
        self.grey
    }

    /// Get the alpha component.
    #[inline]
    pub const fn alpha(&self) -> T {
        self.alpha
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

    /// Set the alpha component.
    ///
    /// # Panics
    ///
    /// Panics if the value is not in [0, 1].
    #[inline]
    pub fn set_alpha(&mut self, alpha: T) {
        assert!(
            alpha >= T::zero() && alpha <= T::one(),
            "Alpha component must be between 0 and 1."
        );
        self.alpha = alpha;
    }
}

impl<T: Display + AddAssign + Float> Colour<T, 2> for GreyAlpha<T> {
    #[inline]
    fn from_components(components: [T; 2]) -> Self {
        Self::new(components[0], components[1])
    }

    #[inline]
    fn components(&self) -> [T; 2] {
        [self.grey, self.alpha]
    }

    #[inline]
    fn set_components(&mut self, components: [T; 2]) {
        self.set_grey(components[0]);
        self.set_alpha(components[1]);
    }

    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn from_bytes(bytes: [u8; 2]) -> Self {
        let max = T::from(255_u8).unwrap();
        let grey = T::from(bytes[0]).unwrap() / max;
        let alpha = T::from(bytes[1]).unwrap() / max;
        Self::new(grey, alpha)
    }

    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn to_bytes(self) -> [u8; 2] {
        let max = T::from(255_u8).unwrap();
        let grey = (self.grey * max).round().to_u8().unwrap();
        let alpha = (self.alpha * max).round().to_u8().unwrap();
        [grey, alpha]
    }

    /// Linear interpolate between two `GreyAlpha`s.
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
        Self::new(
            lhs.grey() * (T::one() - t) + rhs.grey() * t,
            lhs.alpha() * (T::one() - t) + rhs.alpha() * t,
        )
    }
}

impl<T: Float + AddAssign + Display> PartialEq for GreyAlpha<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (self.grey - other.grey).abs() <= Self::tolerance() && (self.alpha - other.alpha).abs() <= Self::tolerance()
    }
}

impl<T> FromStr for GreyAlpha<T>
where
    T: Display + AddAssign + Float + FromStr + ToPrimitive,
{
    type Err = ParseGreyAlphaError<<T as FromStr>::Err>;

    #[expect(clippy::min_ident_chars, reason = "The variable `s` for a string is idiomatic.")]
    #[expect(clippy::unwrap_in_result, reason = "Unwrap will not fail here.")]
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(hex) = s.trim().strip_prefix('#') {
            match hex.len() {
                // Short form: #GA
                2 => {
                    let mut chars = hex.chars();
                    let grey_digit = chars.next().unwrap();
                    let alpha_digit = chars.next().unwrap();

                    let grey_value = u8::from_str_radix(&grey_digit.to_string(), 16).map_err(ParseGreyAlphaError::ParseHex)?;
                    let alpha_value =
                        u8::from_str_radix(&alpha_digit.to_string(), 16).map_err(ParseGreyAlphaError::ParseHex)?;

                    // Expand short form (e.g., #FA becomes #FFAA)
                    let grey = T::from(grey_value * 17).ok_or(ParseGreyAlphaError::OutOfRange)? / T::from(255).unwrap();
                    let alpha = T::from(alpha_value * 17).ok_or(ParseGreyAlphaError::OutOfRange)? / T::from(255).unwrap();

                    Ok(Self::new(grey, alpha))
                }
                // Long form: #GGAA
                4 => {
                    let mut chars = hex.chars();
                    let g1 = chars.next().unwrap().to_string();
                    let g2 = chars.next().unwrap().to_string();
                    let a1 = chars.next().unwrap().to_string();
                    let a2 = chars.next().unwrap().to_string();

                    let grey_value = u8::from_str_radix(&format!("{g1}{g2}"), 16).map_err(ParseGreyAlphaError::ParseHex)?;
                    let alpha_value = u8::from_str_radix(&format!("{a1}{a2}"), 16).map_err(ParseGreyAlphaError::ParseHex)?;

                    let grey = T::from(grey_value).ok_or(ParseGreyAlphaError::OutOfRange)? / T::from(255).unwrap();
                    let alpha = T::from(alpha_value).ok_or(ParseGreyAlphaError::OutOfRange)? / T::from(255).unwrap();

                    Ok(Self::new(grey, alpha))
                }
                _ => Err(ParseGreyAlphaError::InvalidFormat),
            }
        } else {
            // Look for comma-separated float values
            let parts: Vec<&str> = s.split(',').collect();
            if parts.len() != 2 {
                return Err(ParseGreyAlphaError::InvalidFormat);
            }

            let grey = parts[0].trim().parse::<T>().map_err(ParseGreyAlphaError::ParseFloat)?;
            let alpha = parts[1].trim().parse::<T>().map_err(ParseGreyAlphaError::ParseFloat)?;

            Ok(Self::new(grey, alpha))
        }
    }
}

impl<T> Display for GreyAlpha<T>
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
        let alpha = (self.alpha * max).round().to_u8().unwrap();
        write!(f, "#{grey:X}{alpha:X}")
    }
}
