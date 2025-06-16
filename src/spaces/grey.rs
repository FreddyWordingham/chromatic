//! Monochrome colour representation.

use num_traits::Float;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    config::PRINT_BLOCK,
    error::{
        ColourParsingError, Result, component_to_u8, format_terminal_color, parse_hex_component, safe_constant,
        u8_to_component, validate_interpolation_factor, validate_unit_component,
    },
    spaces::{GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz, XyzAlpha},
    traits::{Colour, Convert},
};

/// Monochrome colour.
#[derive(Debug, Clone, Copy)]
pub struct Grey<T: Float + Send + Sync> {
    /// Grey component in range [0, 1].
    grey: T,
}

impl<T: Float + Send + Sync> Grey<T> {
    /// Create a new `Grey` instance with validation.
    ///
    /// # Arguments
    ///
    /// * `grey` - The grey value, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if the grey value is outside the range [0, 1].
    pub fn new(grey: T) -> Result<Self> {
        validate_unit_component(grey, "grey")?;
        Ok(Self { grey })
    }

    /// Get the `grey` component.
    pub const fn grey(&self) -> T {
        self.grey
    }

    /// Set the `grey` component with validation.
    ///
    /// # Arguments
    ///
    /// * `grey` - The new grey value, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 1].
    pub fn set_grey(&mut self, grey: T) -> Result<()> {
        validate_unit_component(grey, "grey")?;
        self.grey = grey;
        Ok(())
    }
}

impl<T: Float + Send + Sync> Colour<T, 1> for Grey<T> {
    fn from_hex(hex: &str) -> Result<Self> {
        let hex = hex.trim();

        // Check for # prefix
        let components = hex
            .strip_prefix('#')
            .ok_or_else(|| ColourParsingError::MissingHexPrefix(hex.to_string()))?;

        let grey = match components.len() {
            // Short form: #G
            1 => {
                let value = parse_hex_component(components, "grey")?;
                // Expand short form (e.g., #F becomes #FF)
                let expanded = value * 17;
                u8_to_component(expanded, safe_constant(255.0)?)?
            }
            // Long form: #GG
            2 => {
                let value = parse_hex_component(components, "grey")?;
                u8_to_component(value, safe_constant(255.0)?)?
            }
            _ => {
                return Err(ColourParsingError::InvalidHexLength {
                    actual: components.len(),
                }
                .into());
            }
        };

        Self::new(grey)
    }

    fn to_hex(&self) -> Result<String> {
        let scale = safe_constant(255.0)?;
        let grey = component_to_u8(self.grey, "grey", scale)?;
        Ok(format!("#{grey:02X}"))
    }

    fn from_bytes(bytes: [u8; 1]) -> Result<Self> {
        let scale = safe_constant(255.0)?;
        let value = u8_to_component(bytes[0], scale)?;
        Self::new(value)
    }

    fn to_bytes(self) -> Result<[u8; 1]> {
        let scale = safe_constant(255.0)?;
        let value = component_to_u8(self.grey, "grey", scale)?;
        Ok([value])
    }

    fn lerp(lhs: &Self, rhs: &Self, t: T) -> Result<Self> {
        validate_interpolation_factor(t)?;
        Self::new(lhs.grey() * (T::one() - t) + rhs.grey() * t)
    }
}

impl<T: Float + Send + Sync> Convert<T> for Grey<T> {
    fn to_grey(&self) -> Result<Self> {
        Ok(*self)
    }

    fn to_grey_alpha(&self) -> Result<GreyAlpha<T>> {
        GreyAlpha::new(self.grey, T::one())
    }

    fn to_hsl(&self) -> Result<Hsl<T>> {
        // For greyscale, hue is undefined (0), saturation is 0, and lightness equals the grey value
        Hsl::new(T::zero(), T::zero(), self.grey)
    }

    fn to_hsl_alpha(&self) -> Result<HslAlpha<T>> {
        HslAlpha::new(T::zero(), T::zero(), self.grey, T::one())
    }

    fn to_hsv(&self) -> Result<Hsv<T>> {
        // For greyscale, hue is undefined (0), saturation is 0, and value equals the grey value
        Hsv::new(T::zero(), T::zero(), self.grey)
    }

    fn to_hsv_alpha(&self) -> Result<HsvAlpha<T>> {
        HsvAlpha::new(T::zero(), T::zero(), self.grey, T::one())
    }

    fn to_lab(&self) -> Result<Lab<T>> {
        // Convert Grey to Lab via XYZ
        self.to_xyz()?.to_lab()
    }

    fn to_lab_alpha(&self) -> Result<LabAlpha<T>> {
        let lab = self.to_lab()?;
        LabAlpha::new(lab.lightness(), lab.a_star(), lab.b_star(), T::one())
    }

    fn to_rgb(&self) -> Result<Rgb<T>> {
        Rgb::new(self.grey, self.grey, self.grey)
    }

    fn to_rgb_alpha(&self) -> Result<RgbAlpha<T>> {
        RgbAlpha::new(self.grey, self.grey, self.grey, T::one())
    }

    fn to_srgb(&self) -> Result<Srgb<T>> {
        let sg = Srgb::gamma_encode(self.grey)?;
        Srgb::new(sg, sg, sg)
    }

    fn to_srgb_alpha(&self) -> Result<SrgbAlpha<T>> {
        let sg = Srgb::gamma_encode(self.grey)?;
        SrgbAlpha::new(sg, sg, sg, T::one())
    }

    fn to_xyz(&self) -> Result<Xyz<T>> {
        // Grey in XYZ space with D65 reference white
        // For greyscale, X, Y, and Z values are proportional to the reference white
        // Y (luminance) equals grey value, and X and Z are scaled according to D65

        // Simplified approach: use the luminance (Y) value directly,
        // and scale X and Z based on D65 reference white
        let white = Xyz::<T>::d65_reference_white()?;

        // Scale all values by the grey value (luminance)
        let x = white.x() * self.grey();
        let y = self.grey(); // Y value is directly the grey value (luminance)
        let z = white.z() * self.grey();

        Xyz::new(x, y, z)
    }

    fn to_xyz_alpha(&self) -> Result<XyzAlpha<T>> {
        let xyz = self.to_xyz()?;
        XyzAlpha::new(xyz.x(), xyz.y(), xyz.z(), T::one())
    }
}

impl<T: Float + Send + Sync> Display for Grey<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        let color_string = format_terminal_color(self.grey, self.grey, self.grey, PRINT_BLOCK)?;
        write!(fmt, "{color_string}")
    }
}
