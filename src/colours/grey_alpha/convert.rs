//! Convert `GreyAlpha` to other colour types.

use num_traits::Float;

use crate::{Grey, GreyAlpha, Hsv, Hsva, LabRgb, LabRgba, Rgb, Rgba};

impl<T: Float> GreyAlpha<T> {
    /// Convert to `Grey`.
    #[inline]
    pub fn to_grey(&self) -> Grey<T> {
        Grey::new(self.grey())
    }

    /// Convert to `Hsv`.
    #[inline]
    pub fn to_hsv(&self) -> Hsv<T> {
        Hsv::new(T::zero(), T::zero(), self.grey())
    }

    /// Convert to `Hsva`.
    #[inline]
    pub fn to_hsva(&self, alpha: T) -> Hsva<T> {
        Hsva::new(T::zero(), T::zero(), self.grey(), alpha)
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
}
