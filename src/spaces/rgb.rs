//! RGB colour representation.

use num_traits::Float;
use std::{
    any::type_name,
    fmt::{Display, Formatter, Result as FmtResult},
};

use crate::{
    config::PRINT_BLOCK,
    error::{ChromaticError, NumericError, Result, safe_constant},
    spaces::{Grey, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, RgbAlpha, Srgb, SrgbAlpha, Xyz, XyzAlpha},
    traits::{Colour, Convert},
};

/// RGB colour representation.
#[derive(Debug, Clone, Copy)]
pub struct Rgb<T: Float + Send + Sync> {
    /// Red component.
    red: T,
    /// Green component.
    green: T,
    /// Blue component.
    blue: T,
}

impl<T: Float + Send + Sync> Rgb<T> {
    /// Create a new `Rgb` instance with validation.
    ///
    /// # Errors
    ///
    /// Returns an error if any component is outside the range [0, 1].
    pub fn new(red: T, green: T, blue: T) -> Result<Self> {
        Self::validate_component(red, "red")?;
        Self::validate_component(green, "green")?;
        Self::validate_component(blue, "blue")?;

        Ok(Self { red, green, blue })
    }

    /// Validate a single component is in range [0, 1].
    fn validate_component(value: T, name: &str) -> Result<()> {
        if value < T::zero() || value > T::one() {
            return Err(ChromaticError::InvalidColour(format!(
                "{} component ({}) must be between 0 and 1",
                name,
                value.to_f64().unwrap_or(f64::NAN)
            )));
        }
        Ok(())
    }

    /// Get the `red` component.
    pub const fn red(&self) -> T {
        self.red
    }

    /// Get the `green` component.
    pub const fn green(&self) -> T {
        self.green
    }

    /// Get the `blue` component.
    pub const fn blue(&self) -> T {
        self.blue
    }

    /// Set the `red` component with validation.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 1].
    pub fn set_red(&mut self, red: T) -> Result<()> {
        Self::validate_component(red, "red")?;
        self.red = red;
        Ok(())
    }

    /// Set the `green` component with validation.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 1].
    pub fn set_green(&mut self, green: T) -> Result<()> {
        Self::validate_component(green, "green")?;
        self.green = green;
        Ok(())
    }

    /// Set the `blue` component with validation.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 1].
    pub fn set_blue(&mut self, blue: T) -> Result<()> {
        Self::validate_component(blue, "blue")?;
        self.blue = blue;
        Ok(())
    }

    /// Set all components at once with validation.
    ///
    /// # Errors
    ///
    /// Returns an error if any component is outside the range [0, 1].
    pub fn set_components(&mut self, red: T, green: T, blue: T) -> Result<()> {
        Self::validate_component(red, "red")?;
        Self::validate_component(green, "green")?;
        Self::validate_component(blue, "blue")?;

        self.red = red;
        self.green = green;
        self.blue = blue;
        Ok(())
    }
}

impl<T: Float + Send + Sync> Colour<T, 3> for Rgb<T> {
    fn from_hex(hex: &str) -> Result<Self> {
        let components = hex
            .trim()
            .strip_prefix('#')
            .ok_or_else(|| ChromaticError::ColourParsing("Missing '#' prefix".to_string()))?;

        let (red, green, blue) = match components.len() {
            // Short form: #RGB
            3 => {
                let red = u8::from_str_radix(&components[0..1], 16)
                    .map_err(|err| ChromaticError::ColourParsing(format!("Invalid hex digit: {err}")))?;
                let green = u8::from_str_radix(&components[1..2], 16)
                    .map_err(|err| ChromaticError::ColourParsing(format!("Invalid hex digit: {err}")))?;
                let blue = u8::from_str_radix(&components[2..3], 16)
                    .map_err(|err| ChromaticError::ColourParsing(format!("Invalid hex digit: {err}")))?;

                // Expand short form (e.g., #F00 becomes #FF0000)
                let scaled_red = T::from(red).ok_or_else(|| ChromaticError::Math("Failed to convert red value".to_string()))?
                    * safe_constant(17)?
                    / safe_constant(255)?;
                let scaled_green = T::from(green)
                    .ok_or_else(|| ChromaticError::Math("Failed to convert green value".to_string()))?
                    * safe_constant(17)?
                    / safe_constant(255)?;
                let scaled_blue = T::from(blue)
                    .ok_or_else(|| ChromaticError::Math("Failed to convert blue value".to_string()))?
                    * safe_constant(17)?
                    / safe_constant(255)?;

                (scaled_red, scaled_green, scaled_blue)
            }
            // Long form: #RRGGBB
            6 => {
                let red = u8::from_str_radix(&components[0..2], 16)
                    .map_err(|err| ChromaticError::ColourParsing(format!("Invalid hex red: {err}")))?;
                let green = u8::from_str_radix(&components[2..4], 16)
                    .map_err(|err| ChromaticError::ColourParsing(format!("Invalid hex green: {err}")))?;
                let blue = u8::from_str_radix(&components[4..6], 16)
                    .map_err(|err| ChromaticError::ColourParsing(format!("Invalid hex blue: {err}")))?;

                let scaled_red = T::from(red).ok_or_else(|| ChromaticError::Math("Failed to convert red value".to_string()))?
                    / safe_constant(255)?;
                let scaled_green = T::from(green)
                    .ok_or_else(|| ChromaticError::Math("Failed to convert green value".to_string()))?
                    / safe_constant(255)?;
                let scaled_blue = T::from(blue)
                    .ok_or_else(|| ChromaticError::Math("Failed to convert blue value".to_string()))?
                    / safe_constant(255)?;

                (scaled_red, scaled_green, scaled_blue)
            }
            _ => return Err(ChromaticError::ColourParsing("Invalid hex format".to_string())),
        };
        Self::new(red, green, blue)
    }

    fn to_hex(&self) -> Result<String> {
        let u255 = safe_constant(255_u8)?;
        let scaled_red = (self.red * u255).round();
        let scaled_green = (self.green * u255).round();
        let scaled_blue = (self.blue * u255).round();

        let red = scaled_red.to_u8().ok_or_else(|| NumericError::TypeConversionFailed {
            from: type_name::<T>().to_string(),
            to: "u8".to_string(),
            reason: format!(
                "Red value {} is outside u8 range [0, 255]",
                scaled_red.to_f64().unwrap_or(f64::NAN)
            ),
        })?;
        let green = scaled_green.to_u8().ok_or_else(|| NumericError::TypeConversionFailed {
            from: type_name::<T>().to_string(),
            to: "u8".to_string(),
            reason: format!(
                "Green value {} is outside u8 range [0, 255]",
                scaled_green.to_f64().unwrap_or(f64::NAN)
            ),
        })?;
        let blue = scaled_blue.to_u8().ok_or_else(|| NumericError::TypeConversionFailed {
            from: type_name::<T>().to_string(),
            to: "u8".to_string(),
            reason: format!(
                "Blue value {} is outside u8 range [0, 255]",
                scaled_blue.to_f64().unwrap_or(f64::NAN)
            ),
        })?;

        Ok(format!("#{red:02X}{green:02X}{blue:02X}"))
    }

    fn from_bytes(bytes: [u8; 3]) -> Result<Self> {
        let u255 = safe_constant::<u8, T>(255_u8)?;
        let red = safe_constant::<u8, T>(bytes[0])? / u255;
        let green = safe_constant::<u8, T>(bytes[1])? / u255;
        let blue = safe_constant::<u8, T>(bytes[2])? / u255;
        Self::new(red, green, blue)
    }

    fn to_bytes(self) -> Result<[u8; 3]> {
        let u255: T = safe_constant::<u8, T>(255_u8)?;
        let scaled_red = (self.red * u255).round();
        let scaled_green = (self.green * u255).round();
        let scaled_blue = (self.blue * u255).round();

        let red = scaled_red.to_u8().ok_or_else(|| NumericError::TypeConversionFailed {
            from: type_name::<T>().to_string(),
            to: "u8".to_string(),
            reason: format!(
                "Red value {} is outside u8 range [0, 255]",
                scaled_red.to_f64().unwrap_or(f64::NAN)
            ),
        })?;
        let green = scaled_green.to_u8().ok_or_else(|| NumericError::TypeConversionFailed {
            from: type_name::<T>().to_string(),
            to: "u8".to_string(),
            reason: format!(
                "Green value {} is outside u8 range [0, 255]",
                scaled_green.to_f64().unwrap_or(f64::NAN)
            ),
        })?;
        let blue = scaled_blue.to_u8().ok_or_else(|| NumericError::TypeConversionFailed {
            from: type_name::<T>().to_string(),
            to: "u8".to_string(),
            reason: format!(
                "Blue value {} is outside u8 range [0, 255]",
                scaled_blue.to_f64().unwrap_or(f64::NAN)
            ),
        })?;

        Ok([red, green, blue])
    }

    /// Linear interpolate between two RGB colours.
    fn lerp(lhs: &Self, rhs: &Self, t: T) -> Result<Self> {
        if t < T::zero() || t > T::one() {
            return Err(ChromaticError::Interpolation(format!(
                "Interpolation factor ({}) must be between 0 and 1",
                t.to_f64().unwrap_or(f64::NAN)
            )));
        }

        Self::new(
            lhs.red * (T::one() - t) + rhs.red * t,
            lhs.green * (T::one() - t) + rhs.green * t,
            lhs.blue * (T::one() - t) + rhs.blue * t,
        )
    }
}

impl<T: Float + Send + Sync> Convert<T> for Rgb<T> {
    fn to_grey(&self) -> Result<Grey<T>> {
        Grey::new((self.red + self.green + self.blue) / safe_constant(3.0)?)
    }

    fn to_grey_alpha(&self) -> Result<GreyAlpha<T>> {
        GreyAlpha::new((self.red + self.green + self.blue) / safe_constant(3.0)?, T::one())
    }

    fn to_hsl(&self) -> Result<Hsl<T>> {
        let r = self.red();
        let g = self.green();
        let b = self.blue();

        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let delta = max - min;

        // Calculate lightness
        let lightness = (max + min) / safe_constant(2.0)?;

        // If max equals min, the color is a shade of gray (no hue or saturation)
        if delta.abs() < T::epsilon() {
            return Hsl::new(T::zero(), T::zero(), lightness);
        }

        // Calculate saturation
        let saturation = if lightness <= safe_constant(0.5)? {
            delta / (max + min)
        } else {
            delta / (safe_constant::<f64, T>(2.0)? - max - min)
        };

        // Constants
        let f360 = safe_constant(360.0)?;

        // Calculate hue
        let hue = if r.abs_sub(max).abs() < T::epsilon() {
            // Red is max
            let segment = (g - b) / delta;
            let shift = T::zero();
            let segment_6 = segment / safe_constant(6.0)?;

            // If green is less than blue, add 1.0 (360 degrees)
            if g < b {
                f360 + shift + segment_6 * f360
            } else {
                shift + segment_6 * f360
            }
        } else if g.abs_sub(max).abs() < T::epsilon() {
            // Green is max
            let segment = (b - r) / delta;
            let shift = safe_constant::<f64, T>(1.0 / 3.0)?;
            let segment_6 = segment / safe_constant(6.0)?;

            shift + segment_6 * f360
        } else {
            // Blue is max
            let segment = (r - g) / delta;
            let shift = safe_constant::<f64, T>(2.0 / 3.0)?;
            let segment_6 = segment / safe_constant(6.0)?;

            shift + segment_6 * f360
        };

        // Make sure hue is in the range [0, 360)
        let mut normalized_hue = hue;
        while normalized_hue >= f360 {
            normalized_hue = normalized_hue - f360;
        }
        while normalized_hue < T::zero() {
            normalized_hue = normalized_hue + f360;
        }

        Hsl::new(normalized_hue, saturation, lightness)
    }

    fn to_hsl_alpha(&self) -> Result<HslAlpha<T>> {
        let hsl = self.to_hsl()?;
        HslAlpha::new(hsl.hue(), hsl.saturation(), hsl.lightness(), T::one())
    }

    fn to_hsv(&self) -> Result<Hsv<T>> {
        let r = self.red();
        let g = self.green();
        let b = self.blue();

        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let delta = max - min;

        let value = max;

        let zero = T::zero();
        let sixty = safe_constant(60.0)?;
        let two = safe_constant(2.0)?;
        let four = safe_constant(4.0)?;
        let six = safe_constant(6.0)?;

        let saturation = if max == zero { zero } else { delta / max };

        let hue = if delta == zero {
            zero
        } else if max == r {
            let mut h = (g - b) / delta;
            if h < zero {
                h = h + six;
            }
            h * sixty
        } else if max == g {
            ((b - r) / delta + two) * sixty
        } else {
            ((r - g) / delta + four) * sixty
        };

        Hsv::new(hue, saturation, value)
    }

    fn to_hsv_alpha(&self) -> Result<HsvAlpha<T>> {
        let hsv = self.to_hsv()?;
        HsvAlpha::new(hsv.hue(), hsv.saturation(), hsv.value(), T::one())
    }

    fn to_lab(&self) -> Result<Lab<T>> {
        // Convert RGB to Lab via XYZ
        self.to_xyz()?.to_lab()
    }

    fn to_lab_alpha(&self) -> Result<LabAlpha<T>> {
        let lab = self.to_lab()?;
        LabAlpha::new(lab.lightness(), lab.a_star(), lab.b_star(), T::one())
    }

    fn to_rgb(&self) -> Result<Self> {
        Ok(*self)
    }

    fn to_rgb_alpha(&self) -> Result<RgbAlpha<T>> {
        RgbAlpha::new(self.red(), self.green(), self.blue(), T::one())
    }

    fn to_srgb(&self) -> Result<Srgb<T>> {
        // Convert from linear RGB to gamma-encoded sRGB
        let r_srgb = Srgb::gamma_encode(self.red)?;
        let g_srgb = Srgb::gamma_encode(self.green)?;
        let b_srgb = Srgb::gamma_encode(self.blue)?;

        Srgb::new(r_srgb, g_srgb, b_srgb)
    }

    fn to_srgb_alpha(&self) -> Result<SrgbAlpha<T>> {
        let srgb = self.to_srgb()?;
        SrgbAlpha::new(srgb.red(), srgb.green(), srgb.blue(), T::one())
    }

    fn to_xyz(&self) -> Result<Xyz<T>> {
        // Convert linear RGB to XYZ using the standard sRGB transform matrix
        // This matrix is for D65 reference white
        let x = self.red() * safe_constant(0.412_456_4)?
            + self.green() * safe_constant(0.357_576_1)?
            + self.blue() * safe_constant(0.180_437_5)?;

        let y = self.red() * safe_constant(0.212_672_9)?
            + self.green() * safe_constant(0.715_152_2)?
            + self.blue() * safe_constant(0.072_175_0)?;

        let z = self.red() * safe_constant(0.019_333_9)?
            + self.green() * safe_constant(0.119_192_0)?
            + self.blue() * safe_constant(0.950_304_1)?;

        Xyz::new(x, y, z)
    }

    fn to_xyz_alpha(&self) -> Result<XyzAlpha<T>> {
        let xyz = self.to_xyz()?;
        XyzAlpha::new(xyz.x(), xyz.y(), xyz.z(), T::one())
    }
}

impl<T: Float + Send + Sync> Display for Rgb<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        let i255 = safe_constant::<i32, T>(255_i32)?;

        let red = (self.red * i255).round().to_u8().ok_or(std::fmt::Error)?;
        let green = (self.green * i255).round().to_u8().ok_or(std::fmt::Error)?;
        let blue = (self.blue * i255).round().to_u8().ok_or(std::fmt::Error)?;

        write!(fmt, "\x1b[38;2;{red};{green};{blue}m{PRINT_BLOCK}\x1b[0m")
    }
}
