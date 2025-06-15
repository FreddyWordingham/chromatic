//! Trait implemented by all colour types.

use num_traits::Float;

use crate::{
    error::Result,
    spaces::{Grey, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz, XyzAlpha},
};

/// Types implementing this trait can be converted to various `Colour` `crate::spaces`.
pub trait Convert<T: Float + Send + Sync> {
    /// Convert a colour to the `Grey` colour space.
    fn to_grey(&self) -> Result<Grey<T>>;

    /// Convert a colour to the `GreyAlpha` colour space.
    fn to_grey_alpha(&self) -> Result<GreyAlpha<T>>;

    /// Convert a colour to the `Hsl` colour space.
    fn to_hsl(&self) -> Result<Hsl<T>>;

    /// Convert a colour to the `HslAlpha` colour space.
    fn to_hsl_alpha(&self) -> Result<HslAlpha<T>>;

    /// Convert a colour to the `Hsv` colour space.
    fn to_hsv(&self) -> Result<Hsv<T>>;

    /// Convert a colour to the `HsvAlpha` colour space.
    fn to_hsv_alpha(&self) -> Result<HsvAlpha<T>>;

    /// Convert a colour to the `Lab` colour space.
    fn to_lab(&self) -> Result<Lab<T>>;

    /// Convert a colour to the `LabAlpha` colour space.
    fn to_lab_alpha(&self) -> Result<LabAlpha<T>>;

    /// Convert a colour to the `Rgb` colour space.
    fn to_rgb(&self) -> Result<Rgb<T>>;

    /// Convert a colour to the `RgbAlpha` colour space.
    fn to_rgb_alpha(&self) -> Result<RgbAlpha<T>>;

    /// Convert a colour to the `Srgb` colour space.
    fn to_srgb(&self) -> Result<Srgb<T>>;

    /// Convert a colour to the `SrgbAlpha` colour space.
    fn to_srgb_alpha(&self) -> Result<SrgbAlpha<T>>;

    /// Convert a colour to the `Xyz` colour space.
    fn to_xyz(&self) -> Result<Xyz<T>>;

    /// Convert a colour to the `XyzAlpha` colour space.
    fn to_xyz_alpha(&self) -> Result<XyzAlpha<T>>;
}
