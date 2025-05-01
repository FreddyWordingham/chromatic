//! RGB colour representation.

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    num::ParseIntError,
    ops::AddAssign,
    str::FromStr,
};
use num_traits::{Float, ToPrimitive};

use crate::{Colour, Grey, GreyAlpha, LabRgb, LabRgba, Rgba};

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
pub struct Rgb<T: Float> {
    /// Red component.
    red: T,
    /// Green component.
    green: T,
    /// Blue component.
    blue: T,
}

impl<T: Display + AddAssign + Float> Rgb<T> {
    /// Create a new `Rgb` instance.
    ///
    /// # Panics
    ///
    /// Panics if any component is not in [0, 1].
    #[inline]
    pub fn new(red: T, green: T, blue: T) -> Self {
        assert!(!(red < T::zero() || red > T::one()), "Red component {red} out of [0, 1].");
        assert!(
            !(green < T::zero() || green > T::one()),
            "Green component {green} out of [0, 1]."
        );
        assert!(!(blue < T::zero() || blue > T::one()), "Blue component {blue} out of [0, 1].");
        Self { red, green, blue }
    }

    /// Get the red component.
    #[inline]
    pub const fn red(&self) -> T {
        self.red
    }

    /// Get the green component.
    #[inline]
    pub const fn green(&self) -> T {
        self.green
    }

    /// Get the blue component.
    #[inline]
    pub const fn blue(&self) -> T {
        self.blue
    }

    /// Set the red component.
    ///
    /// # Panics
    ///
    /// Panics if the value is not in [0, 1].
    #[inline]
    pub fn set_red(&mut self, red: T) {
        assert!(red >= T::zero() && red <= T::one(), "Red component must be between 0 and 1.");
        self.red = red;
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
        self.green = green;
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
        self.blue = blue;
    }

    /// Convert to `Grey` by averaging the RGB components.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    pub fn to_grey(&self) -> Grey<T> {
        Grey::new((self.red + self.green + self.blue) / T::from(3).unwrap())
    }

    /// Convert to `GreyAlpha` by averaging the RGB components.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    pub fn to_grey_alpha(&self, alpha: T) -> GreyAlpha<T> {
        GreyAlpha::new((self.red + self.green + self.blue) / T::from(3).unwrap(), alpha)
    }

    /// Convert to `Rgba`.
    #[inline]
    pub fn to_rgba(&self, alpha: T) -> Rgba<T> {
        Rgba::new(self.red, self.green, self.blue, alpha)
    }

    /// Convert to `LabRgb`.
    #[inline]
    pub fn to_lab_rgb(&self) -> LabRgb<T> {
        LabRgb::new(self.red, self.green, self.blue)
    }

    /// Convert to `LabRgba`.
    #[inline]
    pub fn to_lab_rgba(&self, alpha: T) -> LabRgba<T> {
        LabRgba::new(self.red, self.green, self.blue, alpha)
    }
}

impl<T: Display + AddAssign + Float> Colour<T, 3> for Rgb<T> {
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
        assert!(t >= T::zero() && t <= T::one(), "Interpolation factor {t} out of [0, 1].");
        Self::new(
            lhs.red() * (T::one() - t) + rhs.red() * t,
            lhs.green() * (T::one() - t) + rhs.green() * t,
            lhs.blue() * (T::one() - t) + rhs.blue() * t,
        )
    }
}

impl<T: Display + AddAssign + Float> PartialEq for Rgb<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (self.red - other.red).abs() <= Self::tolerance()
            && (self.green - other.green).abs() <= Self::tolerance()
            && (self.blue - other.blue).abs() <= Self::tolerance()
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
        let red = (self.red * max).round().to_u8().unwrap();
        let green = (self.green * max).round().to_u8().unwrap();
        let blue = (self.blue * max).round().to_u8().unwrap();
        write!(f, "#{red:02X}{green:02X}{blue:02X}")
    }
}
