//! RGB colour representation.

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    num::ParseIntError,
    ops::AddAssign,
    str::FromStr,
};
use num_traits::{Float, ToPrimitive};

use crate::Colour;

/// Error parsing `Rgb` from string.
#[derive(Debug)]
#[non_exhaustive]
pub enum ParseRgbError<E> {
    /// Error parsing float.
    ParseFloat(E),
    /// Error parsing hex string.
    ParseHex(ParseIntError),
    /// Value out of range.
    OutOfRange,
    /// Invalid format.
    InvalidFormat,
}

/// RGB colour representation.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Rgb<T: Float>(T, T, T);

impl<T: Display + AddAssign + Float> Rgb<T> {
    /// Create a new `Rgb` instance.
    ///
    /// # Panics
    ///
    /// Panics if any component is not in [0, 1].
    #[inline]
    pub fn new(mut red: T, mut green: T, mut blue: T) -> Self {
        let tolerance = Self::tolerance();
        if red < T::zero() - tolerance || red > T::one() + tolerance {
            assert!(
                !(red < T::zero() - tolerance || red > T::one() + tolerance),
                "Red component {red} out of [0, 1]\u{b1}{tolerance}."
            );
        }
        if green < T::zero() - tolerance || green > T::one() + tolerance {
            assert!(
                !(green < T::zero() - tolerance || green > T::one() + tolerance),
                "Green component {green} out of [0, 1]\u{b1}{tolerance}."
            );
        }
        if blue < T::zero() - tolerance || blue > T::one() + tolerance {
            assert!(
                !(blue < T::zero() - tolerance || blue > T::one() + tolerance),
                "Blue component {blue} out of [0, 1]\u{b1}{tolerance}."
            );
        }
        red = red.clamp(T::zero(), T::one());
        green = green.clamp(T::zero(), T::one());
        blue = blue.clamp(T::zero(), T::one());
        Self(red, green, blue)
    }

    /// Get the red component.
    #[inline]
    pub const fn red(&self) -> T {
        self.0
    }

    /// Get the green component.
    #[inline]
    pub const fn green(&self) -> T {
        self.1
    }

    /// Get the blue component.
    #[inline]
    pub const fn blue(&self) -> T {
        self.2
    }

    /// Set the red component.
    ///
    /// # Panics
    ///
    /// Panics if the value is not in [0, 1].
    #[inline]
    pub fn set_red(&mut self, red: T) {
        assert!(red >= T::zero() && red <= T::one(), "Red component must be between 0 and 1.");
        self.0 = red;
    }

    /// Set the green component.
    ///
    /// # Panics
    ///
    /// Panics if the value is not in [0, 1].
    #[inline]
    pub fn set_green(&mut self, green: T) {
        assert!(
            green >= T::zero() && green <= T::one(),
            "Green component must be between 0 and 1."
        );
        self.1 = green;
    }

    /// Set the blue component.
    ///
    /// # Panics
    ///
    /// Panics if the value is not in [0, 1].
    #[inline]
    pub fn set_blue(&mut self, blue: T) {
        assert!(
            blue >= T::zero() && blue <= T::one(),
            "Blue component must be between 0 and 1."
        );
        self.2 = blue;
    }
}

impl<T: Display + AddAssign + Float> Colour<T, 3> for Rgb<T> {
    #[inline]
    fn from_components(components: [T; 3]) -> Self {
        Self::new(components[0], components[1], components[2])
    }

    #[inline]
    fn components(&self) -> [T; 3] {
        [self.0, self.1, self.2]
    }

    #[inline]
    fn set_components(&mut self, components: [T; 3]) {
        self.set_red(components[0]);
        self.set_green(components[1]);
        self.set_blue(components[2]);
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
        let red = (self.0 * max).round().to_u8().unwrap();
        let green = (self.1 * max).round().to_u8().unwrap();
        let blue = (self.2 * max).round().to_u8().unwrap();
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
        assert!(t >= T::zero() && t <= T::one(), "Interpolation factor {t} out of [0, 1].");
        Self::new(
            lhs.red() * (T::one() - t) + rhs.red() * t,
            lhs.green() * (T::one() - t) + rhs.green() * t,
            lhs.blue() * (T::one() - t) + rhs.blue() * t,
        )
    }
}

impl<T: Float + AddAssign + Display> PartialEq for Rgb<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() <= Self::tolerance()
            && (self.1 - other.1).abs() <= Self::tolerance()
            && (self.2 - other.2).abs() <= Self::tolerance()
    }
}

impl<T> FromStr for Rgb<T>
where
    T: Display + AddAssign + Float + FromStr + ToPrimitive,
{
    type Err = ParseRgbError<<T as FromStr>::Err>;

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

                    let red = u8::from_str_radix(&r_digit.to_string(), 16).map_err(ParseRgbError::ParseHex)?;
                    let green = u8::from_str_radix(&g_digit.to_string(), 16).map_err(ParseRgbError::ParseHex)?;
                    let blue = u8::from_str_radix(&b_digit.to_string(), 16).map_err(ParseRgbError::ParseHex)?;

                    // Expand short form (e.g., #F00 becomes #FF0000)
                    let scaled_red = T::from(red * 17).ok_or(ParseRgbError::OutOfRange)? / T::from(255).unwrap();
                    let scaled_green = T::from(green * 17).ok_or(ParseRgbError::OutOfRange)? / T::from(255).unwrap();
                    let scaled_blue = T::from(blue * 17).ok_or(ParseRgbError::OutOfRange)? / T::from(255).unwrap();

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

                    let red = u8::from_str_radix(&format!("{r1}{r2}"), 16).map_err(ParseRgbError::ParseHex)?;
                    let green = u8::from_str_radix(&format!("{g1}{g2}"), 16).map_err(ParseRgbError::ParseHex)?;
                    let blue = u8::from_str_radix(&format!("{b1}{b2}"), 16).map_err(ParseRgbError::ParseHex)?;

                    let scaled_red = T::from(red).ok_or(ParseRgbError::OutOfRange)? / T::from(255).unwrap();
                    let scaled_green = T::from(green).ok_or(ParseRgbError::OutOfRange)? / T::from(255).unwrap();
                    let scaled_blue = T::from(blue).ok_or(ParseRgbError::OutOfRange)? / T::from(255).unwrap();

                    Ok(Self::new(scaled_red, scaled_green, scaled_blue))
                }
                _ => Err(ParseRgbError::InvalidFormat),
            }
        } else {
            // Look for comma-separated float values
            let parts: Vec<&str> = s.split(',').collect();
            if parts.len() != 3 {
                return Err(ParseRgbError::InvalidFormat);
            }

            let red = parts[0].trim().parse::<T>().map_err(ParseRgbError::ParseFloat)?;
            let green = parts[1].trim().parse::<T>().map_err(ParseRgbError::ParseFloat)?;
            let blue = parts[2].trim().parse::<T>().map_err(ParseRgbError::ParseFloat)?;

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
        let red = (self.0 * max).round().to_u8().unwrap();
        let green = (self.1 * max).round().to_u8().unwrap();
        let blue = (self.2 * max).round().to_u8().unwrap();
        write!(f, "#{red:02X}{green:02X}{blue:02X}")
    }
}
