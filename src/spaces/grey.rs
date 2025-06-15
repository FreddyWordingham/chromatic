//! Monochrome colour representation.

use num_traits::Float;
use std::{
    any::type_name,
    fmt::{Display, Formatter, Result as FmtResult},
};

use crate::{
    config::PRINT_BLOCK,
    error::{ChromaticError, NumericError, Result, safe_constant},
    spaces::{GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz, XyzAlpha},
    traits::{Colour, Convert},
};

/// Monochrome colour.
#[derive(Debug, Clone, Copy)]
pub struct Grey<T: Float + Send + Sync> {
    /// Grey component.
    grey: T,
}

impl<T: Float + Send + Sync> Grey<T> {
    /// Create a new `Grey` instance with validation.
    ///
    /// # Errors
    ///
    /// Returns an error if the grey value is outside the range [0, 1].
    pub fn new(grey: T) -> Result<Self> {
        Self::validate_grey(grey)?;
        Ok(Self { grey })
    }

    /// Validate grey component is in range [0, 1].
    fn validate_grey(grey: T) -> Result<()> {
        if grey < T::zero() || grey > T::one() {
            return Err(ChromaticError::InvalidColour(format!(
                "Grey component ({}) must be between 0 and 1",
                grey.to_f64().unwrap_or(f64::NAN)
            )));
        }
        Ok(())
    }

    /// Get the `grey` component.
    pub const fn grey(&self) -> T {
        self.grey
    }

    /// Set the `grey` component with validation.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 1].
    pub fn set_grey(&mut self, grey: T) -> Result<()> {
        Self::validate_grey(grey)?;
        self.grey = grey;
        Ok(())
    }
}

impl<T: Float + Send + Sync> Colour<T, 1> for Grey<T> {
    fn from_hex(hex: &str) -> Result<Self> {
        let components = hex
            .trim()
            .strip_prefix('#')
            .ok_or_else(|| ChromaticError::ColourParsing("Missing '#' prefix".to_string()))?;
        let grey = match components.len() {
            // Short form: #G
            1 => {
                let value = u8::from_str_radix(components, 16)
                    .map_err(|err| ChromaticError::ColourParsing(format!("Invalid hex digit: {err}")))?;
                // Expand short form (e.g., #F becomes #FF)
                T::from(value).ok_or_else(|| ChromaticError::Math("Failed to convert grey value".to_string()))?
                    * safe_constant(17.0)?
                    / safe_constant(255.0)?
            }
            // Long form: #GG
            2 => {
                let value = u8::from_str_radix(components, 16)
                    .map_err(|err| ChromaticError::ColourParsing(format!("Invalid hex value: {err}")))?;
                T::from(value).ok_or_else(|| ChromaticError::Math("Failed to convert grey value".to_string()))?
                    / safe_constant(255.0)?
            }
            _ => return Err(ChromaticError::ColourParsing("Invalid hex format".to_string())),
        };
        Self::new(grey)
    }

    fn to_hex(&self) -> Result<String> {
        let max: T = safe_constant::<i32, T>(255_i32)?;
        let scaled = (self.grey * max).round();
        let grey = scaled.to_u8().ok_or_else(|| NumericError::TypeConversionFailed {
            from: type_name::<T>().to_string(),
            to: "u8".to_string(),
            reason: format!(
                "Grey value {} is outside u8 range [0, 255]",
                scaled.to_f64().unwrap_or(f64::NAN)
            ),
        })?;
        Ok(format!("#{grey:02X}"))
    }
    fn from_bytes(bytes: [u8; 1]) -> Result<Self> {
        let max = safe_constant::<u8, T>(255_u8)?;
        let value = safe_constant::<u8, T>(bytes[0])? / max;
        Self::new(value)
    }

    fn to_bytes(self) -> Result<[u8; 1]> {
        let max: T = safe_constant::<u8, T>(255_u8)?;
        let scaled = (self.grey * max).round();
        let value = scaled.to_u8().ok_or_else(|| NumericError::TypeConversionFailed {
            from: type_name::<T>().to_string(),
            to: "u8".to_string(),
            reason: format!(
                "Grey value {} is outside u8 range [0, 255]",
                scaled.to_f64().unwrap_or(f64::NAN)
            ),
        })?;
        Ok([value])
    }

    fn lerp(lhs: &Self, rhs: &Self, t: T) -> Result<Self> {
        if t < T::zero() || t > T::one() {
            return Err(ChromaticError::Interpolation(format!(
                "Interpolation factor ({}) must be between 0 and 1",
                t.to_f64().unwrap_or(f64::NAN)
            )));
        }

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
        let max = safe_constant(255_i32)?;
        let value = (self.grey * max).round().to_u8().unwrap();
        write!(fmt, "\x1b[38;2;{value};{value};{value}m{PRINT_BLOCK}\x1b[0m")
    }
}
