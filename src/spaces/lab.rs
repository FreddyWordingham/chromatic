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
    Colour, Convert, Grey, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, LabAlpha, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz, XyzAlpha,
    config::PRINT_BLOCK, error::Result,
};

/// LAB colour representation.
#[derive(Debug, Clone, Copy)]
pub struct Lab<T: Float + Send + Sync> {
    /// Lightness component in range [0, 100].
    lightness: T,
    /// a component in range [-128, 127].
    a_star: T,
    /// b component in range [-128, 127].
    b_star: T,
}

impl<T: Float + Send + Sync> Lab<T> {
    /// Create a new `Lab` instance.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    pub fn new(lightness: T, a_star: T, b_star: T) -> Self {
        debug_assert!(
            lightness >= T::zero() && lightness <= T::from(100.0).unwrap(),
            "Lightness component must be between 0 and 100."
        );
        debug_assert!(
            a_star >= T::from(-128.0).unwrap() && a_star <= T::from(127.0).unwrap(),
            "a component must be between -128 and 127."
        );
        debug_assert!(
            b_star >= T::from(-128.0).unwrap() && b_star <= T::from(127.0).unwrap(),
            "b component must be between -128 and 127."
        );
        Self {
            lightness,
            a_star,
            b_star,
        }
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

    /// Set the `lightness` component (L).
    ///
    /// # Panics
    ///
    /// This function will not panic.
    pub fn set_lightness(&mut self, lightness: T) {
        debug_assert!(
            lightness >= T::zero() && lightness <= T::from(100.0).unwrap(),
            "Lightness component must be between 0 and 100."
        );
        self.lightness = lightness;
    }

    /// Set the `a_star` component.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    pub fn set_a_star(&mut self, a_star: T) {
        debug_assert!(
            a_star >= T::from(-128.0).unwrap() && a_star <= T::from(127.0).unwrap(),
            "a component must be between -128 and 127."
        );
        self.a_star = a_star;
    }

    /// Set the `b_star` component.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    pub fn set_b_star(&mut self, b_star: T) {
        debug_assert!(
            b_star >= T::from(-128.0).unwrap() && b_star <= T::from(127.0).unwrap(),
            "b component must be between -128 and 127."
        );
        self.b_star = b_star;
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
    /// # Panics
    ///
    /// This function will not panic.
    pub fn delta_e94(&self, other: &Self) -> T {
        // Weighting factors
        let k_l = T::one();
        let k_c = T::one();
        let k_h = T::one();
        let k1 = T::from(0.045).unwrap();
        let k2 = T::from(0.015).unwrap();

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

        (term1 + term2 + term3).sqrt()
    }
}

impl<T: Float + Send + Sync> Colour<T, 3> for Lab<T> {
    fn from_hex(hex: &str) -> Result<Self> {
        // Convert from hex to Lab via sRGB and XYZ
        let srgb = Srgb::from_hex(hex)?;
        Ok(srgb.to_lab())
    }

    fn to_hex(&self) -> String {
        // Convert to hex via sRGB
        self.to_srgb().to_hex()
    }

    fn from_bytes(bytes: [u8; 3]) -> Self {
        // Convert from bytes to Lab via sRGB and XYZ
        Srgb::from_bytes(bytes).to_lab()
    }

    fn to_bytes(self) -> [u8; 3] {
        // Convert to bytes via sRGB
        self.to_srgb().to_bytes()
    }

    /// Linear interpolate between two Lab colours.
    ///
    /// Lab is designed to be perceptually uniform, so linear interpolation
    /// in this space produces perceptually uniform gradients.
    fn lerp(lhs: &Self, rhs: &Self, t: T) -> Self {
        debug_assert!(
            t >= T::zero() && t <= T::one(),
            "Interpolation factor must be in range [0, 1]."
        );
        Self::new(
            lhs.lightness * (T::one() - t) + rhs.lightness * t,
            lhs.a_star * (T::one() - t) + rhs.a_star * t,
            lhs.b_star * (T::one() - t) + rhs.b_star * t,
        )
    }
}

impl<T: Float + Send + Sync> Convert<T> for Lab<T> {
    fn to_grey(&self) -> Grey<T> {
        // For greyscale, we should just use the L component (lightness)
        // We need to normalize from [0, 100] to [0, 1]
        let l_normalized = self.lightness / T::from(100.0).unwrap();
        Grey::new(l_normalized)
    }

    fn to_grey_alpha(&self) -> GreyAlpha<T> {
        let l_normalized = self.lightness / T::from(100.0).unwrap();
        GreyAlpha::new(l_normalized, T::one())
    }

    fn to_hsl(&self) -> Hsl<T> {
        self.to_rgb().to_hsl()
    }

    fn to_hsl_alpha(&self) -> HslAlpha<T> {
        let hsl = self.to_hsl();
        HslAlpha::new(hsl.hue(), hsl.saturation(), hsl.lightness(), T::one())
    }

    fn to_hsv(&self) -> Hsv<T> {
        // Convert Lab to HSV via XYZ and RGB
        self.to_xyz().to_rgb().to_hsv()
    }

    fn to_hsv_alpha(&self) -> HsvAlpha<T> {
        let hsv = self.to_hsv();
        HsvAlpha::new(hsv.hue(), hsv.saturation(), hsv.value(), T::one())
    }

    fn to_lab(&self) -> Self {
        *self
    }

    fn to_lab_alpha(&self) -> LabAlpha<T> {
        LabAlpha::new(self.lightness(), self.a_star(), self.b_star(), T::one())
    }

    fn to_rgb(&self) -> Rgb<T> {
        // Convert Lab to RGB via XYZ
        self.to_xyz().to_rgb()
    }

    fn to_rgb_alpha(&self) -> RgbAlpha<T> {
        let rgb = self.to_rgb();
        RgbAlpha::new(rgb.red(), rgb.green(), rgb.blue(), T::one())
    }

    fn to_srgb(&self) -> Srgb<T> {
        // Convert Lab to sRGB via XYZ
        self.to_xyz().to_srgb()
    }

    fn to_srgb_alpha(&self) -> SrgbAlpha<T> {
        let srgb = self.to_srgb();
        SrgbAlpha::new(srgb.red(), srgb.green(), srgb.blue(), T::one())
    }

    fn to_xyz(&self) -> Xyz<T> {
        // Constants for the conversion
        let epsilon = T::from(0.008856).unwrap(); // Intent is 216/24389
        let kappa = T::from(903.3).unwrap(); // Intent is 24389/27

        // D65 reference white
        let ref_white = Xyz::<T>::d65_reference_white();

        // Compute f_y
        let l = self.lightness;
        let f_y = (l + T::from(16.0).unwrap()) / T::from(116.0).unwrap();

        // Compute f_x and f_z using a and b
        let f_x = self.a_star / T::from(500.0).unwrap() + f_y;
        let f_z = f_y - self.b_star / T::from(200.0).unwrap();

        // Convert f values to XYZ coordinates
        let x_r = if f_x.powi(3) > epsilon {
            f_x.powi(3)
        } else {
            (f_x * T::from(116.0).unwrap() - T::from(16.0).unwrap()) / kappa
        };

        let y_r = if l > T::from(8.0).unwrap() {
            ((l + T::from(16.0).unwrap()) / T::from(116.0).unwrap()).powi(3)
        } else {
            l / kappa
        };

        let z_r = if f_z.powi(3) > epsilon {
            f_z.powi(3)
        } else {
            (f_z * T::from(116.0).unwrap() - T::from(16.0).unwrap()) / kappa
        };

        // Scale by reference white
        let x = x_r * ref_white.x();
        let y = y_r * ref_white.y();
        let z = z_r * ref_white.z();

        Xyz::new(x, y, z)
    }

    fn to_xyz_alpha(&self) -> XyzAlpha<T> {
        let xyz = self.to_xyz();
        XyzAlpha::new(xyz.x(), xyz.y(), xyz.z(), T::one())
    }
}

impl<T: Float + Send + Sync> Display for Lab<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        // Convert to RGB for terminal display
        let rgb = self.to_rgb();
        let max = T::from(255_i32).unwrap();
        let red = (rgb.red() * max).round().to_u8().unwrap();
        let green = (rgb.green() * max).round().to_u8().unwrap();
        let blue = (rgb.blue() * max).round().to_u8().unwrap();
        write!(fmt, "\x1b[38;2;{red};{green};{blue}m{PRINT_BLOCK}\x1b[0m")
    }
}
