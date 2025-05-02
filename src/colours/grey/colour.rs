//! Implements the `Colour` trait for `Grey`.

use core::{fmt::Display, num::ParseIntError, ops::AddAssign};
use num_traits::Float;

use crate::{Colour, Grey, ParseColourError};

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

    #[expect(clippy::unwrap_in_result, reason = "Unwrap will not fail here.")]
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn from_hex(hex: &str) -> Result<Self, ParseColourError<ParseIntError>> {
        let components = hex.trim().strip_prefix('#').ok_or(ParseColourError::InvalidFormat)?;
        match components.len() {
            // Short form: #G
            1 => {
                let value = u8::from_str_radix(components, 16).map_err(ParseColourError::ParseHex)?;
                // Expand short form (e.g., #F becomes #FF)
                let grey = T::from(value * 17).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                Ok(Self::new(grey))
            }
            // Long form: #GG
            2 => {
                let value = u8::from_str_radix(components, 16).map_err(ParseColourError::ParseHex)?;
                let grey = T::from(value).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                Ok(Self::new(grey))
            }
            _ => Err(ParseColourError::InvalidFormat),
        }
    }

    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn to_hex(self) -> String {
        let max = T::from(255_i32).unwrap();
        let grey = (self.grey * max).round().to_u8().unwrap();
        format!("#{grey:02X}")
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
