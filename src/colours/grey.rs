//! Monochrome colour representation.

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    num::ParseIntError,
    ops::AddAssign,
    str::FromStr,
};
use num_traits::{Float, ToPrimitive};

use crate::Colour;

/// Error parsing `Grey` from string.
#[derive(Debug)]
#[non_exhaustive]
pub enum ParseGreyError<E> {
    /// Error parsing float.
    ParseFloat(E),
    /// Error parsing hex string.
    ParseHex(ParseIntError),
    /// Value out of range.
    OutOfRange,
}

/// Monochrome colour.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Grey<T: Float>(T);

impl<T: Display + AddAssign + Float> Grey<T> {
    /// Create a new `Grey` instance.
    ///
    /// # Panics
    ///
    /// Panics if the component is not in [0, 1].
    #[inline]
    pub fn new(mut grey: T) -> Self {
        let tolerance = Self::tolerance();
        if grey < T::zero() - tolerance || grey > T::one() + tolerance {
            assert!(
                !(grey < T::zero() - tolerance || grey > T::one() + tolerance),
                "Grey component {grey} out of [0, 1]\u{b1}{tolerance}."
            );
        }
        grey = grey.max(T::zero()).min(T::one());
        Self(grey)
    }

    /// Get the grey component.
    #[inline]
    pub const fn grey(&self) -> T {
        self.0
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
}

impl<T: Display + AddAssign + Float> Colour<T, 1> for Grey<T> {
    #[inline]
    fn num_components(&self) -> usize {
        1
    }

    #[inline]
    fn components(&self) -> [T; 1] {
        [self.0]
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
        let value = (self.0 * max).round().to_u8().unwrap();
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
        (self.0 - other.0).abs() <= Self::tolerance()
    }
}

impl<T> FromStr for Grey<T>
where
    T: Display + AddAssign + Float + FromStr + ToPrimitive,
{
    type Err = ParseGreyError<<T as FromStr>::Err>;

    #[expect(clippy::min_ident_chars, reason = "The variable `s` for a string is idiomatic.")]
    #[expect(clippy::unwrap_in_result, reason = "Unwrap will not fail here.")]
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(hex) = s.trim().strip_prefix('#') {
            // parse hex 0–F into u8
            let value = u8::from_str_radix(hex, 16).map_err(ParseGreyError::ParseHex)?;
            // scale from 0–15 into 0.0–1.0
            let grey = T::from(value).ok_or(ParseGreyError::OutOfRange)? / T::from(15).unwrap();
            Ok(Self::new(grey))
        } else {
            // parse as float
            let f = s.parse::<T>().map_err(ParseGreyError::ParseFloat)?;
            Ok(Self::new(f))
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
        let index = (self.0 * max).round().to_u8().unwrap();
        write!(f, "#{index:X}")
    }
}
