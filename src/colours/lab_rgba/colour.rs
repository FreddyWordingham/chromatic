//! Implements the `Colour` trait for `LabRgba`.

use core::num::ParseIntError;
use num_traits::Float;

use crate::{Colour, LabRgba, ParseColourError};

impl<T: Float> Colour<T, 4> for LabRgba<T> {
    #[expect(clippy::unwrap_in_result, reason = "Unwrap will not fail here.")]
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn from_hex(hex: &str) -> Result<Self, ParseColourError<ParseIntError>> {
        let components = hex.trim().strip_prefix('#').ok_or(ParseColourError::InvalidFormat)?;
        let mut chars = components.chars();

        let (red, green, blue, alpha) = match components.len() {
            // Short form: #RGBA
            4 => {
                let r_digit = chars.next().unwrap();
                let g_digit = chars.next().unwrap();
                let b_digit = chars.next().unwrap();
                let a_digit = chars.next().unwrap();

                // Use to_digit instead of from_str_radix to avoid string allocations
                let red = match r_digit.to_digit(16) {
                    Some(v) => v as u8,
                    None => return Err(ParseColourError::InvalidFormat),
                };
                let green = match g_digit.to_digit(16) {
                    Some(v) => v as u8,
                    None => return Err(ParseColourError::InvalidFormat),
                };
                let blue = match b_digit.to_digit(16) {
                    Some(v) => v as u8,
                    None => return Err(ParseColourError::InvalidFormat),
                };
                let alpha = match a_digit.to_digit(16) {
                    Some(v) => v as u8,
                    None => return Err(ParseColourError::InvalidFormat),
                };

                // Expand short form (e.g., #F00F becomes #FF00FF)
                let scaled_red = T::from(red * 17).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                let scaled_green = T::from(green * 17).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                let scaled_blue = T::from(blue * 17).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                let scaled_alpha = T::from(alpha * 17).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();

                (scaled_red, scaled_green, scaled_blue, scaled_alpha)
            }
            // Long form: #RRGGBBAA
            8 => {
                // Process two characters at a time to avoid string allocations
                let r1 = match chars.next().unwrap().to_digit(16) {
                    Some(v) => v as u8,
                    None => return Err(ParseColourError::InvalidFormat),
                };
                let r2 = match chars.next().unwrap().to_digit(16) {
                    Some(v) => v as u8,
                    None => return Err(ParseColourError::InvalidFormat),
                };
                let g1 = match chars.next().unwrap().to_digit(16) {
                    Some(v) => v as u8,
                    None => return Err(ParseColourError::InvalidFormat),
                };
                let g2 = match chars.next().unwrap().to_digit(16) {
                    Some(v) => v as u8,
                    None => return Err(ParseColourError::InvalidFormat),
                };
                let b1 = match chars.next().unwrap().to_digit(16) {
                    Some(v) => v as u8,
                    None => return Err(ParseColourError::InvalidFormat),
                };
                let b2 = match chars.next().unwrap().to_digit(16) {
                    Some(v) => v as u8,
                    None => return Err(ParseColourError::InvalidFormat),
                };
                let a1 = match chars.next().unwrap().to_digit(16) {
                    Some(v) => v as u8,
                    None => return Err(ParseColourError::InvalidFormat),
                };
                let a2 = match chars.next().unwrap().to_digit(16) {
                    Some(v) => v as u8,
                    None => return Err(ParseColourError::InvalidFormat),
                };

                let red = r1 << 4 | r2;
                let green = g1 << 4 | g2;
                let blue = b1 << 4 | b2;
                let alpha = a1 << 4 | a2;

                let scaled_red = T::from(red).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                let scaled_green = T::from(green).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                let scaled_blue = T::from(blue).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                let scaled_alpha = T::from(alpha).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();

                (scaled_red, scaled_green, scaled_blue, scaled_alpha)
            }
            _ => return Err(ParseColourError::InvalidFormat),
        };

        Ok(Self::from_rgba(red, green, blue, alpha))
    }

    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn to_hex(self) -> String {
        let rgb = self.rgb_components();
        let max = T::from(255_u8).unwrap();
        let red = (rgb[0] * max).round().to_u8().unwrap();
        let green = (rgb[1] * max).round().to_u8().unwrap();
        let blue = (rgb[2] * max).round().to_u8().unwrap();
        let alpha = (self.alpha * max).round().to_u8().unwrap();
        format!("#{red:02X}{green:02X}{blue:02X}{alpha:02X}")
    }

    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn from_bytes(bytes: [u8; 4]) -> Self {
        let max = T::from(255_u8).unwrap();
        let red = T::from(bytes[0]).unwrap() / max;
        let green = T::from(bytes[1]).unwrap() / max;
        let blue = T::from(bytes[2]).unwrap() / max;
        let alpha = T::from(bytes[3]).unwrap() / max;
        Self::from_rgba(red, green, blue, alpha)
    }

    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn to_bytes(self) -> [u8; 4] {
        let rgb = self.rgb_components();
        let max = T::from(255_u8).unwrap();
        let red = (rgb[0] * max).round().to_u8().unwrap();
        let green = (rgb[1] * max).round().to_u8().unwrap();
        let blue = (rgb[2] * max).round().to_u8().unwrap();
        let alpha = (self.alpha * max).round().to_u8().unwrap();
        [red, green, blue, alpha]
    }

    /// Linear interpolate between two RGBA colours using Lab colour space for perceptual uniformity.
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

        // Direct interpolation in Lab space with separate alpha interpolation
        let l = lhs.lightness() * (T::one() - t) + rhs.lightness() * t;
        let a = lhs.a_axis() * (T::one() - t) + rhs.a_axis() * t;
        let b = lhs.b_axis() * (T::one() - t) + rhs.b_axis() * t;
        let alpha = lhs.alpha() * (T::one() - t) + rhs.alpha() * t;

        // Create result directly in Lab space
        Self {
            lightness: l,
            a_axis: a,
            b_axis: b,
            alpha,
        }
    }
}
