//! Lab colour representation.
//!
//! The Lab colour space (also known as CIELAB) is a colour space defined by the International
//! Commission on Illumination (CIE) in 1976. It expresses colour as three values:
//! - L* for perceptual lightness (0 to 100)
//! - a* from green (-) to red (+)
//! - b* from blue (-) to yellow (+)
//!
//! Lab is designed to be perceptually uniform, meaning a change of the same amount in a value
//! should produce a change of about the same visual importance.

use num_traits::Float;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    config::PRINT_BLOCK,
    error::{Result, format_terminal_color, safe_constant, validate_component_range, validate_interpolation_factor},
    spaces::{Grey, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, LabAlpha, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz, XyzAlpha},
    traits::{Colour, Convert},
};

/// LAB colour representation.
#[derive(Debug, Clone, Copy)]
pub struct Lab<T: Float + Send + Sync> {
    /// Lightness component in range [0, 100].
    lightness: T,
    /// a* component in range [-128, 127].
    a_star: T,
    /// b* component in range [-128, 127].
    b_star: T,
}

impl<T: Float + Send + Sync> Lab<T> {
    /// Create a new `Lab` instance with validation.
    ///
    /// # Arguments
    ///
    /// * `lightness` - The L* component, must be in range [0, 100]
    /// * `a_star` - The a* component, must be in range [-128, 127]
    /// * `b_star` - The b* component, must be in range [-128, 127]
    ///
    /// # Errors
    ///
    /// Returns an error if lightness is outside [0, 100] or if a*/b* are outside [-128, 127].
    pub fn new(lightness: T, a_star: T, b_star: T) -> Result<Self> {
        let max_lightness = safe_constant(100.0)?;
        let min_chroma = safe_constant(-128.0)?;
        let max_chroma = safe_constant(127.0)?;

        validate_component_range(lightness, "lightness", T::zero(), max_lightness)?;
        validate_component_range(a_star, "a*", min_chroma, max_chroma)?;
        validate_component_range(b_star, "b*", min_chroma, max_chroma)?;

        Ok(Self {
            lightness,
            a_star,
            b_star,
        })
    }

    /// Get the `lightness` component (L*).
    pub const fn lightness(&self) -> T {
        self.lightness
    }

    /// Get the `a_star` component (a*).
    pub const fn a_star(&self) -> T {
        self.a_star
    }

    /// Get the `b_star` component (b*).
    pub const fn b_star(&self) -> T {
        self.b_star
    }

    /// Set the `lightness` component with validation.
    ///
    /// # Arguments
    ///
    /// * `lightness` - The new L* value, must be in range [0, 100]
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 100].
    pub fn set_lightness(&mut self, lightness: T) -> Result<()> {
        let max_lightness = safe_constant(100.0)?;
        validate_component_range(lightness, "lightness", T::zero(), max_lightness)?;
        self.lightness = lightness;
        Ok(())
    }

    /// Set the `a_star` component with validation.
    ///
    /// # Arguments
    ///
    /// * `a_star` - The new a* value, must be in range [-128, 127]
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [-128, 127].
    pub fn set_a_star(&mut self, a_star: T) -> Result<()> {
        let min_chroma = safe_constant(-128.0)?;
        let max_chroma = safe_constant(127.0)?;
        validate_component_range(a_star, "a*", min_chroma, max_chroma)?;
        self.a_star = a_star;
        Ok(())
    }

    /// Set the `b_star` component with validation.
    ///
    /// # Arguments
    ///
    /// * `b_star` - The new b* value, must be in range [-128, 127]
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [-128, 127].
    pub fn set_b_star(&mut self, b_star: T) -> Result<()> {
        let min_chroma = safe_constant(-128.0)?;
        let max_chroma = safe_constant(127.0)?;
        validate_component_range(b_star, "b*", min_chroma, max_chroma)?;
        self.b_star = b_star;
        Ok(())
    }

    /// Calculate perceptual colour difference in Lab space (CIE76 Delta E).
    /// The Delta E value indicates how different two colours appear, with values:
    /// - < 1.0: Not perceptible by human eyes
    /// - 1-2: Perceptible through close observation
    /// - 2-10: Perceptible at a glance
    /// - > 10: Colours are more similar than opposite
    pub fn delta_e(&self, other: &Self) -> T {
        let dl = self.lightness - other.lightness;
        let da = self.a_star - other.a_star;
        let db = self.b_star - other.b_star;

        (dl * dl + da * da + db * db).sqrt()
    }

    /// Calculate perceptual colour difference using the improved CIE94 Delta E formula.
    /// This is more accurate than the basic `delta_e` method, especially for saturated colours.
    ///
    /// # Errors
    ///
    /// Returns an error if mathematical operations fail during calculation.
    pub fn delta_e94(&self, other: &Self) -> Result<T> {
        // Weighting factors
        let k_l = T::one();
        let k_c = T::one();
        let k_h = T::one();
        let k1 = safe_constant::<f64, T>(0.045)?;
        let k2 = safe_constant::<f64, T>(0.015)?;

        // Calculate differences
        let delta_l = self.lightness - other.lightness;

        // Calculate C1, C2 (Chroma)
        let c1 = (self.a_star * self.a_star + self.b_star * self.b_star).sqrt();
        let c2 = (other.a_star * other.a_star + other.b_star * other.b_star).sqrt();

        // Calculate delta_c (difference in Chroma)
        let delta_c = c1 - c2;

        // Calculate delta_h (difference in Hue)
        let delta_a = self.a_star - other.a_star;
        let delta_b = self.b_star - other.b_star;
        let delta_h_squared = delta_a * delta_a + delta_b * delta_b - delta_c * delta_c;
        let delta_h = if delta_h_squared.is_sign_negative() {
            T::zero()
        } else {
            delta_h_squared.sqrt()
        };

        // Calculate the S_L, S_C, S_H scaling factors
        let s_l = T::one();
        let s_c = T::one() + k1 * c1;
        let s_h = T::one() + k2 * c1;

        // Calculate the final Delta E94
        let term1 = (delta_l / (k_l * s_l)).powi(2);
        let term2 = (delta_c / (k_c * s_c)).powi(2);
        let term3 = (delta_h / (k_h * s_h)).powi(2);

        Ok((term1 + term2 + term3).sqrt())
    }
}

impl<T: Float + Send + Sync> Colour<T, 3> for Lab<T> {
    fn from_hex(hex: &str) -> Result<Self> {
        // Convert from hex to Lab via sRGB and XYZ
        let srgb = Srgb::from_hex(hex)?;
        srgb.to_lab()
    }

    fn to_hex(&self) -> Result<String> {
        // Convert to hex via sRGB
        self.to_srgb()?.to_hex()
    }

    fn from_bytes(bytes: [u8; 3]) -> Result<Self> {
        // Convert from bytes to Lab via sRGB and XYZ
        Srgb::from_bytes(bytes)?.to_lab()
    }

    fn to_bytes(self) -> Result<[u8; 3]> {
        // Convert to bytes via sRGB
        self.to_srgb()?.to_bytes()
    }

    /// Linear interpolate between two Lab colours.
    ///
    /// Lab is designed to be perceptually uniform, so linear interpolation
    /// in this space produces perceptually uniform gradients.
    fn lerp(lhs: &Self, rhs: &Self, t: T) -> Result<Self> {
        validate_interpolation_factor(t)?;

        Self::new(
            lhs.lightness * (T::one() - t) + rhs.lightness * t,
            lhs.a_star * (T::one() - t) + rhs.a_star * t,
            lhs.b_star * (T::one() - t) + rhs.b_star * t,
        )
    }
}

impl<T: Float + Send + Sync> Convert<T> for Lab<T> {
    fn to_grey(&self) -> Result<Grey<T>> {
        // For greyscale, we should just use the L component (lightness)
        // We need to normalize from [0, 100] to [0, 1]
        let l_normalized = self.lightness / safe_constant(100.0)?;
        Grey::new(l_normalized)
    }

    fn to_grey_alpha(&self) -> Result<GreyAlpha<T>> {
        let l_normalized = self.lightness / safe_constant(100.0)?;
        GreyAlpha::new(l_normalized, T::one())
    }

    fn to_hsl(&self) -> Result<Hsl<T>> {
        self.to_rgb()?.to_hsl()
    }

    fn to_hsl_alpha(&self) -> Result<HslAlpha<T>> {
        let hsl = self.to_hsl()?;
        HslAlpha::new(hsl.hue(), hsl.saturation(), hsl.lightness(), T::one())
    }

    fn to_hsv(&self) -> Result<Hsv<T>> {
        // Convert Lab to HSV via XYZ and RGB
        self.to_xyz()?.to_rgb()?.to_hsv()
    }

    fn to_hsv_alpha(&self) -> Result<HsvAlpha<T>> {
        let hsv = self.to_hsv()?;
        HsvAlpha::new(hsv.hue(), hsv.saturation(), hsv.value(), T::one())
    }

    fn to_lab(&self) -> Result<Self> {
        Ok(*self)
    }

    fn to_lab_alpha(&self) -> Result<LabAlpha<T>> {
        LabAlpha::new(self.lightness(), self.a_star(), self.b_star(), T::one())
    }

    fn to_rgb(&self) -> Result<Rgb<T>> {
        // Convert Lab to RGB via XYZ
        self.to_xyz()?.to_rgb()
    }

    fn to_rgb_alpha(&self) -> Result<RgbAlpha<T>> {
        let rgb = self.to_rgb()?;
        RgbAlpha::new(rgb.red(), rgb.green(), rgb.blue(), T::one())
    }

    fn to_srgb(&self) -> Result<Srgb<T>> {
        // Convert Lab to sRGB via XYZ
        self.to_xyz()?.to_srgb()
    }

    fn to_srgb_alpha(&self) -> Result<SrgbAlpha<T>> {
        let srgb = self.to_srgb()?;
        SrgbAlpha::new(srgb.red(), srgb.green(), srgb.blue(), T::one())
    }

    fn to_xyz(&self) -> Result<Xyz<T>> {
        // Constants for the conversion
        let epsilon = safe_constant(0.008_856)?; // Intent is 216/24389
        let kappa = safe_constant(903.3)?; // Intent is 24389/27

        // D65 reference white
        let ref_white = Xyz::<T>::d65_reference_white()?;

        // Compute f_y
        let l = self.lightness;
        let f_y = (l + safe_constant(16.0)?) / safe_constant(116.0)?;

        // Compute f_x and f_z using a and b
        let f_x = self.a_star / safe_constant(500.0)? + f_y;
        let f_z = f_y - self.b_star / safe_constant(200.0)?;

        // Convert f values to XYZ coordinates
        let x_r = if f_x.powi(3) > epsilon {
            f_x.powi(3)
        } else {
            (f_x * safe_constant(116.0)? - safe_constant(16.0)?) / kappa
        };

        let y_r = if l > safe_constant(8.0)? {
            ((l + safe_constant(16.0)?) / safe_constant(116.0)?).powi(3)
        } else {
            l / kappa
        };

        let z_r = if f_z.powi(3) > epsilon {
            f_z.powi(3)
        } else {
            (f_z * safe_constant(116.0)? - safe_constant(16.0)?) / kappa
        };

        // Scale by reference white
        let x = x_r * ref_white.x();
        let y = y_r * ref_white.y();
        let z = z_r * ref_white.z();

        Xyz::new(x, y, z)
    }

    fn to_xyz_alpha(&self) -> Result<XyzAlpha<T>> {
        let xyz = self.to_xyz()?;
        XyzAlpha::new(xyz.x(), xyz.y(), xyz.z(), T::one())
    }
}

impl<T: Float + Send + Sync> Display for Lab<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        let rgb = self.to_rgb()?;
        let color_string = format_terminal_color(rgb.red(), rgb.green(), rgb.blue(), PRINT_BLOCK)?;
        write!(fmt, "{color_string}")
    }
}
