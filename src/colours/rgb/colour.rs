//! Implements the `Colour` trait for `Rgb`.

use core::num::ParseIntError;
use num_traits::Float;

use crate::{Colour, ParseColourError, Rgb};

impl<T: Float> Colour<T, 3> for Rgb<T> {
    #[inline]
    fn from_components(components: [T; 3]) -> Self {
        Self::new(components[0], components[1], components[2])
    }

    #[inline]
    fn components(&self) -> [T; 3] {
        [self.red, self.green, self.blue]
    }

    #[inline]
    fn set_components(&mut self, components: [T; 3]) {
        self.set_red(components[0]);
        self.set_green(components[1]);
        self.set_blue(components[2]);
    }

    #[expect(clippy::unwrap_in_result, reason = "Unwrap will not fail here.")]
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn from_hex(hex: &str) -> Result<Self, ParseColourError<ParseIntError>> {
        let components = hex.trim().strip_prefix('#').ok_or(ParseColourError::InvalidFormat)?;
        match components.len() {
            // Short form: #RGB
            3 => {
                let mut chars = components.chars();
                let r_digit = chars.next().unwrap();
                let g_digit = chars.next().unwrap();
                let b_digit = chars.next().unwrap();

                let red = u8::from_str_radix(&r_digit.to_string(), 16).map_err(ParseColourError::ParseHex)?;
                let green = u8::from_str_radix(&g_digit.to_string(), 16).map_err(ParseColourError::ParseHex)?;
                let blue = u8::from_str_radix(&b_digit.to_string(), 16).map_err(ParseColourError::ParseHex)?;

                // Expand short form (e.g., #F00 becomes #FF0000)
                let scaled_red = T::from(red * 17).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                let scaled_green = T::from(green * 17).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                let scaled_blue = T::from(blue * 17).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();

                Ok(Self::new(scaled_red, scaled_green, scaled_blue))
            }
            // Long form: #RRGGBB
            6 => {
                let mut chars = components.chars();
                let r1 = chars.next().unwrap().to_string();
                let r2 = chars.next().unwrap().to_string();
                let g1 = chars.next().unwrap().to_string();
                let g2 = chars.next().unwrap().to_string();
                let b1 = chars.next().unwrap().to_string();
                let b2 = chars.next().unwrap().to_string();

                let red = u8::from_str_radix(&format!("{r1}{r2}"), 16).map_err(ParseColourError::ParseHex)?;
                let green = u8::from_str_radix(&format!("{g1}{g2}"), 16).map_err(ParseColourError::ParseHex)?;
                let blue = u8::from_str_radix(&format!("{b1}{b2}"), 16).map_err(ParseColourError::ParseHex)?;

                let scaled_red = T::from(red).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                let scaled_green = T::from(green).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                let scaled_blue = T::from(blue).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();

                Ok(Self::new(scaled_red, scaled_green, scaled_blue))
            }
            _ => Err(ParseColourError::InvalidFormat),
        }
    }

    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn to_hex(self) -> String {
        let max = T::from(255_u8).unwrap();
        let red = (self.red * max).round().to_u8().unwrap();
        let green = (self.green * max).round().to_u8().unwrap();
        let blue = (self.blue * max).round().to_u8().unwrap();
        format!("#{red:02X}{green:02X}{blue:02X}")
    }

    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn from_bytes(bytes: [u8; 3]) -> Self {
        let max = T::from(255_u8).unwrap();
        let red = T::from(bytes[0]).unwrap() / max;
        let green = T::from(bytes[1]).unwrap() / max;
        let blue = T::from(bytes[2]).unwrap() / max;
        Self::new(red, green, blue)
    }

    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn to_bytes(self) -> [u8; 3] {
        let max = T::from(255_u8).unwrap();
        let red = (self.red * max).round().to_u8().unwrap();
        let green = (self.green * max).round().to_u8().unwrap();
        let blue = (self.blue * max).round().to_u8().unwrap();
        [red, green, blue]
    }

    /// Linear interpolate between two RGB colours.
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
            lhs.red() * (T::one() - t) + rhs.red() * t,
            lhs.green() * (T::one() - t) + rhs.green() * t,
            lhs.blue() * (T::one() - t) + rhs.blue() * t,
        )
    }
}
