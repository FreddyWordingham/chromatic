//! Implements the `Colour` trait for `Grey`.

use core::num::ParseIntError;
use num_traits::Float;

use crate::{Colour, Grey, ParseColourError};

impl<T: Float + Send + Sync> Colour<T, 1> for Grey<T> {
    #[inline]
    fn from_hex(hex: &str) -> Result<Self, ParseColourError<ParseIntError>> {
        let components = hex.trim().strip_prefix('#').ok_or(ParseColourError::InvalidFormat)?;
        let grey = match components.len() {
            // Short form: #G
            1 => {
                let value = u8::from_str_radix(components, 16).map_err(ParseColourError::ParseHex)?;
                // Expand short form (e.g., #F becomes #FF)
                T::from(value * 17).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap()
            }
            // Long form: #GG
            2 => {
                let value = u8::from_str_radix(components, 16).map_err(ParseColourError::ParseHex)?;
                T::from(value).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap()
            }
            _ => return Err(ParseColourError::InvalidFormat),
        };
        Ok(Self::new(grey))
    }

    #[inline]
    fn to_hex(&self) -> String {
        let max = T::from(255_i32).unwrap();
        let grey = (self.grey * max).round().to_u8().unwrap();
        format!("#{grey:02X}")
    }

    #[inline]
    fn from_bytes(bytes: [u8; 1]) -> Self {
        let max = T::from(255_u8).unwrap();
        let value = T::from(bytes[0]).unwrap() / max;
        Self::new(value)
    }

    #[inline]
    fn to_bytes(self) -> [u8; 1] {
        let max = T::from(255_u8).unwrap();
        let value = (self.grey * max).round().to_u8().unwrap();
        [value]
    }

    #[inline]
    fn lerp(lhs: &Self, rhs: &Self, t: T) -> Self {
        debug_assert!(
            t >= T::zero() && t <= T::one(),
            "Interpolation factor must be in range [0, 1]."
        );
        Self::new(lhs.grey() * (T::one() - t) + rhs.grey() * t)
    }
}
