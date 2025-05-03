//! Convert `Hsla` to other colour types.

use num_traits::Float;

use crate::{Grey, GreyAlpha, Hsl, Hsla, Hsv, Hsva, LabRgb, LabRgba, Rgb, Rgba};

impl<T: Float> Hsla<T> {
    /// Convert to `Grey`.
    ///
    /// Converts HSLA to RGB first, then averages the RGB components, discarding the alpha channel.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    pub fn to_grey(&self) -> Grey<T> {
        let (red, green, blue, _) = self.rgba_components();
        Grey::new((red + green + blue) / T::from(3).unwrap())
    }

    /// Convert to `GreyAlpha`.
    ///
    /// Converts HSLA to RGB first, then averages the RGB components, preserving the alpha channel.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    pub fn to_grey_alpha(&self) -> GreyAlpha<T> {
        let (red, green, blue, alpha) = self.rgba_components();
        GreyAlpha::new((red + green + blue) / T::from(3).unwrap(), alpha)
    }

    /// Convert to `Hsl` by discarding the alpha channel.
    #[inline]
    pub fn to_hsl(&self) -> Hsl<T> {
        Hsl::new(self.hue, self.saturation, self.lightness)
    }

    /// Convert to `Hsv`.
    #[inline]
    pub fn to_hsv(&self) -> Hsv<T> {
        self.to_hsl().to_hsv()
    }

    /// Convert to `Hsva`.
    #[inline]
    pub fn to_hsva(&self) -> Hsva<T> {
        let hsv = self.to_hsv();
        Hsva::new(hsv.hue(), hsv.saturation(), hsv.value(), self.alpha)
    }

    /// Convert to `Rgb`.
    #[inline]
    pub fn to_rgb(&self) -> Rgb<T> {
        let (red, green, blue, _) = self.rgba_components();
        Rgb::new(red, green, blue)
    }

    /// Convert to `Rgba`.
    #[inline]
    pub fn to_rgba(&self) -> Rgba<T> {
        let (red, green, blue, alpha) = self.rgba_components();
        Rgba::new(red, green, blue, alpha)
    }

    /// Convert to `LabRgb`.
    #[inline]
    pub fn to_lab_rgb(&self) -> LabRgb<T> {
        let (red, green, blue, _) = self.rgba_components();
        LabRgb::from_rgb(red, green, blue)
    }

    /// Convert to `LabRgba`.
    #[inline]
    pub fn to_lab_rgba(&self) -> LabRgba<T> {
        let (red, green, blue, alpha) = self.rgba_components();
        LabRgba::from_rgba(red, green, blue, alpha)
    }
}
