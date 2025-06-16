//! HSL (Hue, Saturation, Lightness) colour representation.

use num_traits::Float;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    config::PRINT_BLOCK,
    error::{
        InterpolationError, Result, format_terminal_color, normalize_hue, safe_constant, validate_interpolation_factor,
        validate_unit_component,
    },
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
    /// # Arguments
    ///
    /// * `hue` - The hue in degrees, will be normalized to [0, 360)
    /// * `saturation` - The saturation, must be in range [0, 1]
    /// * `lightness` - The lightness, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if saturation or lightness are outside [0, 1],
    /// or if hue normalization fails.
    pub fn new(hue: T, saturation: T, lightness: T) -> Result<Self> {
        let normalized_hue = normalize_hue(hue)?;
        validate_unit_component(saturation, "saturation")?;
        validate_unit_component(lightness, "lightness")?;

        Ok(Self {
            hue: normalized_hue,
            saturation,
            lightness,
        })
    }

    /// Get the `hue` component in degrees [0, 360).
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
    ///
    /// # Arguments
    ///
    /// * `hue` - The new hue in degrees, will be normalized to [0, 360)
    ///
    /// # Errors
    ///
    /// Returns an error if hue normalization fails.
    pub fn set_hue(&mut self, hue: T) -> Result<()> {
        self.hue = normalize_hue(hue)?;
        Ok(())
    }

    /// Set the `saturation` component with validation.
    ///
    /// # Arguments
    ///
    /// * `saturation` - The new saturation, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 1].
    pub fn set_saturation(&mut self, saturation: T) -> Result<()> {
        validate_unit_component(saturation, "saturation")?;
        self.saturation = saturation;
        Ok(())
    }

    /// Set the `lightness` component with validation.
    ///
    /// # Arguments
    ///
    /// * `lightness` - The new lightness, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 1].
    pub fn set_lightness(&mut self, lightness: T) -> Result<()> {
        validate_unit_component(lightness, "lightness")?;
        self.lightness = lightness;
        Ok(())
    }

    /// Helper function for HSL to RGB conversion.
    fn hue_to_rgb(p: T, q: T, mut t: T) -> Result<T> {
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
        validate_interpolation_factor(t)?;

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
        } else if hue >= f360 {
            hue = hue - f360;
        }

        // Validate hue calculation didn't produce invalid values
        if !hue.is_finite() {
            return Err(InterpolationError::HueInterpolationError {
                hue1: lhs.hue.to_f64().unwrap_or(f64::NAN),
                hue2: rhs.hue.to_f64().unwrap_or(f64::NAN),
            }
            .into());
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
        // Convert HSL to HSV
        // v = L + S_l * min(L, 1-L)
        let min_lightness = (T::one() - self.lightness).min(self.lightness);
        let delta = self.saturation * min_lightness;
        let mut value = self.lightness + delta;

        // Clamp value to [0,1]
        if value < T::zero() {
            value = T::zero();
        } else if value > T::one() {
            value = T::one();
        }

        // s_v = 2 * delta / v (or 0 when v≈0)
        let saturation = if value.abs() <= T::epsilon() {
            T::zero()
        } else {
            safe_constant::<f64, T>(2.0)? * delta / value
        };

        Hsv::new(self.hue, saturation, value)
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

        let q = if lightness < safe_constant(0.5)? {
            lightness * (T::one() + saturation)
        } else {
            lightness + saturation - lightness * saturation
        };

        let p = safe_constant::<f64, T>(2.0)? * lightness - q;
        let h = self.hue / safe_constant(360.0)?;

        let r = Self::hue_to_rgb(p, q, h + safe_constant(1.0 / 3.0)?)?;
        let g = Self::hue_to_rgb(p, q, h)?;
        let b = Self::hue_to_rgb(p, q, h - safe_constant(1.0 / 3.0)?)?;

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
        let color_string = format_terminal_color(rgb.red(), rgb.green(), rgb.blue(), PRINT_BLOCK)?;
        write!(fmt, "{}", color_string)
    }
}
