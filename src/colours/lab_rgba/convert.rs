//! Convert `LabRgba` to other colour types.

use num_traits::Float;

use crate::{Grey, GreyAlpha, LabRgb, LabRgba, Rgb, Rgba};

impl<T: Float> LabRgba<T> {
    /// Convert to `Grey`.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    pub fn to_grey(&self) -> Grey<T> {
        let [red, green, blue] = self.rgb_components();
        Grey::new((red + green + blue) / T::from(3).unwrap())
    }

    /// Convert to `GreyAlpha`.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    pub fn to_grey_alpha(&self) -> GreyAlpha<T> {
        let [red, green, blue] = self.rgb_components();
        GreyAlpha::new((red + green + blue) / T::from(3).unwrap(), self.alpha)
    }

    /// Convert to `Rgb`.
    #[inline]
    pub fn to_rgb(&self) -> Rgb<T> {
        let [red, green, blue] = self.rgb_components();
        Rgb::new(red, green, blue)
    }

    /// Convert to `Rgba`.
    #[inline]
    pub fn to_rgba(&self) -> Rgba<T> {
        let [red, green, blue] = self.rgb_components();
        Rgba::new(red, green, blue, self.alpha)
    }

    /// Convert to `LabRgb`.
    #[inline]
    pub fn to_lab_rgb(&self) -> LabRgb<T> {
        LabRgb::from_lab(self.lightness, self.a_axis, self.b_axis)
    }
}
