//! Monochrome colour representation.

use num_traits::Float;
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    num::ParseIntError,
};

use crate::{
    Colour, Convert, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, ParseColourError, Rgb, RgbAlpha, Srgb, SrgbAlpha,
    Xyz, XyzAlpha, config::PRINT_BLOCK,
};

/// Monochrome colour.
#[derive(Debug, Clone, Copy)]
pub struct Grey<T: Float + Send + Sync> {
    /// Grey component.
    grey: T,
}

impl<T: Float + Send + Sync> Grey<T> {
    /// Create a new `Grey` instance.
    pub fn new(grey: T) -> Self {
        debug_assert!(
            !(grey < T::zero() || grey > T::one()),
            "Grey component must be between 0 and 1."
        );
        Self { grey }
    }

    /// Get the `grey` component.
    pub const fn grey(&self) -> T {
        self.grey
    }

    /// Set the `grey` component.
    pub fn set_grey(&mut self, grey: T) {
        debug_assert!(
            grey >= T::zero() && grey <= T::one(),
            "Grey component must be between 0 and 1."
        );
        self.grey = grey;
    }
}

impl<T: Float + Send + Sync> Colour<T, 1> for Grey<T> {
    fn from_hex(hex: &str) -> Result<Self, ParseColourError<ParseIntError>> {
        let components = hex.trim().strip_prefix('#').ok_or(ParseColourError::InvalidFormat)?;
        let grey = match components.len() {
            // Short form: #G
            1 => {
                let value = u8::from_str_radix(components, 16).map_err(ParseColourError::ParseHex)?;
                // Expand short form (e.g., #F becomes #FF)
                T::from(value * 17).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap()
            }
            // Long form: #GG
            2 => {
                let value = u8::from_str_radix(components, 16).map_err(ParseColourError::ParseHex)?;
                T::from(value).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap()
            }
            _ => return Err(ParseColourError::InvalidFormat),
        };
        Ok(Self::new(grey))
    }

    fn to_hex(&self) -> String {
        let max = T::from(255_i32).unwrap();
        let grey = (self.grey * max).round().to_u8().unwrap();
        format!("#{grey:02X}")
    }

    fn from_bytes(bytes: [u8; 1]) -> Self {
        let max = T::from(255_u8).unwrap();
        let value = T::from(bytes[0]).unwrap() / max;
        Self::new(value)
    }

    fn to_bytes(self) -> [u8; 1] {
        let max = T::from(255_u8).unwrap();
        let value = (self.grey * max).round().to_u8().unwrap();
        [value]
    }

    fn lerp(lhs: &Self, rhs: &Self, t: T) -> Self {
        debug_assert!(
            t >= T::zero() && t <= T::one(),
            "Interpolation factor must be in range [0, 1]."
        );
        Self::new(lhs.grey() * (T::one() - t) + rhs.grey() * t)
    }
}

impl<T: Float + Send + Sync> Convert<T> for Grey<T> {
    fn to_grey(&self) -> Self {
        *self
    }

    fn to_grey_alpha(&self) -> GreyAlpha<T> {
        GreyAlpha::new(self.grey, T::one())
    }

    fn to_hsl(&self) -> Hsl<T> {
        // For greyscale, hue is undefined (0), saturation is 0, and lightness equals the grey value
        Hsl::new(T::zero(), T::zero(), self.grey)
    }

    fn to_hsl_alpha(&self) -> HslAlpha<T> {
        HslAlpha::new(T::zero(), T::zero(), self.grey, T::one())
    }

    fn to_hsv(&self) -> Hsv<T> {
        // For greyscale, hue is undefined (0), saturation is 0, and value equals the grey value
        Hsv::new(T::zero(), T::zero(), self.grey)
    }

    fn to_hsv_alpha(&self) -> HsvAlpha<T> {
        HsvAlpha::new(T::zero(), T::zero(), self.grey, T::one())
    }

    fn to_lab(&self) -> Lab<T> {
        // Convert Grey to Lab via XYZ
        self.to_xyz().to_lab()
    }

    fn to_lab_alpha(&self) -> LabAlpha<T> {
        let lab = self.to_lab();
        LabAlpha::new(lab.lightness(), lab.a_star(), lab.b_star(), T::one())
    }

    fn to_rgb(&self) -> Rgb<T> {
        Rgb::new(self.grey, self.grey, self.grey)
    }

    fn to_rgb_alpha(&self) -> RgbAlpha<T> {
        RgbAlpha::new(self.grey, self.grey, self.grey, T::one())
    }

    fn to_srgb(&self) -> Srgb<T> {
        let sg = Srgb::gamma_encode(self.grey);
        Srgb::new(sg, sg, sg)
    }

    fn to_srgb_alpha(&self) -> SrgbAlpha<T> {
        let sg = Srgb::gamma_encode(self.grey);
        SrgbAlpha::new(sg, sg, sg, T::one())
    }

    fn to_xyz(&self) -> Xyz<T> {
        // Grey in XYZ space with D65 reference white
        // For greyscale, X, Y, and Z values are proportional to the reference white
        // Y (luminance) equals grey value, and X and Z are scaled according to D65

        // Simplified approach: use the luminance (Y) value directly,
        // and scale X and Z based on D65 reference white
        let white = Xyz::<T>::d65_reference_white();

        // Scale all values by the grey value (luminance)
        let x = white.x() * self.grey();
        let y = self.grey(); // Y value is directly the grey value (luminance)
        let z = white.z() * self.grey();

        Xyz::new(x, y, z)
    }

    fn to_xyz_alpha(&self) -> XyzAlpha<T> {
        let xyz = self.to_xyz();
        XyzAlpha::new(xyz.x(), xyz.y(), xyz.z(), T::one())
    }
}

impl<T: Float + Send + Sync> Display for Grey<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        let max = T::from(255_i32).unwrap();
        let value = (self.grey * max).round().to_u8().unwrap();
        write!(fmt, "\x1b[38;2;{value};{value};{value}m{PRINT_BLOCK}\x1b[0m")
    }
}
