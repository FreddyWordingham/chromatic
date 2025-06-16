//! HSV (Hue, Saturation, Value) colour representation.

use num_traits::Float;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    config::PRINT_BLOCK,
    error::{
        InterpolationError, Result, format_terminal_color, normalize_hue, safe_constant, validate_interpolation_factor,
        validate_unit_component,
    },
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
    /// # Arguments
    ///
    /// * `hue` - The hue in degrees, will be normalized to [0, 360)
    /// * `saturation` - The saturation, must be in range [0, 1]
    /// * `value` - The value (brightness), must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if saturation or value are outside [0, 1],
    /// or if hue normalization fails.
    pub fn new(hue: T, saturation: T, value: T) -> Result<Self> {
        let normalized_hue = normalize_hue(hue)?;
        validate_unit_component(saturation, "saturation")?;
        validate_unit_component(value, "value")?;

        Ok(Self {
            hue: normalized_hue,
            saturation,
            value,
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

    /// Get the `value` component.
    pub const fn value(&self) -> T {
        self.value
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

    /// Set the `value` component with validation.
    ///
    /// # Arguments
    ///
    /// * `value` - The new value (brightness), must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 1].
    pub fn set_value(&mut self, value: T) -> Result<()> {
        validate_unit_component(value, "value")?;
        self.value = value;
        Ok(())
    }

    /// Set all components at once with validation.
    ///
    /// # Arguments
    ///
    /// * `hue` - The hue in degrees, will be normalized to [0, 360)
    /// * `saturation` - The saturation, must be in range [0, 1]
    /// * `value` - The value (brightness), must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if any component is invalid.
    pub fn set_components(&mut self, hue: T, saturation: T, value: T) -> Result<()> {
        let normalized_hue = normalize_hue(hue)?;
        validate_unit_component(saturation, "saturation")?;
        validate_unit_component(value, "value")?;

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
        validate_interpolation_factor(t)?;

        // For hue, we need special handling to ensure we take the shortest path around the color wheel
        let mut hue_diff = rhs.hue - lhs.hue;

        // If the difference is greater than 180 degrees, it's shorter to go the other way around the color wheel
        let f180 = safe_constant::<u32, T>(180)?;
        let f360 = safe_constant::<u32, T>(360)?;

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
            return Err(InterpolationError::HueInterpolation {
                hue1: lhs.hue.to_f64().unwrap_or(f64::NAN),
                hue2: rhs.hue.to_f64().unwrap_or(f64::NAN),
            }
            .into());
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

        // Calculate lightness: L = V * (2 - S) / 2
        let lightness = self.value * (safe_constant::<f64, T>(2.0)? - self.saturation) / safe_constant(2.0)?;

        // Calculate saturation for HSL
        let saturation = if lightness.abs() < T::epsilon() || (lightness - T::one()).abs() < T::epsilon() {
            // If lightness is 0 or 1, saturation is 0
            T::zero()
        } else {
            // S_hsl = V * S / (1 - |2L - 1|)
            let denominator = T::one() - (safe_constant::<f64, T>(2.0)? * lightness - T::one()).abs();
            if denominator.abs() < T::epsilon() {
                T::zero()
            } else {
                self.value * self.saturation / denominator
            }
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

        // Handle achromatic case (no hue)
        if s.abs() < T::epsilon() {
            return Rgb::new(v, v, v);
        }

        let c = v * s; // Chroma
        let h_prime = h / safe_constant(60.0)?;
        let x = c * (T::one() - ((h_prime % safe_constant(2.0)?) - T::one()).abs());
        let m = v - c;

        let (r_prime, g_prime, b_prime) = if h < safe_constant(60.0)? {
            (c, x, T::zero())
        } else if h < safe_constant(120.0)? {
            (x, c, T::zero())
        } else if h < safe_constant(180.0)? {
            (T::zero(), c, x)
        } else if h < safe_constant(240.0)? {
            (T::zero(), x, c)
        } else if h < safe_constant(300.0)? {
            (x, T::zero(), c)
        } else {
            (c, T::zero(), x)
        };

        Rgb::new(r_prime + m, g_prime + m, b_prime + m)
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
        let color_string = format_terminal_color(rgb.red(), rgb.green(), rgb.blue(), PRINT_BLOCK)?;
        write!(fmt, "{color_string}")
    }
}
