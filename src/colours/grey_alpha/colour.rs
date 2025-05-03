//! Implements the `Colour` trait for `GreyAlpha`.

use core::num::ParseIntError;
use num_traits::Float;

use crate::{Colour, GreyAlpha, ParseColourError};

impl<T: Float> Colour<T, 2> for GreyAlpha<T> {
    #[expect(clippy::unwrap_in_result, reason = "Unwrap will not fail here.")]
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn from_hex(hex: &str) -> Result<Self, ParseColourError<ParseIntError>> {
        let components = hex.trim().strip_prefix('#').ok_or(ParseColourError::InvalidFormat)?;
        let mut chars = components.chars();

        let (grey, alpha) = match components.len() {
            // Short form: #GA
            2 => {
                let grey_digit = chars.next().unwrap();
                let alpha_digit = chars.next().unwrap();

                let grey_value = u8::from_str_radix(&grey_digit.to_string(), 16).map_err(ParseColourError::ParseHex)?;
                let alpha_value = u8::from_str_radix(&alpha_digit.to_string(), 16).map_err(ParseColourError::ParseHex)?;

                // Expand short form (e.g., #FA becomes #FFAA)
                let grey = T::from(grey_value * 17).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                let alpha = T::from(alpha_value * 17).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();

                (grey, alpha)
            }
            // Long form: #GGAA
            4 => {
                let g1 = chars.next().unwrap().to_string();
                let g2 = chars.next().unwrap().to_string();
                let a1 = chars.next().unwrap().to_string();
                let a2 = chars.next().unwrap().to_string();

                let grey_value = u8::from_str_radix(&format!("{g1}{g2}"), 16).map_err(ParseColourError::ParseHex)?;
                let alpha_value = u8::from_str_radix(&format!("{a1}{a2}"), 16).map_err(ParseColourError::ParseHex)?;

                let grey = T::from(grey_value).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                let alpha = T::from(alpha_value).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();

                (grey, alpha)
            }
            _ => return Err(ParseColourError::InvalidFormat),
        };

        Ok(Self::new(grey, alpha))
    }

    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn to_hex(self) -> String {
        let max = T::from(255_i32).unwrap();
        let grey = (self.grey * max).round().to_u8().unwrap();
        let alpha = (self.alpha * max).round().to_u8().unwrap();
        format!("#{grey:02X}{alpha:02X}")
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
        assert!(
            t >= T::zero() && t <= T::one(),
            "Interpolation factor must be in range [0, 1]."
        );
        Self::new(
            lhs.grey() * (T::one() - t) + rhs.grey() * t,
            lhs.alpha() * (T::one() - t) + rhs.alpha() * t,
        )
    }
}
