//! sRGB colour representation.

use num_traits::Float;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    config::PRINT_BLOCK,
    error::{ChromaticError, Result},
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
        Ok(if linear <= T::from(0.003_130_8).unwrap() {
            T::from(12.92).unwrap() * linear
        } else {
            T::from(1.055).unwrap() * linear.powf(T::from(1.0 / 2.4).unwrap()) - T::from(0.055).unwrap()
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
    pub fn gamma_decode(srgb: T) -> T {
        if srgb <= T::from(0.04045).unwrap() {
            srgb / T::from(12.92).unwrap()
        } else {
            ((srgb + T::from(0.055).unwrap()) / T::from(1.055).unwrap()).powf(T::from(2.4).unwrap())
        }
    }
}

impl<T: Float + Send + Sync> Colour<T, 3> for Srgb<T> {
    fn from_hex(hex: &str) -> Result<Self> {
        let components = hex
            .trim()
            .strip_prefix('#')
            .ok_or_else(|| ChromaticError::ColourParsing("Missing '#' prefix".to_string()))?;
        let mut chars = components.chars();
        let (red, green, blue) = match components.len() {
            // Short form: #RGB
            3 => {
                let r_digit = chars.next().unwrap();
                let g_digit = chars.next().unwrap();
                let b_digit = chars.next().unwrap();

                let red = u8::from_str_radix(&r_digit.to_string(), 16)
                    .map_err(|e| ChromaticError::ColourParsing(format!("Invalid hex digit: {e}")))?;
                let green = u8::from_str_radix(&g_digit.to_string(), 16)
                    .map_err(|e| ChromaticError::ColourParsing(format!("Invalid hex digit: {e}")))?;
                let blue = u8::from_str_radix(&b_digit.to_string(), 16)
                    .map_err(|e| ChromaticError::ColourParsing(format!("Invalid hex digit: {e}")))?;

                // Expand short form (e.g., #F00 becomes #FF0000)
                let scaled_red = T::from(red).ok_or_else(|| ChromaticError::Math("Failed to convert red value".to_string()))?
                    * T::from(17).unwrap()
                    / T::from(255).unwrap();
                let scaled_green = T::from(green)
                    .ok_or_else(|| ChromaticError::Math("Failed to convert green value".to_string()))?
                    * T::from(17).unwrap()
                    / T::from(255).unwrap();
                let scaled_blue = T::from(blue)
                    .ok_or_else(|| ChromaticError::Math("Failed to convert blue value".to_string()))?
                    * T::from(17).unwrap()
                    / T::from(255).unwrap();

                (scaled_red, scaled_green, scaled_blue)
            }
            // Long form: #RRGGBB
            6 => {
                let r1 = chars.next().unwrap().to_string();
                let r2 = chars.next().unwrap().to_string();
                let g1 = chars.next().unwrap().to_string();
                let g2 = chars.next().unwrap().to_string();
                let b1 = chars.next().unwrap().to_string();
                let b2 = chars.next().unwrap().to_string();

                let red = u8::from_str_radix(&format!("{r1}{r2}"), 16)
                    .map_err(|e| ChromaticError::ColourParsing(format!("Invalid hex red: {e}")))?;
                let green = u8::from_str_radix(&format!("{g1}{g2}"), 16)
                    .map_err(|e| ChromaticError::ColourParsing(format!("Invalid hex green: {e}")))?;
                let blue = u8::from_str_radix(&format!("{b1}{b2}"), 16)
                    .map_err(|e| ChromaticError::ColourParsing(format!("Invalid hex blue: {e}")))?;

                let scaled_red = T::from(red).ok_or_else(|| ChromaticError::Math("Failed to convert red value".to_string()))?
                    / T::from(255).unwrap();
                let scaled_green = T::from(green)
                    .ok_or_else(|| ChromaticError::Math("Failed to convert green value".to_string()))?
                    / T::from(255).unwrap();
                let scaled_blue = T::from(blue)
                    .ok_or_else(|| ChromaticError::Math("Failed to convert blue value".to_string()))?
                    / T::from(255).unwrap();

                (scaled_red, scaled_green, scaled_blue)
            }
            _ => return Err(ChromaticError::ColourParsing("Invalid hex format".to_string())),
        };
        Self::new(red, green, blue)
    }

    fn to_hex(&self) -> Result<String> {
        let max = T::from(255_u8).unwrap();
        let red = (self.red * max).round().to_u8().unwrap();
        let green = (self.green * max).round().to_u8().unwrap();
        let blue = (self.blue * max).round().to_u8().unwrap();
        Ok(format!("#{red:02X}{green:02X}{blue:02X}"))
    }

    fn from_bytes(bytes: [u8; 3]) -> Result<Self> {
        let max = T::from(255_u8).unwrap();
        let red = T::from(bytes[0]).unwrap() / max;
        let green = T::from(bytes[1]).unwrap() / max;
        let blue = T::from(bytes[2]).unwrap() / max;
        Self::new(red, green, blue)
    }

    fn to_bytes(self) -> Result<[u8; 3]> {
        let max = T::from(255_u8).unwrap();
        let red = (self.red * max).round().to_u8().unwrap();
        let green = (self.green * max).round().to_u8().unwrap();
        let blue = (self.blue * max).round().to_u8().unwrap();
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
        let r_linear = Self::gamma_decode(self.red);
        let g_linear = Self::gamma_decode(self.green);
        let b_linear = Self::gamma_decode(self.blue);

        let y_linear = r_linear * T::from(0.212_672_9).unwrap()
            + g_linear * T::from(0.715_152_2).unwrap()
            + b_linear * T::from(0.072_175_0).unwrap();

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
        let r_linear = Self::gamma_decode(self.red);
        let g_linear = Self::gamma_decode(self.green);
        let b_linear = Self::gamma_decode(self.blue);

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
        let max = T::from(255_i32).unwrap();
        let red = (self.red * max).round().to_u8().unwrap();
        let green = (self.green * max).round().to_u8().unwrap();
        let blue = (self.blue * max).round().to_u8().unwrap();
        write!(fmt, "\x1b[38;2;{red};{green};{blue}m{PRINT_BLOCK}\x1b[0m")
    }
}
