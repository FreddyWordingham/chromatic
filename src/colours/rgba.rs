//! RGB colour representation with transparency.

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    num::ParseIntError,
    ops::AddAssign,
    str::FromStr,
};
use num_traits::{Float, ToPrimitive};

use crate::{Colour, Grey, GreyAlpha, LabRgb, LabRgba, Rgb};

/// Error parsing `Rgba` from string.
#[derive(Debug)]
#[non_exhaustive]
pub enum ParseRgbaError<E> {
    /// Error parsing float.
    ParseFloat(E),
    /// Error parsing hex string.
    ParseHex(ParseIntError),
    /// Value out of range.
    OutOfRange,
    /// Invalid format.
    InvalidFormat,
}

/// RGB colour representation with transparency.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Rgba<T: Float> {
    /// Red component.
    red: T,
    /// Green component.
    green: T,
    /// Blue component.
    blue: T,
    /// Alpha component.
    alpha: T,
}

impl<T: Display + AddAssign + Float> Rgba<T> {
    /// Create a new `Rgba` instance.
    ///
    /// # Panics
    ///
    /// Panics if any component is not in [0, 1].
    #[inline]
    pub fn new(red: T, green: T, blue: T, alpha: T) -> Self {
        assert!(!(red < T::zero() || red > T::one()), "Red component {red} out of [0, 1].");
        assert!(
            !(green < T::zero() || green > T::one()),
            "Green component {green} out of [0, 1]."
        );
        assert!(!(blue < T::zero() || blue > T::one()), "Blue component {blue} out of [0, 1].");
        assert!(
            !(alpha < T::zero() || alpha > T::one()),
            "Alpha component {alpha} out of [0, 1]."
        );
        Self { red, green, blue, alpha }
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

    /// Get the alpha component.
    #[inline]
    pub const fn alpha(&self) -> T {
        self.alpha
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

    /// Set the alpha component.
    ///
    /// # Panics
    ///
    /// Panics if the value is not in [0, 1].
    #[inline]
    pub fn set_alpha(&mut self, alpha: T) {
        assert!(
            alpha >= T::zero() && alpha <= T::one(),
            "Alpha component must be between 0 and 1."
        );
        self.alpha = alpha;
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
    pub fn to_grey_alpha(&self) -> GreyAlpha<T> {
        GreyAlpha::new((self.red + self.green + self.blue) / T::from(3).unwrap(), self.alpha)
    }

    /// Convert to `Rgb` by discarding the alpha channel.
    #[inline]
    pub fn to_rgb(&self) -> Rgb<T> {
        Rgb::new(self.red, self.green, self.blue)
    }

    /// Convert to `LabRgb`.
    #[inline]
    pub fn to_lab_rgb(&self) -> LabRgb<T> {
        LabRgb::new(self.red, self.green, self.blue)
    }

    /// Convert to `LabRgba`.
    #[inline]
    pub fn to_lab_rgba(&self) -> LabRgba<T> {
        LabRgba::new(self.red, self.green, self.blue, self.alpha)
    }
}

impl<T: Display + AddAssign + Float> Colour<T, 4> for Rgba<T> {
    #[inline]
    fn from_components(components: [T; 4]) -> Self {
        Self::new(components[0], components[1], components[2], components[3])
    }

    #[inline]
    fn components(&self) -> [T; 4] {
        [self.red, self.green, self.blue, self.alpha]
    }

    #[inline]
    fn set_components(&mut self, components: [T; 4]) {
        self.set_red(components[0]);
        self.set_green(components[1]);
        self.set_blue(components[2]);
        self.set_alpha(components[3]);
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
        let max = T::from(255_u8).unwrap();
        let red = (self.red * max).round().to_u8().unwrap();
        let green = (self.green * max).round().to_u8().unwrap();
        let blue = (self.blue * max).round().to_u8().unwrap();
        let alpha = (self.alpha * max).round().to_u8().unwrap();
        [red, green, blue, alpha]
    }

    /// Linear interpolate between two `Rgba` colours.
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
            lhs.alpha() * (T::one() - t) + rhs.alpha() * t,
        )
    }
}

impl<T: Display + AddAssign + Float> PartialEq for Rgba<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (self.red - other.red).abs() <= Self::tolerance()
            && (self.green - other.green).abs() <= Self::tolerance()
            && (self.blue - other.blue).abs() <= Self::tolerance()
            && (self.alpha - other.alpha).abs() <= Self::tolerance()
    }
}

impl<T> FromStr for Rgba<T>
where
    T: Display + AddAssign + Float + FromStr + ToPrimitive,
{
    type Err = ParseRgbaError<<T as FromStr>::Err>;

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

                    let red = u8::from_str_radix(&r_digit.to_string(), 16).map_err(ParseRgbaError::ParseHex)?;
                    let green = u8::from_str_radix(&g_digit.to_string(), 16).map_err(ParseRgbaError::ParseHex)?;
                    let blue = u8::from_str_radix(&b_digit.to_string(), 16).map_err(ParseRgbaError::ParseHex)?;
                    let alpha = u8::from_str_radix(&a_digit.to_string(), 16).map_err(ParseRgbaError::ParseHex)?;

                    // Expand short form (e.g., #F00F becomes #FF00FF)
                    let scaled_red = T::from(red * 17).ok_or(ParseRgbaError::OutOfRange)? / T::from(255).unwrap();
                    let scaled_green = T::from(green * 17).ok_or(ParseRgbaError::OutOfRange)? / T::from(255).unwrap();
                    let scaled_blue = T::from(blue * 17).ok_or(ParseRgbaError::OutOfRange)? / T::from(255).unwrap();
                    let scaled_alpha = T::from(alpha * 17).ok_or(ParseRgbaError::OutOfRange)? / T::from(255).unwrap();

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

                    let red = u8::from_str_radix(&format!("{r1}{r2}"), 16).map_err(ParseRgbaError::ParseHex)?;
                    let green = u8::from_str_radix(&format!("{g1}{g2}"), 16).map_err(ParseRgbaError::ParseHex)?;
                    let blue = u8::from_str_radix(&format!("{b1}{b2}"), 16).map_err(ParseRgbaError::ParseHex)?;
                    let alpha = u8::from_str_radix(&format!("{a1}{a2}"), 16).map_err(ParseRgbaError::ParseHex)?;

                    let scaled_red = T::from(red).ok_or(ParseRgbaError::OutOfRange)? / T::from(255).unwrap();
                    let scaled_green = T::from(green).ok_or(ParseRgbaError::OutOfRange)? / T::from(255).unwrap();
                    let scaled_blue = T::from(blue).ok_or(ParseRgbaError::OutOfRange)? / T::from(255).unwrap();
                    let scaled_alpha = T::from(alpha).ok_or(ParseRgbaError::OutOfRange)? / T::from(255).unwrap();

                    Ok(Self::new(scaled_red, scaled_green, scaled_blue, scaled_alpha))
                }
                _ => Err(ParseRgbaError::InvalidFormat),
            }
        } else {
            // Look for comma-separated float values
            let parts: Vec<&str> = s.split(',').collect();
            if parts.len() != 4 {
                return Err(ParseRgbaError::InvalidFormat);
            }

            let red = parts[0].trim().parse::<T>().map_err(ParseRgbaError::ParseFloat)?;
            let green = parts[1].trim().parse::<T>().map_err(ParseRgbaError::ParseFloat)?;
            let blue = parts[2].trim().parse::<T>().map_err(ParseRgbaError::ParseFloat)?;
            let alpha = parts[3].trim().parse::<T>().map_err(ParseRgbaError::ParseFloat)?;

            Ok(Self::new(red, green, blue, alpha))
        }
    }
}

impl<T> Display for Rgba<T>
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
        let alpha = (self.alpha * max).round().to_u8().unwrap();
        write!(f, "#{red:02X}{green:02X}{blue:02X}{alpha:02X}")
    }
}
