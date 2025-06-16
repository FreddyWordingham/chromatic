//! sRGB colour representation.

use num_traits::Float;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    config::PRINT_BLOCK,
    error::{
        ColourParsingError, Result, component_to_u8, format_terminal_color, parse_hex_component, safe_constant,
        u8_to_component, validate_interpolation_factor, validate_unit_component,
    },
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
    ///
    /// # Arguments
    ///
    /// * `red` - The red component, must be in range [0, 1]
    /// * `green` - The green component, must be in range [0, 1]
    /// * `blue` - The blue component, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if any component is outside the range [0, 1].
    pub fn new(red: T, green: T, blue: T) -> Result<Self> {
        validate_unit_component(red, "red")?;
        validate_unit_component(green, "green")?;
        validate_unit_component(blue, "blue")?;

        Ok(Self { red, green, blue })
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
    ///
    /// # Arguments
    ///
    /// * `red` - The new red value, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 1].
    pub fn set_red(&mut self, red: T) -> Result<()> {
        validate_unit_component(red, "red")?;
        self.red = red;
        Ok(())
    }

    /// Set the `green` component.
    ///
    /// # Arguments
    ///
    /// * `green` - The new green value, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 1].
    pub fn set_green(&mut self, green: T) -> Result<()> {
        validate_unit_component(green, "green")?;
        self.green = green;
        Ok(())
    }

    /// Set the `blue` component.
    ///
    /// # Arguments
    ///
    /// * `blue` - The new blue value, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 1].
    pub fn set_blue(&mut self, blue: T) -> Result<()> {
        validate_unit_component(blue, "blue")?;
        self.blue = blue;
        Ok(())
    }

    /// Set all components at once with validation.
    ///
    /// # Arguments
    ///
    /// * `red` - The red component, must be in range [0, 1]
    /// * `green` - The green component, must be in range [0, 1]
    /// * `blue` - The blue component, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if any component is outside the range [0, 1].
    pub fn set_components(&mut self, red: T, green: T, blue: T) -> Result<()> {
        validate_unit_component(red, "red")?;
        validate_unit_component(green, "green")?;
        validate_unit_component(blue, "blue")?;

        self.red = red;
        self.green = green;
        self.blue = blue;
        Ok(())
    }

    /// Apply the standard sRGB gamma encoding to a linear component.
    ///
    /// This converts a linear RGB value to an sRGB value using the standard
    /// piecewise encoding function specified in the sRGB standard.
    ///
    /// # Arguments
    ///
    /// * `linear` - Linear RGB component value in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if mathematical operations fail during encoding.
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
    /// # Arguments
    ///
    /// * `srgb` - sRGB component value in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if mathematical operations fail during decoding.
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
        let hex = hex.trim();

        // Check for # prefix
        let components = hex
            .strip_prefix('#')
            .ok_or_else(|| ColourParsingError::MissingHexPrefix(hex.to_string()))?;

        let (red, green, blue) = match components.len() {
            // Short form: #RGB
            3 => {
                let red = parse_hex_component(&components[0..1], "red")?;
                let green = parse_hex_component(&components[1..2], "green")?;
                let blue = parse_hex_component(&components[2..3], "blue")?;

                // Expand short form (e.g., #F00 becomes #FF0000)
                let scale = safe_constant(255.0)?;
                let expanded_red = red * 17;
                let expanded_green = green * 17;
                let expanded_blue = blue * 17;

                (
                    u8_to_component(expanded_red, scale)?,
                    u8_to_component(expanded_green, scale)?,
                    u8_to_component(expanded_blue, scale)?,
                )
            }
            // Long form: #RRGGBB
            6 => {
                let red = parse_hex_component(&components[0..2], "red")?;
                let green = parse_hex_component(&components[2..4], "green")?;
                let blue = parse_hex_component(&components[4..6], "blue")?;

                let scale = safe_constant(255.0)?;
                (
                    u8_to_component(red, scale)?,
                    u8_to_component(green, scale)?,
                    u8_to_component(blue, scale)?,
                )
            }
            _ => {
                return Err(ColourParsingError::InvalidHexLength {
                    actual: components.len(),
                }
                .into());
            }
        };

        Self::new(red, green, blue)
    }

    fn to_hex(&self) -> Result<String> {
        let scale = safe_constant(255.0)?;
        let red = component_to_u8(self.red, "red", scale)?;
        let green = component_to_u8(self.green, "green", scale)?;
        let blue = component_to_u8(self.blue, "blue", scale)?;

        Ok(format!("#{red:02X}{green:02X}{blue:02X}"))
    }

    fn from_bytes(bytes: [u8; 3]) -> Result<Self> {
        let scale = safe_constant(255.0)?;
        let red = u8_to_component(bytes[0], scale)?;
        let green = u8_to_component(bytes[1], scale)?;
        let blue = u8_to_component(bytes[2], scale)?;
        Self::new(red, green, blue)
    }

    fn to_bytes(self) -> Result<[u8; 3]> {
        let scale = safe_constant(255.0)?;
        let red = component_to_u8(self.red, "red", scale)?;
        let green = component_to_u8(self.green, "green", scale)?;
        let blue = component_to_u8(self.blue, "blue", scale)?;

        Ok([red, green, blue])
    }

    /// Linear interpolate between two sRGB colours.
    /// Note: This performs interpolation in sRGB space, which is not perceptually
    /// uniform. For perceptually uniform interpolation, consider converting to Lab
    /// or another perceptually uniform color space.
    fn lerp(lhs: &Self, rhs: &Self, t: T) -> Result<Self> {
        validate_interpolation_factor(t)?;

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
        let color_string = format_terminal_color(self.red, self.green, self.blue, PRINT_BLOCK)?;
        write!(fmt, "{color_string}")
    }
}
