//! XYZ colour representation.
//! The XYZ colour space is a device-independent colour space defined by the CIE (International Commission on Illumination).
//! It was created to be a standard reference space for mapping human colour perception.

use num_traits::Float;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    config::PRINT_BLOCK,
    error::{Result, format_terminal_color, safe_constant, validate_interpolation_factor, validate_unit_component},
    spaces::{Grey, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, Rgb, RgbAlpha, Srgb, SrgbAlpha, XyzAlpha},
    traits::{Colour, Convert},
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
    /// Note: XYZ values are theoretically unbounded, but we enforce non-negative values
    /// and practical upper bounds for this implementation.
    /// Typical values for D65 reference white are X ≈ 0.95, Y = 1.0, Z ≈ 1.09.
    ///
    /// # Arguments
    ///
    /// * `x` - The X component, must be in range [0, 1]
    /// * `y` - The Y component (luminance), must be in range [0, 1]  
    /// * `z` - The Z component, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if any component is outside the range [0, 1].
    pub fn new(x: T, y: T, z: T) -> Result<Self> {
        validate_unit_component(x, "x")?;
        validate_unit_component(y, "y")?;
        validate_unit_component(z, "z")?;

        Ok(Self { x, y, z })
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
    ///
    /// # Arguments
    ///
    /// * `x` - The new X value, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 1].
    pub fn set_x(&mut self, x: T) -> Result<()> {
        validate_unit_component(x, "x")?;
        self.x = x;
        Ok(())
    }

    /// Set the `y` component (luminance).
    ///
    /// # Arguments
    ///
    /// * `y` - The new Y value, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 1].
    pub fn set_y(&mut self, y: T) -> Result<()> {
        validate_unit_component(y, "y")?;
        self.y = y;
        Ok(())
    }

    /// Set the `z` component.
    ///
    /// # Arguments
    ///
    /// * `z` - The new Z value, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 1].
    pub fn set_z(&mut self, z: T) -> Result<()> {
        validate_unit_component(z, "z")?;
        self.z = z;
        Ok(())
    }

    /// Set all components at once with validation.
    ///
    /// # Arguments
    ///
    /// * `x` - The X component, must be in range [0, 1]
    /// * `y` - The Y component (luminance), must be in range [0, 1]
    /// * `z` - The Z component, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if any component is outside the range [0, 1].
    pub fn set_components(&mut self, x: T, y: T, z: T) -> Result<()> {
        validate_unit_component(x, "x")?;
        validate_unit_component(y, "y")?;
        validate_unit_component(z, "z")?;

        self.x = x;
        self.y = y;
        self.z = z;
        Ok(())
    }

    /// Create an XYZ colour representing the D65 standard illuminant (daylight, 6504K).
    ///
    /// # Errors
    ///
    /// Returns an error if constant conversion fails.
    pub fn d65_reference_white() -> Result<Self> {
        Self::new(safe_constant(0.95047)?, safe_constant(1.0)?, safe_constant(1.08883)?)
    }

    /// Create an XYZ colour representing the D50 standard illuminant (horizon light, 5003K).
    ///
    /// # Errors
    ///
    /// Returns an error if constant conversion fails.
    pub fn d50_reference_white() -> Result<Self> {
        Self::new(safe_constant(0.96422)?, safe_constant(1.0)?, safe_constant(0.82521)?)
    }

    /// Get XYZ values relative to D65 reference white.
    /// Returns (X/Xn, Y/Yn, Z/Zn)
    ///
    /// # Errors
    ///
    /// Returns an error if reference white calculation fails.
    pub fn relative_to_white(&self) -> Result<(T, T, T)> {
        let white = Self::d65_reference_white()?;
        Ok((self.x / white.x, self.y / white.y, self.z / white.z))
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
        srgb.to_xyz()
    }

    fn to_hex(&self) -> Result<String> {
        // Convert to hex via sRGB
        self.to_srgb()?.to_hex()
    }

    fn from_bytes(bytes: [u8; 3]) -> Result<Self> {
        // Convert from bytes to XYZ via sRGB
        Srgb::from_bytes(bytes)?.to_xyz()
    }

    fn to_bytes(self) -> Result<[u8; 3]> {
        // Convert to bytes via sRGB
        self.to_srgb()?.to_bytes()
    }

    /// Linear interpolate between two XYZ colours.
    /// Note: Prefer Lab for perceptually uniform interpolation.
    fn lerp(lhs: &Self, rhs: &Self, t: T) -> Result<Self> {
        validate_interpolation_factor(t)?;

        Self::new(
            lhs.x * (T::one() - t) + rhs.x * t,
            lhs.y * (T::one() - t) + rhs.y * t,
            lhs.z * (T::one() - t) + rhs.z * t,
        )
    }
}

impl<T: Float + Send + Sync> Convert<T> for Xyz<T> {
    fn to_grey(&self) -> Result<Grey<T>> {
        // Use the Y component (luminance) for greyscale
        // Clamp to [0, 1] range for Grey
        Grey::new(self.y.min(T::one()))
    }

    fn to_grey_alpha(&self) -> Result<GreyAlpha<T>> {
        GreyAlpha::new(self.y.min(T::one()), T::one())
    }

    fn to_hsl(&self) -> Result<Hsl<T>> {
        self.to_rgb()?.to_hsl()
    }

    fn to_hsl_alpha(&self) -> Result<HslAlpha<T>> {
        let hsl = self.to_hsl()?;
        HslAlpha::new(hsl.hue(), hsl.saturation(), hsl.lightness(), T::one())
    }

    fn to_hsv(&self) -> Result<Hsv<T>> {
        // Convert XYZ to HSV via linear RGB
        self.to_rgb()?.to_hsv()
    }

    fn to_hsv_alpha(&self) -> Result<HsvAlpha<T>> {
        let hsv = self.to_hsv()?;
        HsvAlpha::new(hsv.hue(), hsv.saturation(), hsv.value(), T::one())
    }

    fn to_lab(&self) -> Result<Lab<T>> {
        // Constants for the conversion
        let epsilon = safe_constant(0.008_856)?; // Intent is 216/24389
        let kappa = safe_constant::<f64, T>(903.3)?; // Intent is 24389/27

        // Get XYZ values relative to reference white (D65)
        let (x_r, y_r, z_r) = self.relative_to_white()?;

        // Compute f(x), f(y), f(z)
        let f_x = if x_r > epsilon {
            x_r.powf(safe_constant(1.0 / 3.0)?)
        } else {
            (kappa * x_r + safe_constant(16.0)?) / safe_constant(116.0)?
        };

        let f_y = if y_r > epsilon {
            y_r.powf(safe_constant(1.0 / 3.0)?)
        } else {
            (kappa * y_r + safe_constant(16.0)?) / safe_constant(116.0)?
        };

        let f_z = if z_r > epsilon {
            z_r.powf(safe_constant(1.0 / 3.0)?)
        } else {
            (kappa * z_r + safe_constant(16.0)?) / safe_constant(116.0)?
        };

        // Compute Lab components
        let l = safe_constant::<f64, T>(116.0)? * f_y - safe_constant(16.0)?;
        let a = safe_constant::<f64, T>(500.0)? * (f_x - f_y);
        let b = safe_constant::<f64, T>(200.0)? * (f_y - f_z);

        Lab::new(l, a, b)
    }

    fn to_lab_alpha(&self) -> Result<LabAlpha<T>> {
        let lab = self.to_lab()?;
        LabAlpha::new(lab.lightness(), lab.a_star(), lab.b_star(), T::one())
    }

    fn to_rgb(&self) -> Result<Rgb<T>> {
        // XYZ to linear RGB transformation
        // Using the inverse of the RGB to XYZ matrix
        let r =
            self.x * safe_constant(3.240_454_2)? - self.y * safe_constant(1.537_138_5)? - self.z * safe_constant(0.498_531_4)?;

        let g =
            -self.x * safe_constant(0.969_266_0)? + self.y * safe_constant(1.876_010_8)? + self.z * safe_constant(0.041_556_0)?;

        let b =
            self.x * safe_constant(0.055_643_4)? - self.y * safe_constant(0.204_025_9)? + self.z * safe_constant(1.057_225_2)?;

        // Clamp to [0, 1] range
        let clamped_r = r.max(T::zero()).min(T::one());
        let clamped_g = g.max(T::zero()).min(T::one());
        let clamped_b = b.max(T::zero()).min(T::one());

        Rgb::new(clamped_r, clamped_g, clamped_b)
    }

    fn to_rgb_alpha(&self) -> Result<RgbAlpha<T>> {
        let rgb = self.to_rgb()?;
        RgbAlpha::new(rgb.red(), rgb.green(), rgb.blue(), T::one())
    }

    fn to_srgb(&self) -> Result<Srgb<T>> {
        // Convert XYZ to sRGB via linear RGB
        let rgb = self.to_rgb()?;

        // Apply gamma encoding to get sRGB
        let r_srgb = Srgb::gamma_encode(rgb.red())?;
        let g_srgb = Srgb::gamma_encode(rgb.green())?;
        let b_srgb = Srgb::gamma_encode(rgb.blue())?;

        Srgb::new(r_srgb, g_srgb, b_srgb)
    }

    fn to_srgb_alpha(&self) -> Result<SrgbAlpha<T>> {
        let srgb = self.to_srgb()?;
        SrgbAlpha::new(srgb.red(), srgb.green(), srgb.blue(), T::one())
    }

    fn to_xyz(&self) -> Result<Self> {
        Ok(*self)
    }

    fn to_xyz_alpha(&self) -> Result<XyzAlpha<T>> {
        XyzAlpha::new(self.x(), self.y(), self.z(), T::one())
    }
}

impl<T: Float + Send + Sync> Display for Xyz<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        let rgb = self.to_rgb()?;
        let color_string = format_terminal_color(rgb.red(), rgb.green(), rgb.blue(), PRINT_BLOCK)?;
        write!(fmt, "{color_string}")
    }
}
