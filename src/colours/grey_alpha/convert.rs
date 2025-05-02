//! Convert `GreyAlpha` to other colour types.

use num_traits::Float;

use crate::{GreyAlpha, LabRgb, LabRgba, Rgb, Rgba};

impl<T: Float> GreyAlpha<T> {
    /// Convert to `Rgb`.
    #[inline]
    pub fn to_rgb(&self) -> Rgb<T> {
        Rgb::new(self.grey, self.grey, self.grey)
    }

    /// Convert to `Rgba`.
    #[inline]
    pub fn to_rgba(&self) -> Rgba<T> {
        Rgba::new(self.grey, self.grey, self.grey, self.alpha)
    }

    /// Convert to `LabRgb`.
    #[inline]
    pub fn to_lab_rgb(&self) -> LabRgb<T> {
        LabRgb::new(self.grey, self.grey, self.grey)
    }

    /// Convert to `LabRgba`.
    #[inline]
    pub fn to_lab_rgba(&self) -> LabRgba<T> {
        LabRgba::new(self.grey, self.grey, self.grey, self.alpha)
    }
}
