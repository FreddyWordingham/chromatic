//! Convert `Grey` to other colour types.

use core::{fmt::Display, ops::AddAssign};
use num_traits::Float;

use crate::{Grey, GreyAlpha, LabRgb, LabRgba, Rgb, Rgba};

impl<T: Display + AddAssign + Float> Grey<T> {
    /// Convert to `GreyAlpha`.
    #[inline]
    pub fn to_grey_alpha(&self, alpha: T) -> GreyAlpha<T> {
        GreyAlpha::new(self.grey, alpha)
    }

    /// Convert to `Rgb`.
    #[inline]
    pub fn to_rgb(&self) -> Rgb<T> {
        Rgb::new(self.grey, self.grey, self.grey)
    }

    /// Convert to `Rgba`.
    #[inline]
    pub fn to_rgba(&self, alpha: T) -> Rgba<T> {
        Rgba::new(self.grey, self.grey, self.grey, alpha)
    }

    /// Convert to `LabRgb`.
    #[inline]
    pub fn to_lab_rgb(&self) -> LabRgb<T> {
        LabRgb::new(self.grey, self.grey, self.grey)
    }

    /// Convert to `LabRgba`.
    #[inline]
    pub fn to_lab_rgba(&self, alpha: T) -> LabRgba<T> {
        LabRgba::new(self.grey, self.grey, self.grey, alpha)
    }
}
