//! Read and write `Grey` as a string.

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::AddAssign,
    str::FromStr,
};
use num_traits::{Float, ToPrimitive};

use crate::{Grey, ParseColourError};

impl<T> FromStr for Grey<T>
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
                // Short form: #G
                1 => {
                    let value = u8::from_str_radix(hex, 16).map_err(ParseColourError::ParseHex)?;
                    // Expand short form (e.g., #F becomes #FF)
                    let grey = T::from(value * 17).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                    Ok(Self::new(grey))
                }
                // Long form: #GG
                2 => {
                    let value = u8::from_str_radix(hex, 16).map_err(ParseColourError::ParseHex)?;
                    let grey = T::from(value).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                    Ok(Self::new(grey))
                }
                _ => Err(ParseColourError::InvalidFormat),
            }
        } else {
            // Parse as comma-separated float values
            let parts: Vec<&str> = s.split(',').collect();
            if parts.len() != 1 {
                return Err(ParseColourError::InvalidFormat);
            }

            let grey = parts[0].trim().parse::<T>().map_err(ParseColourError::ParseFloat)?;
            Ok(Self::new(grey))
        }
    }
}

impl<T> Display for Grey<T>
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
        write!(f, "#{grey:X}")
    }
}
