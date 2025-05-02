//! Read and write `GreyAlpha` as a string.

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::AddAssign,
    str::FromStr,
};
use num_traits::{Float, ToPrimitive};

use crate::{GreyAlpha, ParseColourError};

impl<T> FromStr for GreyAlpha<T>
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
                // Short form: #GA
                2 => {
                    let mut chars = hex.chars();
                    let grey_digit = chars.next().unwrap();
                    let alpha_digit = chars.next().unwrap();

                    let grey_value = u8::from_str_radix(&grey_digit.to_string(), 16).map_err(ParseColourError::ParseHex)?;
                    let alpha_value = u8::from_str_radix(&alpha_digit.to_string(), 16).map_err(ParseColourError::ParseHex)?;

                    // Expand short form (e.g., #FA becomes #FFAA)
                    let grey = T::from(grey_value * 17).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                    let alpha = T::from(alpha_value * 17).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();

                    Ok(Self::new(grey, alpha))
                }
                // Long form: #GGAA
                4 => {
                    let mut chars = hex.chars();
                    let g1 = chars.next().unwrap().to_string();
                    let g2 = chars.next().unwrap().to_string();
                    let a1 = chars.next().unwrap().to_string();
                    let a2 = chars.next().unwrap().to_string();

                    let grey_value = u8::from_str_radix(&format!("{g1}{g2}"), 16).map_err(ParseColourError::ParseHex)?;
                    let alpha_value = u8::from_str_radix(&format!("{a1}{a2}"), 16).map_err(ParseColourError::ParseHex)?;

                    let grey = T::from(grey_value).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                    let alpha = T::from(alpha_value).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();

                    Ok(Self::new(grey, alpha))
                }
                _ => Err(ParseColourError::InvalidFormat),
            }
        } else {
            // Look for comma-separated float values
            let parts: Vec<&str> = s.split(',').collect();
            if parts.len() != 2 {
                return Err(ParseColourError::InvalidFormat);
            }

            let grey = parts[0].trim().parse::<T>().map_err(ParseColourError::ParseFloat)?;
            let alpha = parts[1].trim().parse::<T>().map_err(ParseColourError::ParseFloat)?;

            Ok(Self::new(grey, alpha))
        }
    }
}

impl<T> Display for GreyAlpha<T>
where
    T: Float + ToPrimitive,
{
    #[expect(clippy::min_ident_chars, reason = "The variable `f` for a formatter is idiomatic.")]
    #[expect(clippy::unwrap_in_result, reason = "Unwrap will not fail here.")]
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let max = T::from(15_i32).unwrap();
        let grey = (self.grey * max).round().to_u8().unwrap();
        let alpha = (self.alpha * max).round().to_u8().unwrap();
        write!(f, "#{grey:X}{alpha:X}")
    }
}
