//! sRGB colour representation.

use num_traits::Float;
use std::{
    any::type_name,
    fmt::{Display, Formatter, Result as FmtResult},
};

use crate::{
    config::PRINT_BLOCK,
    error::{ChromaticError, NumericError, Result, safe_constant},
    spaces::{Grey, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, Rgb, RgbAlpha, SrgbAlpha, Xyz, XyzAlpha},
    traits::{Colour, Convert},
};

/// sRGB colour representation.
///
/// sRGB is a standard RGB color space widely used in digital displays, image formats, and web content.
/// It uses a specific non-linear gamma encoding to represent colors in a way that's perceptually more
/// uniform than linear RGB.
#[derive(Debug, Clone, Copy)]
pub struct Srgb<T: Float + Send + Sync> {
    /// Red component in range [0, 1].
    red: T,
    /// Green component in range [0, 1].
    green: T,
    /// Blue component in range [0, 1].
    blue: T,
}

impl<T: Float + Send + Sync> Srgb<T> {
    /// Create a new `Srgb` instance.
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

    /// Set the `red` component.
    pub fn set_red(&mut self, red: T) -> Result<()> {
        Self::validate_component(red, "red")?;
        self.red = red;
        Ok(())
    }

    /// Set the `green` component.
    pub fn set_green(&mut self, green: T) -> Result<()> {
        Self::validate_component(green, "green")?;
        self.green = green;
        Ok(())
    }

    /// Set the `blue` component.
    pub fn set_blue(&mut self, blue: T) -> Result<()> {
        Self::validate_component(blue, "blue")?;
        self.blue = blue;
        Ok(())
    }

    /// Apply the standard sRGB gamma encoding to a linear component.
    ///
    /// This converts a linear RGB value to an sRGB value using the standard
    /// piecewise encoding function specified in the sRGB standard.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    pub fn gamma_encode(linear: T) -> Result<T> {
        Ok(if linear <= safe_constant(0.003_130_8)? {
            safe_constant::<f64, T>(12.92)? * linear
        } else {
            safe_constant::<f64, T>(1.055)? * linear.powf(safe_constant(1.0 / 2.4)?) - safe_constant(0.055)?
        })
    }

    /// Apply the standard sRGB gamma decoding to an sRGB component.
    ///
    /// This converts an sRGB value to a linear RGB value using the standard
    /// piecewise decoding function specified in the sRGB standard.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    pub fn gamma_decode(srgb: T) -> Result<T> {
        Ok(if srgb <= safe_constant::<f64, T>(0.04045)? {
            srgb / safe_constant(12.92)?
        } else {
            ((srgb + safe_constant(0.055)?) / safe_constant(1.055)?).powf(safe_constant(2.4)?)
        })
    }
}

impl<T: Float + Send + Sync> Colour<T, 3> for Srgb<T> {
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

    /// Linear interpolate between two sRGB colours.
    /// Note: This performs interpolation in sRGB space, which is not perceptually
    /// uniform. For perceptually uniform interpolation, consider converting to Lab
    /// or another perceptually uniform color space.
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

impl<T: Float + Send + Sync> Convert<T> for Srgb<T> {
    fn to_grey(&self) -> Result<Grey<T>> {
        // For perceptually correct greyscale, use the luminance formula
        // Y = 0.2126*R + 0.7152*G + 0.0722*B (same as in XYZ conversion)
        // This applies to gamma-encoded (non-linear) sRGB values
        let r_linear = Self::gamma_decode(self.red)?;
        let g_linear = Self::gamma_decode(self.green)?;
        let b_linear = Self::gamma_decode(self.blue)?;

        let y_linear = r_linear * safe_constant(0.212_672_9)?
            + g_linear * safe_constant(0.715_152_2)?
            + b_linear * safe_constant(0.072_175_0)?;

        // Keep in linear space for Grey, as Grey is a linear space
        Grey::new(y_linear)
    }

    fn to_grey_alpha(&self) -> Result<GreyAlpha<T>> {
        // Use the luminance formula as in to_grey
        let grey = self.to_grey()?;
        GreyAlpha::new(grey.grey(), T::one())
    }

    fn to_hsl(&self) -> Result<Hsl<T>> {
        self.to_rgb()?.to_hsl()
    }

    fn to_hsl_alpha(&self) -> Result<HslAlpha<T>> {
        let hsl = self.to_hsl()?;
        HslAlpha::new(hsl.hue(), hsl.saturation(), hsl.lightness(), T::one())
    }

    fn to_hsv(&self) -> Result<Hsv<T>> {
        // Convert to linear RGB first for accurate HSV conversion
        self.to_rgb()?.to_hsv()
    }

    fn to_hsv_alpha(&self) -> Result<HsvAlpha<T>> {
        let hsv = self.to_hsv()?;
        HsvAlpha::new(hsv.hue(), hsv.saturation(), hsv.value(), T::one())
    }

    fn to_lab(&self) -> Result<Lab<T>> {
        // Convert to XYZ first, then to Lab
        self.to_xyz()?.to_lab()
    }

    fn to_lab_alpha(&self) -> Result<LabAlpha<T>> {
        let lab = self.to_lab()?;
        LabAlpha::new(lab.lightness(), lab.a_star(), lab.b_star(), T::one())
    }

    fn to_rgb(&self) -> Result<Rgb<T>> {
        // Convert from gamma-encoded sRGB to linear RGB
        let r_linear = Self::gamma_decode(self.red)?;
        let g_linear = Self::gamma_decode(self.green)?;
        let b_linear = Self::gamma_decode(self.blue)?;

        Rgb::new(r_linear, g_linear, b_linear)
    }

    fn to_rgb_alpha(&self) -> Result<RgbAlpha<T>> {
        let rgb = self.to_rgb()?;
        RgbAlpha::new(rgb.red(), rgb.green(), rgb.blue(), T::one())
    }

    fn to_srgb(&self) -> Result<Self> {
        Ok(*self)
    }

    fn to_srgb_alpha(&self) -> Result<SrgbAlpha<T>> {
        SrgbAlpha::new(self.red, self.green, self.blue, T::one())
    }

    fn to_xyz(&self) -> Result<Xyz<T>> {
        // Convert to linear RGB first, then to XYZ
        self.to_rgb()?.to_xyz()
    }

    fn to_xyz_alpha(&self) -> Result<XyzAlpha<T>> {
        let xyz = self.to_xyz()?;
        XyzAlpha::new(xyz.x(), xyz.y(), xyz.z(), T::one())
    }
}

impl<T: Float + Send + Sync> Display for Srgb<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        let i255 = safe_constant::<i32, T>(255_i32)?;

        let red = (self.red * i255).round().to_u8().ok_or(std::fmt::Error)?;
        let green = (self.green * i255).round().to_u8().ok_or(std::fmt::Error)?;
        let blue = (self.blue * i255).round().to_u8().ok_or(std::fmt::Error)?;

        write!(fmt, "\x1b[38;2;{red};{green};{blue}m{PRINT_BLOCK}\x1b[0m")
    }
}
