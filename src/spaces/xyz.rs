//! XYZ colour representation.
//! The XYZ colour space is a device-independent colour space defined by the CIE (International Commission on Illumination).
//! It was created to be a standard reference space for mapping human colour perception.

use num_traits::Float;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    Colour, Convert, Grey, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, Rgb, RgbAlpha, Srgb, SrgbAlpha, XyzAlpha,
    config::PRINT_BLOCK, error::Result,
};

/// XYZ colour representation.
#[derive(Debug, Clone, Copy)]
pub struct Xyz<T: Float + Send + Sync> {
    /// X component.
    x: T,
    /// Y component (luminance).
    y: T,
    /// Z component.
    z: T,
}

impl<T: Float + Send + Sync> Xyz<T> {
    /// Create a new `Xyz` instance.
    /// Note: XYZ values are theoretically unbounded, but non-negative values are enforced here for practical reasons.
    /// Typical values for D65 reference white are X ≈ 0.95, Y = 1.0, Z ≈ 1.09.
    pub fn new(x: T, y: T, z: T) -> Self {
        debug_assert!(x >= T::zero(), "X component should be non-negative.");
        debug_assert!(y >= T::zero(), "Y component should be non-negative.");
        debug_assert!(z >= T::zero(), "Z component should be non-negative.");
        Self { x, y, z }
    }

    /// Get the `x` component.
    pub const fn x(&self) -> T {
        self.x
    }

    /// Get the `y` component (luminance).
    pub const fn y(&self) -> T {
        self.y
    }

    /// Get the `z` component.
    pub const fn z(&self) -> T {
        self.z
    }

    /// Set the `x` component.
    pub fn set_x(&mut self, x: T) {
        debug_assert!(x >= T::zero(), "X component should be non-negative.");
        self.x = x;
    }

    /// Set the `y` component (luminance).
    pub fn set_y(&mut self, y: T) {
        debug_assert!(y >= T::zero(), "Y component should be non-negative.");
        self.y = y;
    }

    /// Set the `z` component.
    pub fn set_z(&mut self, z: T) {
        debug_assert!(z >= T::zero(), "Z component should be non-negative.");
        self.z = z;
    }

    /// Create an XYZ colour representing the D65 standard illuminant (daylight, 6504K).
    ///
    /// # Panics
    ///
    /// This function will not panic.
    #[must_use]
    pub fn d65_reference_white() -> Self {
        Self::new(T::from(0.95047).unwrap(), T::from(1.0).unwrap(), T::from(1.08883).unwrap())
    }

    /// Create an XYZ colour representing the D50 standard illuminant (horizon light, 5003K).
    ///
    /// # Panics
    ///
    /// This function will not panic.
    #[must_use]
    pub fn d50_reference_white() -> Self {
        Self::new(T::from(0.96422).unwrap(), T::from(1.0).unwrap(), T::from(0.82521).unwrap())
    }

    /// Get XYZ values relative to D65 reference white.
    /// Returns (X/Xn, Y/Yn, Z/Zn)
    pub fn relative_to_white(&self) -> (T, T, T) {
        let white = Self::d65_reference_white();
        (self.x / white.x, self.y / white.y, self.z / white.z)
    }

    /// Calculate perceptual colour difference in XYZ space (simple Euclidean distance).
    /// Note: This is not an ideal colour difference metric - consider using Lab with Delta E metrics for better results.
    pub fn distance(&self, other: &Self) -> T {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

impl<T: Float + Send + Sync> Colour<T, 3> for Xyz<T> {
    fn from_hex(hex: &str) -> Result<Self> {
        // Convert from hex to XYZ via sRGB
        // First parse the hex to sRGB
        let srgb = Srgb::from_hex(hex)?;

        // Then convert sRGB to XYZ
        Ok(srgb.to_xyz())
    }

    fn to_hex(&self) -> String {
        // Convert to hex via sRGB
        self.to_srgb().to_hex()
    }

    fn from_bytes(bytes: [u8; 3]) -> Self {
        // Convert from bytes to XYZ via sRGB
        Srgb::from_bytes(bytes).to_xyz()
    }

    fn to_bytes(self) -> [u8; 3] {
        // Convert to bytes via sRGB
        self.to_srgb().to_bytes()
    }

    /// Linear interpolate between two XYZ colours.
    /// Note: Prefer Lab for perceptually uniform interpolation.
    fn lerp(lhs: &Self, rhs: &Self, t: T) -> Self {
        debug_assert!(
            t >= T::zero() && t <= T::one(),
            "Interpolation factor must be in range [0, 1]."
        );
        Self::new(
            lhs.x * (T::one() - t) + rhs.x * t,
            lhs.y * (T::one() - t) + rhs.y * t,
            lhs.z * (T::one() - t) + rhs.z * t,
        )
    }
}

impl<T: Float + Send + Sync> Convert<T> for Xyz<T> {
    fn to_grey(&self) -> Grey<T> {
        // Use the Y component (luminance) for greyscale
        // Clamp to [0, 1] range for Grey
        Grey::new(self.y.min(T::one()))
    }

    fn to_grey_alpha(&self) -> GreyAlpha<T> {
        GreyAlpha::new(self.y.min(T::one()), T::one())
    }

    fn to_hsl(&self) -> Hsl<T> {
        self.to_rgb().to_hsl()
    }

    fn to_hsl_alpha(&self) -> HslAlpha<T> {
        let hsl = self.to_hsl();
        HslAlpha::new(hsl.hue(), hsl.saturation(), hsl.lightness(), T::one())
    }

    fn to_hsv(&self) -> Hsv<T> {
        // Convert XYZ to HSV via linear RGB
        self.to_rgb().to_hsv()
    }

    fn to_hsv_alpha(&self) -> HsvAlpha<T> {
        let hsv = self.to_hsv();
        HsvAlpha::new(hsv.hue(), hsv.saturation(), hsv.value(), T::one())
    }

    fn to_lab(&self) -> Lab<T> {
        // Constants for the conversion
        let epsilon = T::from(0.008856).unwrap(); // Intent is 216/24389
        let kappa = T::from(903.3).unwrap(); // Intent is 24389/27

        // Get XYZ values relative to reference white (D65)
        let (x_r, y_r, z_r) = self.relative_to_white();

        // Compute f(x), f(y), f(z)
        let f_x = if x_r > epsilon {
            x_r.powf(T::from(1.0 / 3.0).unwrap())
        } else {
            (kappa * x_r + T::from(16.0).unwrap()) / T::from(116.0).unwrap()
        };

        let f_y = if y_r > epsilon {
            y_r.powf(T::from(1.0 / 3.0).unwrap())
        } else {
            (kappa * y_r + T::from(16.0).unwrap()) / T::from(116.0).unwrap()
        };

        let f_z = if z_r > epsilon {
            z_r.powf(T::from(1.0 / 3.0).unwrap())
        } else {
            (kappa * z_r + T::from(16.0).unwrap()) / T::from(116.0).unwrap()
        };

        // Compute Lab components
        let l = T::from(116.0).unwrap() * f_y - T::from(16.0).unwrap();
        let a = T::from(500.0).unwrap() * (f_x - f_y);
        let b = T::from(200.0).unwrap() * (f_y - f_z);

        Lab::new(l, a, b)
    }

    fn to_lab_alpha(&self) -> LabAlpha<T> {
        let lab = self.to_lab();
        LabAlpha::new(lab.lightness(), lab.a_star(), lab.b_star(), T::one())
    }

    fn to_rgb(&self) -> Rgb<T> {
        // XYZ to linear RGB transformation
        // Using the inverse of the RGB to XYZ matrix
        let r =
            self.x * T::from(3.2404542).unwrap() - self.y * T::from(1.5371385).unwrap() - self.z * T::from(0.4985314).unwrap();

        let g =
            -self.x * T::from(0.9692660).unwrap() + self.y * T::from(1.8760108).unwrap() + self.z * T::from(0.0415560).unwrap();

        let b =
            self.x * T::from(0.0556434).unwrap() - self.y * T::from(0.2040259).unwrap() + self.z * T::from(1.0572252).unwrap();

        // Clamp to [0, 1] range
        let clamped_r = r.max(T::zero()).min(T::one());
        let clamped_g = g.max(T::zero()).min(T::one());
        let clamped_b = b.max(T::zero()).min(T::one());

        Rgb::new(clamped_r, clamped_g, clamped_b)
    }

    fn to_rgb_alpha(&self) -> RgbAlpha<T> {
        let rgb = self.to_rgb();
        RgbAlpha::new(rgb.red(), rgb.green(), rgb.blue(), T::one())
    }

    fn to_srgb(&self) -> Srgb<T> {
        // Convert XYZ to sRGB via linear RGB
        let rgb = self.to_rgb();

        // Apply gamma encoding to get sRGB
        let r_srgb = Srgb::gamma_encode(rgb.red());
        let g_srgb = Srgb::gamma_encode(rgb.green());
        let b_srgb = Srgb::gamma_encode(rgb.blue());

        Srgb::new(r_srgb, g_srgb, b_srgb)
    }

    fn to_srgb_alpha(&self) -> SrgbAlpha<T> {
        let srgb = self.to_srgb();
        SrgbAlpha::new(srgb.red(), srgb.green(), srgb.blue(), T::one())
    }

    fn to_xyz(&self) -> Self {
        *self
    }

    fn to_xyz_alpha(&self) -> XyzAlpha<T> {
        XyzAlpha::new(self.x(), self.y(), self.z(), T::one())
    }
}

impl<T: Float + Send + Sync> Display for Xyz<T> {
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
