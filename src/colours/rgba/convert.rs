//! Convert `Rgba` to other colour types.

use core::{fmt::Display, ops::AddAssign};
use num_traits::Float;

use crate::{Grey, GreyAlpha, LabRgb, LabRgba, Rgb, Rgba};

impl<T: Display + AddAssign + Float> Rgba<T> {
    /// Convert to `Grey` by averaging the RGB components.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    pub fn to_grey(&self) -> Grey<T> {
        Grey::new((self.red + self.green + self.blue) / T::from(3).unwrap())
    }

    /// Convert to `GreyAlpha` by averaging the RGB components.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    pub fn to_grey_alpha(&self) -> GreyAlpha<T> {
        GreyAlpha::new((self.red + self.green + self.blue) / T::from(3).unwrap(), self.alpha)
    }

    /// Convert to `Rgb` by discarding the alpha channel.
    #[inline]
    pub fn to_rgb(&self) -> Rgb<T> {
        Rgb::new(self.red, self.green, self.blue)
    }

    /// Convert to `LabRgb`.
    #[inline]
    pub fn to_lab_rgb(&self) -> LabRgb<T> {
        LabRgb::new(self.red, self.green, self.blue)
    }

    /// Convert to `LabRgba`.
    #[inline]
    pub fn to_lab_rgba(&self) -> LabRgba<T> {
        LabRgba::new(self.red, self.green, self.blue, self.alpha)
    }
}
