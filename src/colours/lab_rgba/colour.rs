//! Implements the `Colour` trait for `LabRgba`.

use core::{fmt::Display, num::ParseIntError, ops::AddAssign};
use num_traits::Float;

use crate::{
    Colour, LabRgba, ParseColourError,
    colours::lab_utils::{rgb_to_xyz_components, xyz_to_lab},
};

impl<T: Display + AddAssign + Float> Colour<T, 4> for LabRgba<T> {
    #[inline]
    fn from_components(components: [T; 4]) -> Self {
        Self::new(components[0], components[1], components[2], components[3])
    }

    #[inline]
    fn components(&self) -> [T; 4] {
        let [red, green, blue] = self.rgb_components();
        [red, green, blue, self.alpha]
    }

    #[inline]
    fn set_components(&mut self, components: [T; 4]) {
        // Convert the RGB array to Lab at once, keep alpha separate
        let rgb = [components[0], components[1], components[2]];
        let xyz = rgb_to_xyz_components(&rgb);
        let lab = xyz_to_lab(&xyz);
        self.lightness = lab[0];
        self.a_axis = lab[1];
        self.b_axis = lab[2];
        self.alpha = components[3];
    }

    #[inline]
    fn from_hex(hex: &str) -> Result<Self, ParseColourError<ParseIntError>> {
        let components = hex.trim().strip_prefix('#').ok_or(ParseColourError::InvalidFormat)?;
        match components.len() {
            // Short form: #RGBA
            4 => {
                let mut chars = components.chars();
                let r_digit = chars.next().unwrap();
                let g_digit = chars.next().unwrap();
                let b_digit = chars.next().unwrap();
                let a_digit = chars.next().unwrap();

                let red = u8::from_str_radix(&r_digit.to_string(), 16).map_err(ParseColourError::ParseHex)?;
                let green = u8::from_str_radix(&g_digit.to_string(), 16).map_err(ParseColourError::ParseHex)?;
                let blue = u8::from_str_radix(&b_digit.to_string(), 16).map_err(ParseColourError::ParseHex)?;
                let alpha = u8::from_str_radix(&a_digit.to_string(), 16).map_err(ParseColourError::ParseHex)?;

                // Expand short form (e.g., #F00F becomes #FF00FF)
                let scaled_red = T::from(red * 17).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                let scaled_green = T::from(green * 17).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                let scaled_blue = T::from(blue * 17).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                let scaled_alpha = T::from(alpha * 17).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();

                Ok(Self::new(scaled_red, scaled_green, scaled_blue, scaled_alpha))
            }
            // Long form: #RRGGBBAA
            8 => {
                let mut chars = components.chars();
                let r1 = chars.next().unwrap().to_string();
                let r2 = chars.next().unwrap().to_string();
                let g1 = chars.next().unwrap().to_string();
                let g2 = chars.next().unwrap().to_string();
                let b1 = chars.next().unwrap().to_string();
                let b2 = chars.next().unwrap().to_string();
                let a1 = chars.next().unwrap().to_string();
                let a2 = chars.next().unwrap().to_string();

                let red = u8::from_str_radix(&format!("{r1}{r2}"), 16).map_err(ParseColourError::ParseHex)?;
                let green = u8::from_str_radix(&format!("{g1}{g2}"), 16).map_err(ParseColourError::ParseHex)?;
                let blue = u8::from_str_radix(&format!("{b1}{b2}"), 16).map_err(ParseColourError::ParseHex)?;
                let alpha = u8::from_str_radix(&format!("{a1}{a2}"), 16).map_err(ParseColourError::ParseHex)?;

                let scaled_red = T::from(red).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                let scaled_green = T::from(green).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                let scaled_blue = T::from(blue).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                let scaled_alpha = T::from(alpha).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();

                Ok(Self::new(scaled_red, scaled_green, scaled_blue, scaled_alpha))
            }
            _ => Err(ParseColourError::InvalidFormat),
        }
    }

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
        Self::new(red, green, blue, alpha)
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
        assert!(t >= T::zero() && t <= T::one(), "Interpolation factor {t} out of [0, 1].");

        // Direct interpolation in Lab space with separate alpha interpolation
        let l = lhs.lightness * (T::one() - t) + rhs.lightness * t;
        let a = lhs.a_axis * (T::one() - t) + rhs.a_axis * t;
        let b = lhs.b_axis * (T::one() - t) + rhs.b_axis * t;
        let alpha = lhs.alpha * (T::one() - t) + rhs.alpha * t;

        // Create result directly in Lab space
        Self {
            lightness: l,
            a_axis: a,
            b_axis: b,
            alpha,
        }
    }
}
