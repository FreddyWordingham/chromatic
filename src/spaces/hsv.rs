//! HSV (Hue, Saturation, Value) colour representation.

use num_traits::Float;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    config::PRINT_BLOCK,
    error::{ChromaticError, Result},
    spaces::{Grey, GreyAlpha, Hsl, HslAlpha, HsvAlpha, Lab, LabAlpha, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz, XyzAlpha},
    traits::{Colour, Convert},
};

/// HSV colour representation.
#[derive(Debug, Clone, Copy)]
pub struct Hsv<T: Float + Send + Sync> {
    /// Hue component in degrees [0, 360).
    hue: T,
    /// Saturation component [0, 1].
    saturation: T,
    /// Value component [0, 1].
    value: T,
}

impl<T: Float + Send + Sync> Hsv<T> {
    /// Create a new `Hsv` instance with validation.
    ///
    /// # Errors
    ///
    /// Returns an error if saturation or value are outside [0, 1],
    /// or if hue normalization fails.
    pub fn new(hue: T, saturation: T, value: T) -> Result<Self> {
        let normalized_hue = Self::normalize_hue(hue)?;
        Self::validate_saturation(saturation)?;
        Self::validate_value(value)?;

        Ok(Self {
            hue: normalized_hue,
            saturation,
            value,
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

    /// Validate value is in range [0, 1].
    fn validate_value(value: T) -> Result<()> {
        if value < T::zero() || value > T::one() {
            return Err(ChromaticError::InvalidColour(format!(
                "Value ({}) must be between 0 and 1",
                value.to_f64().unwrap_or(f64::NAN)
            )));
        }
        Ok(())
    }

    /// Get the `hue` component in degrees [0, 360).
    pub const fn hue(&self) -> T {
        self.hue
    }

    /// Get the `saturation` component.
    pub const fn saturation(&self) -> T {
        self.saturation
    }

    /// Get the `value` component.
    pub const fn value(&self) -> T {
        self.value
    }

    /// Set the `hue` component with validation.
    ///
    /// # Errors
    ///
    /// Returns an error if hue normalization fails.
    pub fn set_hue(&mut self, hue: T) -> Result<()> {
        self.hue = Self::normalize_hue(hue)?;
        Ok(())
    }

    /// Set the `saturation` component with validation.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 1].
    pub fn set_saturation(&mut self, saturation: T) -> Result<()> {
        Self::validate_saturation(saturation)?;
        self.saturation = saturation;
        Ok(())
    }

    /// Set the `value` component with validation.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 1].
    pub fn set_value(&mut self, value: T) -> Result<()> {
        Self::validate_value(value)?;
        self.value = value;
        Ok(())
    }

    /// Set all components at once with validation.
    ///
    /// # Errors
    ///
    /// Returns an error if any component is invalid.
    pub fn set_components(&mut self, hue: T, saturation: T, value: T) -> Result<()> {
        let normalized_hue = Self::normalize_hue(hue)?;
        Self::validate_saturation(saturation)?;
        Self::validate_value(value)?;

        self.hue = normalized_hue;
        self.saturation = saturation;
        self.value = value;
        Ok(())
    }
}
impl<T: Float + Send + Sync> Colour<T, 3> for Hsv<T> {
    fn from_hex(hex: &str) -> Result<Self> {
        Rgb::from_hex(hex)?.to_hsv()
    }

    fn to_hex(&self) -> Result<String> {
        self.to_rgb()?.to_hex()
    }

    fn from_bytes(bytes: [u8; 3]) -> Result<Self> {
        Rgb::from_bytes(bytes)?.to_hsv()
    }

    fn to_bytes(self) -> Result<[u8; 3]> {
        self.to_rgb()?.to_bytes()
    }

    /// Linear interpolate between two HSV colours.
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

        // If the difference is greater than 180 degrees, it's shorter to go the other way around the color wheel
        let f180 = T::from(180.0).unwrap_or_else(|| T::zero());
        let f360 = T::from(360.0).unwrap_or_else(|| T::one());

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

        // Linear interpolation for saturation and value
        let saturation = lhs.saturation * (T::one() - t) + rhs.saturation * t;
        let value = lhs.value * (T::one() - t) + rhs.value * t;

        Self::new(hue, saturation, value)
    }
}

impl<T: Float + Send + Sync> Convert<T> for Hsv<T> {
    fn to_grey(&self) -> Result<Grey<T>> {
        Grey::new(self.value)
    }

    fn to_grey_alpha(&self) -> Result<GreyAlpha<T>> {
        GreyAlpha::new(self.value, T::one())
    }

    fn to_hsl(&self) -> Result<Hsl<T>> {
        // Convert HSV to HSL
        // Hue remains the same
        let hue = self.hue;

        // Calculate lightness based on value and saturation
        let lightness = self.value * (T::one() - self.saturation / T::from(2.0).unwrap());

        // Calculate saturation for HSL
        let saturation = if lightness.abs() < T::epsilon() || (lightness - T::one()).abs() < T::epsilon() {
            // If lightness is 0 or 1, saturation is 0
            T::zero()
        } else {
            let min_val = lightness * T::from(2.0).unwrap() - T::one();
            let max_val = min_val + self.saturation * (T::one() - min_val.abs());

            // Calculate HSL saturation from min and max values
            (max_val - min_val) / (T::one() - (T::from(2.0).unwrap() * lightness - T::one()).abs())
        };

        Hsl::new(hue, saturation, lightness)
    }

    fn to_hsl_alpha(&self) -> Result<HslAlpha<T>> {
        let hsl = self.to_hsl()?;
        HslAlpha::new(hsl.hue(), hsl.saturation(), hsl.lightness(), T::one())
    }

    fn to_hsv(&self) -> Result<Self> {
        Ok(*self)
    }

    fn to_hsv_alpha(&self) -> Result<HsvAlpha<T>> {
        HsvAlpha::new(self.hue, self.saturation, self.value, T::one())
    }

    fn to_lab(&self) -> Result<Lab<T>> {
        // Convert HSV to Lab via XYZ
        self.to_xyz()?.to_lab()
    }

    fn to_lab_alpha(&self) -> Result<LabAlpha<T>> {
        let lab = self.to_lab()?;
        LabAlpha::new(lab.lightness(), lab.a_star(), lab.b_star(), T::one())
    }

    fn to_rgb(&self) -> Result<Rgb<T>> {
        let h = self.hue;
        let s = self.saturation;
        let v = self.value;

        let c = v * s;
        let h_prime = h / T::from(60.0).unwrap();
        let x = c * (T::one() - ((h_prime % T::from(2.0).unwrap()) - T::one()).abs());
        let m = v - c;

        let (r, g, b) = if h < T::from(60.0).unwrap() {
            (c, x, T::zero())
        } else if h < T::from(120.0).unwrap() {
            (x, c, T::zero())
        } else if h < T::from(180.0).unwrap() {
            (T::zero(), c, x)
        } else if h < T::from(240.0).unwrap() {
            (T::zero(), x, c)
        } else if h < T::from(300.0).unwrap() {
            (x, T::zero(), c)
        } else {
            (c, T::zero(), x)
        };

        Rgb::new(r + m, g + m, b + m)
    }

    fn to_rgb_alpha(&self) -> Result<RgbAlpha<T>> {
        let rgb = self.to_rgb()?;
        RgbAlpha::new(rgb.red(), rgb.green(), rgb.blue(), T::one())
    }

    fn to_srgb(&self) -> Result<Srgb<T>> {
        // Convert HSV to sRGB via linear RGB
        // First convert to linear RGB
        let rgb = self.to_rgb()?;

        // Then convert linear RGB to sRGB
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
        // Convert HSV to XYZ via linear RGB
        self.to_rgb()?.to_xyz()
    }

    fn to_xyz_alpha(&self) -> Result<XyzAlpha<T>> {
        let xyz = self.to_xyz()?;
        XyzAlpha::new(xyz.x(), xyz.y(), xyz.z(), T::one())
    }
}

impl<T: Float + Send + Sync> Display for Hsv<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        let rgb = self.to_rgb()?;
        let max = T::from(255_i32).unwrap();
        let red = (rgb.red() * max).round().to_u8().unwrap();
        let green = (rgb.green() * max).round().to_u8().unwrap();
        let blue = (rgb.blue() * max).round().to_u8().unwrap();
        write!(fmt, "\x1b[38;2;{red};{green};{blue}m{PRINT_BLOCK}\x1b[0m")
    }
}
