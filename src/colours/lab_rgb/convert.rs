//! Convert `LabRgb` to other colour types.

use num_traits::Float;

use crate::{Grey, GreyAlpha, Hsl, Hsla, Hsv, Hsva, LabRgb, LabRgba, Rgb, Rgba};

impl<T: Float> LabRgb<T> {
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
    pub fn to_grey_alpha(&self, alpha: T) -> GreyAlpha<T> {
        let [red, green, blue] = self.rgb_components();
        GreyAlpha::new((red + green + blue) / T::from(3).unwrap(), alpha)
    }

    /// Convert to `Hsl`.
    #[inline]
    pub fn to_hsl(&self) -> Hsl<T> {
        let [red, green, blue] = self.rgb_components();
        Hsl::from_rgb(red, green, blue)
    }

    /// Convert to `Hsla`.
    #[inline]
    pub fn to_hsla(&self, alpha: T) -> Hsla<T> {
        let [red, green, blue] = self.rgb_components();
        Hsla::from_rgba(red, green, blue, alpha)
    }

    /// Convert to `Hsv`.
    #[inline]
    pub fn to_hsv(&self) -> Hsv<T> {
        let [red, green, blue] = self.rgb_components();
        Hsv::from_rgb(red, green, blue)
    }

    /// Convert to `Hsva`.
    #[inline]
    pub fn to_hsva(&self, alpha: T) -> Hsva<T> {
        let [red, green, blue] = self.rgb_components();
        Hsva::from_rgba(red, green, blue, alpha)
    }

    /// Convert to `Rgba`.
    #[inline]
    pub fn to_rgba(&self, alpha: T) -> Rgba<T> {
        let [red, green, blue] = self.rgb_components();
        Rgba::new(red, green, blue, alpha)
    }

    /// Convert to `LabRgba`.
    #[inline]
    pub fn to_lab_rgba(&self, alpha: T) -> LabRgba<T> {
        LabRgba::new(self.lightness, self.a_axis, self.b_axis, alpha)
    }

    /// Convert to `Rgb`.
    #[inline]
    pub fn to_rgb(&self) -> Rgb<T> {
        let [red, green, blue] = self.rgb_components();
        Rgb::new(red, green, blue)
    }
}
