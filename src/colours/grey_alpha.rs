//! Monochrome colour representation with transparency.

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    num::ParseIntError,
    str::FromStr,
};
use num_traits::{Float, ToPrimitive};

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
pub struct GreyAlpha<T: Float>(T, T);

impl<T: Float + Display> GreyAlpha<T> {
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
        grey = grey.max(T::zero()).min(T::one());
        alpha = alpha.max(T::zero()).min(T::one());
        Self(grey, alpha)
    }

    /// Linear interpolate between two greyalphas.
    ///
    /// # Panics
    ///
    /// Panics if the interpolation factor is not in [0, 1].
    #[expect(
        clippy::min_ident_chars,
        reason = "The variable `t` for an interpolation factor is idiomatic."
    )]
    #[inline]
    pub fn lerp(lhs: Self, rhs: Self, t: T) -> Self {
        assert!(t >= T::zero() && t <= T::one(), "Interpolation factor {t} out of [0, 1].");
        Self::new(
            lhs.grey() * (T::one() - t) + rhs.grey() * t,
            lhs.alpha() * (T::one() - t) + rhs.alpha() * t,
        )
    }

    /// Mix N by folding lerp (assumes weights sum to 1).
    ///
    /// # Panics
    ///
    /// Panics if the list of colours is empty.
    /// Panics if the lengths of colours and weights do not match.
    /// Panics if any weight is negative.
    #[expect(
        clippy::min_ident_chars,
        reason = "The variable `t` for an interpolation factor is idiomatic."
    )]
    #[inline]
    pub fn mix_fold(colours: &[Self], weights: &[T]) -> Self {
        assert!(!colours.is_empty(), "Cannot mix an empty list of colours.");
        assert_eq!(colours.len(), weights.len(), "Colours and weights must have the same length.");
        assert!(weights.iter().all(|&w| w >= T::zero()), "Weights must be non-negative.");

        let mut acc = colours[0];
        let mut acc_w = weights[0];
        for (&col, &w) in colours.iter().skip(1).zip(weights.iter().skip(1)) {
            let t = w / (acc_w + w);
            acc = Self::lerp(acc, col, t);
            acc_w = acc_w + w;
        }
        acc
    }

    /// Create a new `GreyAlpha` value from a byte array.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[must_use]
    #[inline]
    pub fn from_bytes(bytes: [u8; 2]) -> Self {
        let max = T::from(255_u8).unwrap();
        let grey = T::from(bytes[0]).unwrap() / max;
        let alpha = T::from(bytes[1]).unwrap() / max;
        Self::new(grey, alpha)
    }

    /// Convert to a byte array.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[must_use]
    #[inline]
    pub fn to_bytes(self) -> [u8; 2] {
        let max = T::from(255_u8).unwrap();
        let grey = (self.0 * max).round().to_u8().unwrap();
        let alpha = (self.1 * max).round().to_u8().unwrap();
        [grey, alpha]
    }

    /// Get the grey component.
    #[inline]
    pub const fn grey(&self) -> T {
        self.0
    }

    /// Get the alpha component.
    #[inline]
    pub const fn alpha(&self) -> T {
        self.1
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
        self.0 = grey;
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
        self.1 = alpha;
    }

    /// Get the tolerance for comparing grey values.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[must_use]
    #[inline]
    pub fn tolerance() -> T {
        T::one() / T::from(256_i32).unwrap()
    }
}

impl<T: Float + Display> PartialEq for GreyAlpha<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() <= Self::tolerance() && (self.1 - other.1).abs() <= Self::tolerance()
    }
}

impl<T> FromStr for GreyAlpha<T>
where
    T: Display + Float + FromStr + ToPrimitive,
{
    type Err = ParseGreyAlphaError<<T as FromStr>::Err>;

    #[expect(clippy::min_ident_chars, reason = "The variable `s` for a string is idiomatic.")]
    #[expect(clippy::unwrap_in_result, reason = "Unwrap will not fail here.")]
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(hex) = s.trim().strip_prefix('#') {
            // Check if we have a valid 2-character hex string
            if hex.len() != 2 {
                return Err(ParseGreyAlphaError::InvalidFormat);
            }

            // parse hex 0–F for grey
            let grey_value = u8::from_str_radix(&hex[0..1], 16).map_err(ParseGreyAlphaError::ParseHex)?;
            // parse hex 0–F for alpha
            let alpha_value = u8::from_str_radix(&hex[1..2], 16).map_err(ParseGreyAlphaError::ParseHex)?;

            // scale from 0–15 into 0.0–1.0
            let grey = T::from(grey_value).ok_or(ParseGreyAlphaError::OutOfRange)? / T::from(15).unwrap();
            let alpha = T::from(alpha_value).ok_or(ParseGreyAlphaError::OutOfRange)? / T::from(15).unwrap();

            Ok(Self::new(grey, alpha))
        } else {
            // Look for two comma-separated float values
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
        let grey_index = (self.0 * max).round().to_u8().unwrap();
        let alpha_index = (self.1 * max).round().to_u8().unwrap();
        write!(f, "#{grey_index:X}{alpha_index:X}")
    }
}
