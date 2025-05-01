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
    pub fn gamma_lerp(lhs: &Self, rhs: &Self, t: T) -> Self {
        assert!(t >= T::zero() && t <= T::one(), "Interpolation factor {t} out of [0, 1].");
        Self::new(
            lhs.red() * (T::one() - t) + rhs.red() * t,
            lhs.green() * (T::one() - t) + rhs.green() * t,
            lhs.blue() * (T::one() - t) + rhs.blue() * t,
        )
    }

    /// Convert RGB to XYZ colour space
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn rgb_to_xyz(rgb: &Self) -> [T; 3] {
        // Convert from gamma-corrected RGB to linear RGB
        let red = Self::gamma_to_linear(rgb.red());
        let green = Self::gamma_to_linear(rgb.green());
        let blue = Self::gamma_to_linear(rgb.blue());

        // Convert to XYZ using sRGB standard matrix
        let x = red * T::from(0.4124564).unwrap() + green * T::from(0.3575761).unwrap() + blue * T::from(0.1804375).unwrap();
        let y = red * T::from(0.2126729).unwrap() + green * T::from(0.7151522).unwrap() + blue * T::from(0.0721750).unwrap();
        let z = red * T::from(0.0193339).unwrap() + green * T::from(0.1191920).unwrap() + blue * T::from(0.9503041).unwrap();

        [x, y, z]
    }

    /// Convert XYZ to Lab colour space
    #[expect(
        clippy::many_single_char_names,
        reason = "The variables `xyz` and `lab` are idiomatic for colour spaces."
    )]
    #[expect(
        clippy::min_ident_chars,
        reason = "The variables `xyz` and `lab` are idiomatic for colour spaces."
    )]
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn xyz_to_lab(xyz: &[T; 3]) -> [T; 3] {
        // Reference white (D65)
        let xn = T::from(0.95047).unwrap();
        let yn = T::from(1.0).unwrap();
        let zn = T::from(1.08883).unwrap();

        // Normalized XYZ
        let x = Self::lab_f(xyz[0] / xn);
        let y = Self::lab_f(xyz[1] / yn);
        let z = Self::lab_f(xyz[2] / zn);

        // Calculate Lab components
        let l = T::from(116.0).unwrap() * y - T::from(16.0).unwrap();
        let a = T::from(500.0).unwrap() * (x - y);
        let b = T::from(200.0).unwrap() * (y - z);

        [l, a, b]
    }

    /// Helper function for Lab conversion
    #[expect(
        clippy::min_ident_chars,
        reason = "The variable `t` for an interpolation factor is idiomatic."
    )]
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn lab_f(t: T) -> T {
        let delta = T::from(6.0 / 29.0).unwrap();
        let delta_cubed = delta * delta * delta;

        if t > delta_cubed {
            t.powf(T::from(1.0 / 3.0).unwrap())
        } else {
            t / (T::from(3.0).unwrap() * delta * delta) + T::from(4.0 / 29.0).unwrap()
        }
    }

    /// Convert Lab to XYZ colour space
    #[expect(
        clippy::single_call_fn,
        reason = "Packaging this code in a function makes it easier to maintain."
    )]
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn lab_to_xyz(lab: &[T; 3]) -> [T; 3] {
        // Reference white (D65)
        let xn = T::from(0.95047).unwrap();
        let yn = T::from(1.0).unwrap();
        let zn = T::from(1.08883).unwrap();

        // Calculate intermediate values
        let fy = (lab[0] + T::from(16.0).unwrap()) / T::from(116.0).unwrap();
        let fx = fy + (lab[1] / T::from(500.0).unwrap());
        let fz = fy - (lab[2] / T::from(200.0).unwrap());

        // Convert to XYZ
        let x = Self::lab_f_inv(fx) * xn;
        let y = Self::lab_f_inv(fy) * yn;
        let z = Self::lab_f_inv(fz) * zn;

        [x, y, z]
    }

    /// Inverse function for Lab conversion
    #[expect(
        clippy::min_ident_chars,
        reason = "The variable `t` for an interpolation factor is idiomatic."
    )]
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn lab_f_inv(t: T) -> T {
        let delta = T::from(6.0 / 29.0).unwrap();

        if t > delta {
            t * t * t
        } else {
            T::from(3.0).unwrap() * delta * delta * (t - T::from(4.0 / 29.0).unwrap())
        }
    }

    /// Convert XYZ to RGB colour space
    #[expect(
        clippy::single_call_fn,
        reason = "Packaging this code in a function makes it easier to maintain."
    )]
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn xyz_to_rgb(xyz: &[T; 3]) -> Self {
        // Convert XYZ to linear RGB using sRGB standard inverse matrix
        let red =
            xyz[0] * T::from(3.2404542).unwrap() - xyz[1] * T::from(1.5371385).unwrap() - xyz[2] * T::from(0.4985314).unwrap();
        let green =
            -xyz[0] * T::from(0.9692660).unwrap() + xyz[1] * T::from(1.8760108).unwrap() + xyz[2] * T::from(0.0415560).unwrap();
        let blue =
            xyz[0] * T::from(0.0556434).unwrap() - xyz[1] * T::from(0.2040259).unwrap() + xyz[2] * T::from(1.0572252).unwrap();

        // Convert from linear RGB to gamma-corrected RGB
        let red_gamma = Self::linear_to_gamma(red);
        let green_gamma = Self::linear_to_gamma(green);
        let blue_gamma = Self::linear_to_gamma(blue);

        // Clamp values to [0, 1] range
        let red_clamped = red_gamma.max(T::zero()).min(T::one());
        let green_clamped = green_gamma.max(T::zero()).min(T::one());
        let blue_clamped = blue_gamma.max(T::zero()).min(T::one());

        Self::new(red_clamped, green_clamped, blue_clamped)
    }

    /// Convert gamma-corrected RGB to linear RGB
    #[expect(clippy::min_ident_chars, reason = "There is only a single variable `c`.")]
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn gamma_to_linear(c: T) -> T {
        if c <= T::from(0.04045).unwrap() {
            c / T::from(12.92).unwrap()
        } else {
            ((c + T::from(0.055).unwrap()) / T::from(1.055).unwrap()).powf(T::from(2.4).unwrap())
        }
    }

    /// Convert linear RGB to gamma-corrected RGB
    #[expect(clippy::min_ident_chars, reason = "There is only a single variable `c`.")]
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn linear_to_gamma(c: T) -> T {
        if c <= T::from(0.0031308).unwrap() {
            c * T::from(12.92).unwrap()
        } else {
            c.powf(T::from(1.0 / 2.4).unwrap()) * T::from(1.055).unwrap() - T::from(0.055).unwrap()
        }
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

    /// Linear interpolate between two RGB colours using Lab colour space for perceptual uniformity.
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

        // Convert RGB to XYZ
        let xyz1 = Self::rgb_to_xyz(lhs);
        let xyz2 = Self::rgb_to_xyz(rhs);

        // Convert XYZ to Lab
        let lab1 = Self::xyz_to_lab(&xyz1);
        let lab2 = Self::xyz_to_lab(&xyz2);

        // Interpolate in Lab space
        let lab_result = [
            lab1[0] * (T::one() - t) + lab2[0] * t,
            lab1[1] * (T::one() - t) + lab2[1] * t,
            lab1[2] * (T::one() - t) + lab2[2] * t,
        ];

        // Convert back to XYZ
        let xyz_result = Self::lab_to_xyz(&lab_result);

        // Convert XYZ to RGB
        Self::xyz_to_rgb(&xyz_result)
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
