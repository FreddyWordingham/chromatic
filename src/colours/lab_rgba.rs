//! RGB colour representation with transparency using Lab colour space interpolation.

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::AddAssign,
    str::FromStr,
};
use num_traits::{Float, ToPrimitive};

use crate::{Colour, Grey, GreyAlpha, LabRgb, ParseRgbaError, Rgb, Rgba, colours::lab_utils::*};

/// RGB colour representation with transparency using Lab colour space internally.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct LabRgba<T: Float> {
    /// Lightness component (L*).
    lightness: T,
    /// A-axis component (a*).
    a_axis: T,
    /// B-axis component (b*).
    b_axis: T,
    /// Alpha (transparency) component.
    alpha: T,
}

impl<T: Float> LabRgba<T> {
    /// Convert Lab components and alpha to RGBA components.
    #[inline]
    fn rgb_components(&self) -> [T; 3] {
        let lab = [self.lightness, self.a_axis, self.b_axis];
        let xyz = lab_to_xyz(&lab);
        let rgb = xyz_to_rgb_components(&xyz);
        [rgb[0], rgb[1], rgb[2]]
    }
}

impl<T: Display + AddAssign + Float> LabRgba<T> {
    /// Create a new `LabRgba` instance.
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

        // Convert RGB to Lab
        let rgb = [red, green, blue];
        let xyz = rgb_to_xyz_components(&rgb);
        let lab = xyz_to_lab(&xyz);

        Self {
            lightness: lab[0],
            a_axis: lab[1],
            b_axis: lab[2],
            alpha,
        }
    }

    /// Create a new `LabRgba` instance from Lab components.
    ///
    /// # Panics
    ///
    /// Panics if any component is not in [0, 1].
    #[inline]
    pub fn from_lab(lightness: T, a_axis: T, b_axis: T, alpha: T) -> Self {
        assert!(
            lightness >= T::zero() && lightness <= T::one(),
            "Lightness component must be between 0 and 1."
        );
        assert!(
            a_axis >= T::zero() && a_axis <= T::one(),
            "A-axis component must be between 0 and 1."
        );
        assert!(
            b_axis >= T::zero() && b_axis <= T::one(),
            "B-axis component must be between 0 and 1."
        );
        assert!(
            alpha >= T::zero() && alpha <= T::one(),
            "Alpha component must be between 0 and 1."
        );
        Self {
            lightness,
            a_axis,
            b_axis,
            alpha,
        }
    }

    /// Get the red component.
    #[inline]
    pub fn red(&self) -> T {
        let rgba = self.rgb_components();
        rgba[0]
    }

    /// Get the green component.
    #[inline]
    pub fn green(&self) -> T {
        let rgba = self.rgb_components();
        rgba[1]
    }

    /// Get the blue component.
    #[inline]
    pub fn blue(&self) -> T {
        let rgba = self.rgb_components();
        rgba[2]
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

        // Get current RGB values
        let rgb = self.rgb_components();

        // Update with new red value
        let new_rgb = [red, rgb[1], rgb[2]];

        // Convert back to Lab
        let xyz = rgb_to_xyz_components(&new_rgb);
        let lab = xyz_to_lab(&xyz);

        self.lightness = lab[0];
        self.a_axis = lab[1];
        self.b_axis = lab[2];
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

        // Get current RGB values
        let rgb = self.rgb_components();

        // Update with new green value
        let new_rgb = [rgb[0], green, rgb[2]];

        // Convert back to Lab
        let xyz = rgb_to_xyz_components(&new_rgb);
        let lab = xyz_to_lab(&xyz);

        self.lightness = lab[0];
        self.a_axis = lab[1];
        self.b_axis = lab[2];
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

        // Get current RGB values
        let rgb = self.rgb_components();

        // Update with new blue value
        let new_rgb = [rgb[0], rgb[1], blue];

        // Convert back to Lab
        let xyz = rgb_to_xyz_components(&new_rgb);
        let lab = xyz_to_lab(&xyz);

        self.lightness = lab[0];
        self.a_axis = lab[1];
        self.b_axis = lab[2];
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

    /// Convert to `Grey`.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    pub fn to_grey(&self) -> Grey<T> {
        let [red, green, blue] = self.rgb_components();
        Grey::new((red + green + blue) / T::from(3).unwrap())
    }

    /// Convert to `GreyAlpha`.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    pub fn to_grey_alpha(&self) -> GreyAlpha<T> {
        let [red, green, blue] = self.rgb_components();
        GreyAlpha::new((red + green + blue) / T::from(3).unwrap(), self.alpha)
    }

    /// Convert to `Rgb`.
    #[inline]
    pub fn to_rgb(&self) -> Rgb<T> {
        let [red, green, blue] = self.rgb_components();
        Rgb::new(red, green, blue)
    }

    /// Convert to `Rgba`.
    #[inline]
    pub fn to_rgba(&self) -> Rgba<T> {
        let [red, green, blue] = self.rgb_components();
        Rgba::new(red, green, blue, self.alpha)
    }

    /// Convert to `LabRgb`.
    #[inline]
    pub fn to_lab_rgb(&self) -> LabRgb<T> {
        LabRgb::from_lab(self.lightness, self.a_axis, self.b_axis)
    }
}

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

impl<T: Display + AddAssign + Float> PartialEq for LabRgba<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        // Compare Lab components directly
        (self.lightness - other.lightness).abs() <= Self::tolerance()
            && (self.a_axis - other.a_axis).abs() <= Self::tolerance()
            && (self.b_axis - other.b_axis).abs() <= Self::tolerance()
            && (self.alpha - other.alpha).abs() <= Self::tolerance()
    }
}

impl<T> FromStr for LabRgba<T>
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
