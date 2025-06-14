//! HSL (Hue, Saturation, Lightness) colour representation.

use num_traits::Float;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    Colour, Convert, Grey, GreyAlpha, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz, XyzAlpha,
    config::PRINT_BLOCK, error::Result,
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
    /// Create a new `Hsl` instance.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    pub fn new(mut hue: T, saturation: T, lightness: T) -> Self {
        // Normalise hue to be within [0, 360).
        let normalised_hue = {
            let f360 = T::from(360.0).unwrap();
            while hue >= f360 {
                hue = hue - f360;
            }
            while hue < T::zero() {
                hue = hue + f360;
            }
            hue
        };

        debug_assert!(
            normalised_hue >= T::zero() && normalised_hue < T::from(360.0).unwrap(),
            "Hue component must be between 0 and 360."
        );
        debug_assert!(
            !(saturation < T::zero() || saturation > T::one()),
            "Saturation component must be between 0 and 1."
        );
        debug_assert!(
            !(lightness < T::zero() || lightness > T::one()),
            "Lightness component must be between 0 and 1."
        );
        Self {
            hue: normalised_hue,
            saturation,
            lightness,
        }
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

    /// Set the `hue` component in degrees [0, 360).
    ///
    /// # Panics
    ///
    /// This function will not panic.
    pub fn set_hue(&mut self, mut hue: T) {
        // Normalize hue to be within [0, 360)
        let f360 = T::from(360.0).unwrap();
        while hue >= f360 {
            hue = hue - f360;
        }
        while hue < T::zero() {
            hue = hue + f360;
        }

        debug_assert!(
            hue >= T::zero() && hue < T::from(360.0).unwrap(),
            "Hue component must be between 0 and 360."
        );
        self.hue = hue;
    }

    /// Set the `saturation` component.
    pub fn set_saturation(&mut self, saturation: T) {
        debug_assert!(
            saturation >= T::zero() && saturation <= T::one(),
            "Saturation component must be between 0 and 1."
        );
        self.saturation = saturation;
    }

    /// Set the `lightness` component.
    pub fn set_lightness(&mut self, lightness: T) {
        debug_assert!(
            lightness >= T::zero() && lightness <= T::one(),
            "Lightness component must be between 0 and 1."
        );
        self.lightness = lightness;
    }
}

impl<T: Float + Send + Sync> Colour<T, 3> for Hsl<T> {
    fn from_hex(hex: &str) -> Result<Self> {
        Ok(Rgb::from_hex(hex)?.to_hsl())
    }

    fn to_hex(&self) -> String {
        self.to_rgb().to_hex()
    }

    fn from_bytes(bytes: [u8; 3]) -> Self {
        Rgb::from_bytes(bytes).to_hsl()
    }

    fn to_bytes(self) -> [u8; 3] {
        self.to_rgb().to_bytes()
    }

    /// Linear interpolate between two HSL colours.
    /// This uses the shortest path around the hue circle for interpolation.
    fn lerp(lhs: &Self, rhs: &Self, t: T) -> Self {
        debug_assert!(
            t >= T::zero() && t <= T::one(),
            "Interpolation factor must be in range [0, 1]."
        );

        // For hue, we need special handling to ensure we take the shortest path around the color wheel
        let mut hue_diff = rhs.hue - lhs.hue;

        // If the difference is greater than 180 degrees, it's shorter to go the other way around the color wheel
        if hue_diff > T::from(180).unwrap() {
            hue_diff = hue_diff - T::from(360).unwrap();
        } else if hue_diff < T::from(-180).unwrap() {
            hue_diff = hue_diff + T::from(360).unwrap();
        }

        // Calculate the interpolated hue and ensure it stays in [0, 360] range
        let mut hue = lhs.hue + t * hue_diff;
        if hue < T::zero() {
            hue = hue + T::from(360).unwrap();
        } else if hue > T::from(360).unwrap() {
            hue = hue - T::from(360).unwrap();
        }

        // Linear interpolation for saturation and lightness
        let saturation = lhs.saturation * (T::one() - t) + rhs.saturation * t;
        let lightness = lhs.lightness * (T::one() - t) + rhs.lightness * t;

        Self::new(hue, saturation, lightness)
    }
}

impl<T: Float + Send + Sync> Convert<T> for Hsl<T> {
    fn to_grey(&self) -> Grey<T> {
        Grey::new(self.lightness)
    }

    fn to_grey_alpha(&self) -> GreyAlpha<T> {
        GreyAlpha::new(self.lightness, T::one())
    }

    fn to_hsv(&self) -> Hsv<T> {
        // v = L + S_l * min(L, 1-L)
        let delta =
            self.saturation * (T::one() - (T::from(2.0).unwrap() * self.lightness - T::one()).abs()) / T::from(2.0).unwrap();
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

    fn to_hsv_alpha(&self) -> HsvAlpha<T> {
        let hsv = self.to_hsv();
        HsvAlpha::new(hsv.hue(), hsv.saturation(), hsv.value(), T::one())
    }

    fn to_hsl(&self) -> Self {
        *self
    }

    fn to_hsl_alpha(&self) -> HslAlpha<T> {
        HslAlpha::new(self.hue, self.saturation, self.lightness, T::one())
    }

    fn to_lab(&self) -> Lab<T> {
        // Convert HSL to Lab via RGB and XYZ
        self.to_rgb().to_lab()
    }

    fn to_lab_alpha(&self) -> LabAlpha<T> {
        let lab = self.to_lab();
        LabAlpha::new(lab.lightness(), lab.a_star(), lab.b_star(), T::one())
    }

    #[expect(
        clippy::many_single_char_names,
        reason = "These variable names are commonly used in HSL to RGB conversion."
    )]
    fn to_rgb(&self) -> Rgb<T> {
        let lightness = self.lightness;
        let saturation = self.saturation;

        // If saturation is 0, the color is a shade of gray
        if saturation.abs() < T::epsilon() {
            return Rgb::new(lightness, lightness, lightness);
        }

        // Helper function for HSL to RGB conversion
        let hue_to_rgb = |p: T, q: T, mut t: T| -> T {
            let f6 = T::from(6.0).unwrap();
            let f2 = T::from(2.0).unwrap();
            let f3 = T::from(3.0).unwrap();

            // Normalize t to be in range [0, 1]
            if t < T::zero() {
                t = t + T::one();
            }
            if t > T::one() {
                t = t - T::one();
            }

            if t < T::one() / f6 {
                return p + (q - p) * f6 * t;
            }
            if t < T::one() / f2 {
                return q;
            }
            if t < f2 / f3 {
                return p + (q - p) * (f2 / f3 - t) * f6;
            }
            p
        };

        let q = if lightness < T::from(0.5).unwrap() {
            lightness * (T::one() + saturation)
        } else {
            lightness + saturation - lightness * saturation
        };

        let p = T::from(2.0).unwrap() * lightness - q;
        let h = self.hue / T::from(360.0).unwrap();

        let r = hue_to_rgb(p, q, h + T::from(1.0 / 3.0).unwrap());
        let g = hue_to_rgb(p, q, h);
        let b = hue_to_rgb(p, q, h - T::from(1.0 / 3.0).unwrap());

        Rgb::new(r, g, b)
    }

    fn to_rgb_alpha(&self) -> RgbAlpha<T> {
        let rgb = self.to_rgb();
        RgbAlpha::new(rgb.red(), rgb.green(), rgb.blue(), T::one())
    }

    fn to_srgb(&self) -> Srgb<T> {
        // Convert HSL to sRGB via linear RGB
        let rgb = self.to_rgb();
        let r_srgb = Srgb::gamma_encode(rgb.red());
        let g_srgb = Srgb::gamma_encode(rgb.green());
        let b_srgb = Srgb::gamma_encode(rgb.blue());

        Srgb::new(r_srgb, g_srgb, b_srgb)
    }

    fn to_srgb_alpha(&self) -> SrgbAlpha<T> {
        let srgb = self.to_srgb();
        SrgbAlpha::new(srgb.red(), srgb.green(), srgb.blue(), T::one())
    }

    fn to_xyz(&self) -> Xyz<T> {
        // Convert HSL to XYZ via linear RGB
        self.to_rgb().to_xyz()
    }

    fn to_xyz_alpha(&self) -> XyzAlpha<T> {
        let xyz = self.to_xyz();
        XyzAlpha::new(xyz.x(), xyz.y(), xyz.z(), T::one())
    }
}

impl<T: Float + Send + Sync> Display for Hsl<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        let rgb = self.to_rgb();
        let max = T::from(255_i32).unwrap();
        let red = (rgb.red() * max).round().to_u8().unwrap();
        let green = (rgb.green() * max).round().to_u8().unwrap();
        let blue = (rgb.blue() * max).round().to_u8().unwrap();
        write!(fmt, "\x1b[38;2;{red};{green};{blue}m{PRINT_BLOCK}\x1b[0m")
    }
}
