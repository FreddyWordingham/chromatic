//! Implements the `Colour` trait for `Hsv`.

use core::num::ParseIntError;
use num_traits::Float;

use crate::{Colour, Hsv, ParseColourError};

impl<T: Float> Colour<T, 3> for Hsv<T> {
    #[expect(clippy::unwrap_in_result, reason = "Unwrap will not fail here.")]
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn from_hex(hex: &str) -> Result<Self, ParseColourError<ParseIntError>> {
        let components = hex.trim().strip_prefix('#').ok_or(ParseColourError::InvalidFormat)?;
        let mut chars = components.chars();

        let (red, green, blue) = match components.len() {
            3 => {
                let r_digit = chars.next().unwrap();
                let g_digit = chars.next().unwrap();
                let b_digit = chars.next().unwrap();

                let red = u8::from_str_radix(&r_digit.to_string(), 16).map_err(ParseColourError::ParseHex)?;
                let green = u8::from_str_radix(&g_digit.to_string(), 16).map_err(ParseColourError::ParseHex)?;
                let blue = u8::from_str_radix(&b_digit.to_string(), 16).map_err(ParseColourError::ParseHex)?;

                let scaled_red = T::from(red * 17).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                let scaled_green = T::from(green * 17).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                let scaled_blue = T::from(blue * 17).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();

                (scaled_red, scaled_green, scaled_blue)
            }
            6 => {
                let r1 = chars.next().unwrap();
                let r2 = chars.next().unwrap();
                let g1 = chars.next().unwrap();
                let g2 = chars.next().unwrap();
                let b1 = chars.next().unwrap();
                let b2 = chars.next().unwrap();

                let red = u8::from_str_radix(&format!("{r1}{r2}"), 16).map_err(ParseColourError::ParseHex)?;
                let green = u8::from_str_radix(&format!("{g1}{g2}"), 16).map_err(ParseColourError::ParseHex)?;
                let blue = u8::from_str_radix(&format!("{b1}{b2}"), 16).map_err(ParseColourError::ParseHex)?;

                let scaled_red = T::from(red).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                let scaled_green = T::from(green).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                let scaled_blue = T::from(blue).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();

                (scaled_red, scaled_green, scaled_blue)
            }
            _ => return Err(ParseColourError::InvalidFormat),
        };

        Ok(Self::from_rgb(red, green, blue))
    }

    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn to_hex(self) -> String {
        // Convert HSV to RGB for hex representation
        let (red, green, blue) = self.rgb_components();

        let max = T::from(255_u8).unwrap();
        let rounded_red = (red * max).round().to_u8().unwrap();
        let rounded_green = (green * max).round().to_u8().unwrap();
        let rounded_blue = (blue * max).round().to_u8().unwrap();

        format!("#{rounded_red:02X}{rounded_green:02X}{rounded_blue:02X}")
    }

    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn from_bytes(bytes: [u8; 3]) -> Self {
        let max = T::from(255_u8).unwrap();
        let red = T::from(bytes[0]).unwrap() / max;
        let green = T::from(bytes[1]).unwrap() / max;
        let blue = T::from(bytes[2]).unwrap() / max;

        Self::from_rgb(red, green, blue)
    }

    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn to_bytes(self) -> [u8; 3] {
        let (red, green, blue) = self.rgb_components();

        let max = T::from(255_u8).unwrap();
        let rounded_red = (red * max).round().to_u8().unwrap();
        let rounded_green = (green * max).round().to_u8().unwrap();
        let rounded_blue = (blue * max).round().to_u8().unwrap();

        [rounded_red, rounded_green, rounded_blue]
    }

    /// Linear interpolate between two `Hsv` colours.
    ///
    /// # Panics
    ///
    /// Panics if the interpolation factor is not in [0, 1].
    #[expect(
        clippy::min_ident_chars,
        reason = "The variable `t` for an interpolation factor is idiomatic."
    )]
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn lerp(lhs: &Self, rhs: &Self, t: T) -> Self {
        assert!(
            t >= T::zero() && t <= T::one(),
            "Interpolation factor must be in range [0, 1]."
        );

        // For hue, we need special handling to ensure we take the shortest path around the color wheel
        let mut hue_diff = rhs.hue - lhs.hue;

        // If the difference is greater than 180 degrees, it's shorter to go the other way around the color wheel
        if hue_diff > T::from(180).unwrap() {
            hue_diff = hue_diff - T::from(360).unwrap();
        } else if hue_diff < T::from(-180).unwrap() {
            hue_diff = hue_diff + T::from(360).unwrap();
        }

        // Calculate the interpolated hue and ensure it stays in [0, 360] range
        let mut hue = lhs.hue + t * hue_diff;
        if hue < T::zero() {
            hue = hue + T::from(360).unwrap();
        } else if hue > T::from(360).unwrap() {
            hue = hue - T::from(360).unwrap();
        }

        // Linear interpolation for saturation and value
        let saturation = lhs.saturation * (T::one() - t) + rhs.saturation * t;
        let value = lhs.value * (T::one() - t) + rhs.value * t;

        Self::new(hue, saturation, value)
    }
}
