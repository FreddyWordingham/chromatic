//! HSL (Hue, Saturation, Lightness) colour representation.

use num_traits::Float;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    config::PRINT_BLOCK,
    error::{ChromaticError, Result, safe_constant},
    spaces::{Grey, GreyAlpha, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz, XyzAlpha},
    traits::{Colour, Convert},
};

/// HSL colour representation.
#[derive(Debug, Clone, Copy)]
pub struct Hsl<T: Float + Send + Sync> {
    /// Hue component in degrees [0, 360).
    hue: T,
    /// Saturation component [0, 1].
    saturation: T,
    /// Lightness component [0, 1].
    lightness: T,
}

impl<T: Float + Send + Sync> Hsl<T> {
    /// Create a new `Hsl` instance with validation.
    ///
    /// # Errors
    ///
    /// Returns an error if saturation or lightness are outside [0, 1],
    /// or if hue normalization fails.
    pub fn new(hue: T, saturation: T, lightness: T) -> Result<Self> {
        let normalized_hue = Self::normalize_hue(hue)?;
        Self::validate_saturation(saturation)?;
        Self::validate_lightness(lightness)?;

        Ok(Self {
            hue: normalized_hue,
            saturation,
            lightness,
        })
    }

    /// Normalize hue to be within [0, 360) range.
    fn normalize_hue(mut hue: T) -> Result<T> {
        let f360 = T::from(360.0).ok_or_else(|| ChromaticError::Math("Failed to convert 360.0 to target type".to_string()))?;

        // Handle potential infinite loops by limiting iterations
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 1000;

        while hue >= f360 && iterations < MAX_ITERATIONS {
            hue = hue - f360;
            iterations += 1;
        }

        iterations = 0;
        while hue < T::zero() && iterations < MAX_ITERATIONS {
            hue = hue + f360;
            iterations += 1;
        }

        if iterations >= MAX_ITERATIONS {
            return Err(ChromaticError::Math(format!(
                "Hue normalization failed: value too large ({})",
                hue.to_f64().unwrap_or(f64::NAN)
            )));
        }

        Ok(hue)
    }

    /// Validate saturation is in range [0, 1].
    fn validate_saturation(saturation: T) -> Result<()> {
        if saturation < T::zero() || saturation > T::one() {
            return Err(ChromaticError::InvalidColour(format!(
                "Saturation ({}) must be between 0 and 1",
                saturation.to_f64().unwrap_or(f64::NAN)
            )));
        }
        Ok(())
    }

    /// Validate lightness is in range [0, 1].
    fn validate_lightness(lightness: T) -> Result<()> {
        if lightness < T::zero() || lightness > T::one() {
            return Err(ChromaticError::InvalidColour(format!(
                "Lightness ({}) must be between 0 and 1",
                lightness.to_f64().unwrap_or(f64::NAN)
            )));
        }
        Ok(())
    }

    /// Get the `hue` component.
    pub const fn hue(&self) -> T {
        self.hue
    }

    /// Get the `saturation` component.
    pub const fn saturation(&self) -> T {
        self.saturation
    }

    /// Get the `lightness` component.
    pub const fn lightness(&self) -> T {
        self.lightness
    }

    /// Set the `hue` component with validation.
    pub fn set_hue(&mut self, hue: T) -> Result<()> {
        self.hue = Self::normalize_hue(hue)?;
        Ok(())
    }

    /// Set the `saturation` component with validation.
    pub fn set_saturation(&mut self, saturation: T) -> Result<()> {
        Self::validate_saturation(saturation)?;
        self.saturation = saturation;
        Ok(())
    }

    /// Set the `lightness` component with validation.
    pub fn set_lightness(&mut self, lightness: T) -> Result<()> {
        Self::validate_lightness(lightness)?;
        self.lightness = lightness;
        Ok(())
    }
}

impl<T: Float + Send + Sync> Colour<T, 3> for Hsl<T> {
    fn from_hex(hex: &str) -> Result<Self> {
        Rgb::from_hex(hex)?.to_hsl()
    }

    fn to_hex(&self) -> Result<String> {
        self.to_rgb()?.to_hex()
    }

    fn from_bytes(bytes: [u8; 3]) -> Result<Self> {
        Rgb::from_bytes(bytes)?.to_hsl()
    }

    fn to_bytes(self) -> Result<[u8; 3]> {
        self.to_rgb()?.to_bytes()
    }

    /// Linear interpolate between two HSL colours.
    /// This uses the shortest path around the hue circle for interpolation.
    fn lerp(lhs: &Self, rhs: &Self, t: T) -> Result<Self> {
        if t < T::zero() || t > T::one() {
            return Err(ChromaticError::Interpolation(format!(
                "Interpolation factor ({}) must be between 0 and 1",
                t.to_f64().unwrap_or(f64::NAN)
            )));
        }

        // For hue, we need special handling to ensure we take the shortest path around the color wheel
        let mut hue_diff = rhs.hue - lhs.hue;

        // Constants
        let f180 = safe_constant::<u32, T>(180)?;
        let f360 = safe_constant::<u32, T>(360)?;

        // If the difference is greater than 180 degrees, it's shorter to go the other way around the color wheel
        if hue_diff > f180 {
            hue_diff = hue_diff - f360;
        } else if hue_diff < -f180 {
            hue_diff = hue_diff + f360;
        }

        // Calculate the interpolated hue and ensure it stays in [0, 360] range
        let mut hue = lhs.hue + t * hue_diff;
        if hue < T::zero() {
            hue = hue + f360;
        } else if hue > f360 {
            hue = hue - f360;
        }

        // Linear interpolation for saturation and lightness
        let saturation = lhs.saturation * (T::one() - t) + rhs.saturation * t;
        let lightness = lhs.lightness * (T::one() - t) + rhs.lightness * t;

        Self::new(hue, saturation, lightness)
    }
}

impl<T: Float + Send + Sync> Convert<T> for Hsl<T> {
    fn to_grey(&self) -> Result<Grey<T>> {
        Grey::new(self.lightness)
    }

    fn to_grey_alpha(&self) -> Result<GreyAlpha<T>> {
        GreyAlpha::new(self.lightness, T::one())
    }

    fn to_hsl(&self) -> Result<Self> {
        Ok(*self)
    }

    fn to_hsl_alpha(&self) -> Result<HslAlpha<T>> {
        HslAlpha::new(self.hue, self.saturation, self.lightness, T::one())
    }

    fn to_hsv(&self) -> Result<Hsv<T>> {
        // v = L + S_l * min(L, 1-L)
        let delta = self.saturation * (T::one() - (safe_constant::<f64, T>(2.0)? * self.lightness - T::one()).abs())
            / safe_constant(2.0)?;
        let mut v = self.lightness + delta;
        // clamp v to [0,1]
        if v < T::zero() {
            v = T::zero();
        } else if v > T::one() {
            v = T::one();
        }

        // s_v = delta / v (or 0 when v≈0)
        let s = if v.abs() <= T::epsilon() { T::zero() } else { delta / v };

        Hsv::new(self.hue, s, v)
    }

    fn to_hsv_alpha(&self) -> Result<HsvAlpha<T>> {
        let hsv = self.to_hsv()?;
        HsvAlpha::new(hsv.hue(), hsv.saturation(), hsv.value(), T::one())
    }

    fn to_lab(&self) -> Result<Lab<T>> {
        // Convert HSL to Lab via RGB and XYZ
        self.to_rgb()?.to_lab()
    }

    fn to_lab_alpha(&self) -> Result<LabAlpha<T>> {
        let lab = self.to_lab()?;
        LabAlpha::new(lab.lightness(), lab.a_star(), lab.b_star(), T::one())
    }

    fn to_rgb(&self) -> Result<Rgb<T>> {
        let lightness = self.lightness;
        let saturation = self.saturation;

        // If saturation is 0, the color is a shade of gray
        if saturation.abs() < T::epsilon() {
            return Rgb::new(lightness, lightness, lightness);
        }

        // Helper function for HSL to RGB conversion
        let hue_to_rgb = |p: T, q: T, mut t: T| -> Result<T> {
            let f6 = safe_constant(6.0)?;
            let f2 = safe_constant(2.0)?;
            let f3 = safe_constant(3.0)?;

            // Normalize t to be in range [0, 1]
            if t < T::zero() {
                t = t + T::one();
            }
            if t > T::one() {
                t = t - T::one();
            }

            if t < T::one() / f6 {
                return Ok(p + (q - p) * f6 * t);
            }
            if t < T::one() / f2 {
                return Ok(q);
            }
            if t < f2 / f3 {
                return Ok(p + (q - p) * (f2 / f3 - t) * f6);
            }
            Ok(p)
        };

        let q = if lightness < safe_constant(0.5)? {
            lightness * (T::one() + saturation)
        } else {
            lightness + saturation - lightness * saturation
        };

        let p = safe_constant::<f64, T>(2.0)? * lightness - q;
        let h = self.hue / safe_constant(360.0)?;

        let r = hue_to_rgb(p, q, h + safe_constant(1.0 / 3.0)?)?;
        let g = hue_to_rgb(p, q, h)?;
        let b = hue_to_rgb(p, q, h - safe_constant(1.0 / 3.0)?)?;

        Rgb::new(r, g, b)
    }

    fn to_rgb_alpha(&self) -> Result<RgbAlpha<T>> {
        let rgb = self.to_rgb()?;
        RgbAlpha::new(rgb.red(), rgb.green(), rgb.blue(), T::one())
    }

    fn to_srgb(&self) -> Result<Srgb<T>> {
        // Convert HSL to sRGB via linear RGB
        let rgb = self.to_rgb()?;
        let r_srgb = Srgb::gamma_encode(rgb.red())?;
        let g_srgb = Srgb::gamma_encode(rgb.green())?;
        let b_srgb = Srgb::gamma_encode(rgb.blue())?;

        Srgb::new(r_srgb, g_srgb, b_srgb)
    }

    fn to_srgb_alpha(&self) -> Result<SrgbAlpha<T>> {
        let srgb = self.to_srgb()?;
        SrgbAlpha::new(srgb.red(), srgb.green(), srgb.blue(), T::one())
    }

    fn to_xyz(&self) -> Result<Xyz<T>> {
        // Convert HSL to XYZ via linear RGB
        self.to_rgb()?.to_xyz()
    }

    fn to_xyz_alpha(&self) -> Result<XyzAlpha<T>> {
        let xyz = self.to_xyz()?;
        XyzAlpha::new(xyz.x(), xyz.y(), xyz.z(), T::one())
    }
}

impl<T: Float + Send + Sync> Display for Hsl<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        let rgb = self.to_rgb()?;
        let i255 = safe_constant::<i32, T>(255_i32)?;

        let red = (rgb.red() * i255).round().to_u8().ok_or(std::fmt::Error)?;
        let green = (rgb.green() * i255).round().to_u8().ok_or(std::fmt::Error)?;
        let blue = (rgb.blue() * i255).round().to_u8().ok_or(std::fmt::Error)?;

        write!(fmt, "\x1b[38;2;{red};{green};{blue}m{PRINT_BLOCK}\x1b[0m")
    }
}
