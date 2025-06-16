//! RGB colour representation.

use num_traits::Float;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    config::PRINT_BLOCK,
    error::{
        ColourParsingError, Result, component_to_u8, format_terminal_color, parse_hex_component, safe_constant,
        u8_to_component, validate_interpolation_factor, validate_unit_component,
    },
    spaces::{Grey, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, RgbAlpha, Srgb, SrgbAlpha, Xyz, XyzAlpha},
    traits::{Colour, Convert},
};

/// RGB colour representation.
#[derive(Debug, Clone, Copy)]
pub struct Rgb<T: Float + Send + Sync> {
    /// Red component in range [0, 1].
    red: T,
    /// Green component in range [0, 1].
    green: T,
    /// Blue component in range [0, 1].
    blue: T,
}

impl<T: Float + Send + Sync> Rgb<T> {
    /// Create a new `Rgb` instance with validation.
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

    /// Set the `red` component with validation.
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

    /// Set the `green` component with validation.
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

    /// Set the `blue` component with validation.
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
}

impl<T: Float + Send + Sync> Colour<T, 3> for Rgb<T> {
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

    /// Linear interpolate between two RGB colours.
    fn lerp(lhs: &Self, rhs: &Self, t: T) -> Result<Self> {
        validate_interpolation_factor(t)?;

        Self::new(
            lhs.red * (T::one() - t) + rhs.red * t,
            lhs.green * (T::one() - t) + rhs.green * t,
            lhs.blue * (T::one() - t) + rhs.blue * t,
        )
    }
}

impl<T: Float + Send + Sync> Convert<T> for Rgb<T> {
    fn to_grey(&self) -> Result<Grey<T>> {
        // Use simple average for RGB to greyscale conversion
        // For perceptually accurate conversion, use luminance weights via XYZ
        Grey::new((self.red + self.green + self.blue) / safe_constant(3.0)?)
    }

    fn to_grey_alpha(&self) -> Result<GreyAlpha<T>> {
        GreyAlpha::new((self.red + self.green + self.blue) / safe_constant(3.0)?, T::one())
    }

    #[expect(
        clippy::similar_names,
        reason = "f60 and f360 are used for hue calculations. Their names are similar only because they represent different numerical constants."
    )]
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
        let f60 = safe_constant(60.0)?;

        // Calculate hue
        let hue = if (r - max).abs() < T::epsilon() {
            // Red is max
            let segment = (g - b) / delta;
            let base_hue = segment * f60;
            // If green is less than blue, add 360 degrees
            if g < b { base_hue + f360 } else { base_hue }
        } else if (g - max).abs() < T::epsilon() {
            // Green is max
            let segment = (b - r) / delta;
            segment * f60 + safe_constant(120.0)?
        } else {
            // Blue is max
            let segment = (r - g) / delta;
            segment * f60 + safe_constant(240.0)?
        };

        // Normalize hue to [0, 360) range
        let normalized_hue = if hue >= f360 {
            hue - f360
        } else if hue < T::zero() {
            hue + f360
        } else {
            hue
        };

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

        // Handle achromatic case
        if delta.abs() < T::epsilon() {
            return Hsv::new(T::zero(), T::zero(), value);
        }

        let saturation = delta / max;

        let sixty = safe_constant(60.0)?;
        let two_sixty = safe_constant(120.0)?;
        let four_sixty = safe_constant(240.0)?;
        let three_sixty = safe_constant(360.0)?;

        let hue = if (max - r).abs() < T::epsilon() {
            // Red is max
            let segment = (g - b) / delta;
            let base_hue = segment * sixty;
            if base_hue < T::zero() {
                base_hue + three_sixty
            } else {
                base_hue
            }
        } else if (max - g).abs() < T::epsilon() {
            // Green is max
            let segment = (b - r) / delta;
            segment * sixty + two_sixty
        } else {
            // Blue is max
            let segment = (r - g) / delta;
            segment * sixty + four_sixty
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
        let color_string = format_terminal_color(self.red, self.green, self.blue, PRINT_BLOCK)?;
        write!(fmt, "{color_string}")
    }
}
