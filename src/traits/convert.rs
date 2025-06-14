//! Trait implemented by all colour types.

use num_traits::Float;

use crate::{Grey, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz, XyzAlpha};

pub trait Convert<T: Float + Send + Sync> {
    fn to_grey(&self) -> Grey<T>;
    fn to_grey_alpha(&self) -> GreyAlpha<T>;
    fn to_hsl(&self) -> Hsl<T>;
    fn to_hsl_alpha(&self) -> HslAlpha<T>;
    fn to_hsv(&self) -> Hsv<T>;
    fn to_hsv_alpha(&self) -> HsvAlpha<T>;
    fn to_lab(&self) -> Lab<T>;
    fn to_lab_alpha(&self) -> LabAlpha<T>;
    fn to_rgb(&self) -> Rgb<T>;
    fn to_rgb_alpha(&self) -> RgbAlpha<T>;
    fn to_srgb(&self) -> Srgb<T>;
    fn to_srgb_alpha(&self) -> SrgbAlpha<T>;
    fn to_xyz(&self) -> Xyz<T>;
    fn to_xyz_alpha(&self) -> XyzAlpha<T>;
}
