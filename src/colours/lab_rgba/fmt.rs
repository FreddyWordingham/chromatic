//! Read and write `LabRgba` as a string.

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::AddAssign,
    str::FromStr,
};
use num_traits::{Float, ToPrimitive};

use crate::{LabRgba, ParseColourError};

impl<T> FromStr for LabRgba<T>
where
    T: Display + AddAssign + Float + FromStr + ToPrimitive,
{
    type Err = ParseColourError<<T as FromStr>::Err>;

    #[expect(clippy::min_ident_chars, reason = "The variable `s` for a string is idiomatic.")]
    #[expect(clippy::unwrap_in_result, reason = "Unwrap will not fail here.")]
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(hex) = s.trim().strip_prefix('#') {
            match hex.len() {
                // Short form: #RGBA
                4 => {
                    let mut chars = hex.chars();
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
                    let mut chars = hex.chars();
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
        } else {
            // Look for comma-separated float values
            let parts: Vec<&str> = s.split(',').collect();
            if parts.len() != 4 {
                return Err(ParseColourError::InvalidFormat);
            }

            let red = parts[0].trim().parse::<T>().map_err(ParseColourError::ParseFloat)?;
            let green = parts[1].trim().parse::<T>().map_err(ParseColourError::ParseFloat)?;
            let blue = parts[2].trim().parse::<T>().map_err(ParseColourError::ParseFloat)?;
            let alpha = parts[3].trim().parse::<T>().map_err(ParseColourError::ParseFloat)?;

            Ok(Self::new(red, green, blue, alpha))
        }
    }
}

impl<T> Display for LabRgba<T>
where
    T: Float + ToPrimitive,
{
    #[expect(clippy::min_ident_chars, reason = "The variable `f` for a formatter is idiomatic.")]
    #[expect(clippy::unwrap_in_result, reason = "Unwrap will not fail here.")]
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let rgb = self.rgb_components();
        let max = T::from(255_u8).unwrap();
        let red = (rgb[0] * max).round().to_u8().unwrap();
        let green = (rgb[1] * max).round().to_u8().unwrap();
        let blue = (rgb[2] * max).round().to_u8().unwrap();
        let alpha = (self.alpha * max).round().to_u8().unwrap();
        write!(f, "#{red:02X}{green:02X}{blue:02X}{alpha:02X}")
    }
}
