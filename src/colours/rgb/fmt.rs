//! Read and write `Rgb` as a string.

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::AddAssign,
    str::FromStr,
};
use num_traits::{Float, ToPrimitive};

use crate::{ParseColourError, Rgb};

impl<T> FromStr for Rgb<T>
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
                // Short form: #RGB
                3 => {
                    let mut chars = hex.chars();
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
                    let mut chars = hex.chars();
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
        } else {
            // Look for comma-separated float values
            let parts: Vec<&str> = s.split(',').collect();
            if parts.len() != 3 {
                return Err(ParseColourError::InvalidFormat);
            }

            let red = parts[0].trim().parse::<T>().map_err(ParseColourError::ParseFloat)?;
            let green = parts[1].trim().parse::<T>().map_err(ParseColourError::ParseFloat)?;
            let blue = parts[2].trim().parse::<T>().map_err(ParseColourError::ParseFloat)?;

            Ok(Self::new(red, green, blue))
        }
    }
}

impl<T> Display for Rgb<T>
where
    T: Float + ToPrimitive,
{
    #[expect(clippy::min_ident_chars, reason = "The variable `f` for a formatter is idiomatic.")]
    #[expect(clippy::unwrap_in_result, reason = "Unwrap will not fail here.")]
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let max = T::from(255_u8).unwrap();
        let red = (self.red * max).round().to_u8().unwrap();
        let green = (self.green * max).round().to_u8().unwrap();
        let blue = (self.blue * max).round().to_u8().unwrap();
        write!(f, "#{red:02X}{green:02X}{blue:02X}")
    }
}
