//! Convert `Hsva` to other colour types.

use num_traits::Float;

use crate::{Grey, GreyAlpha, Hsl, Hsla, Hsv, Hsva, LabRgb, LabRgba, Rgb, Rgba};

impl<T: Float> Hsva<T> {
    /// Convert to `Grey`.
    ///
    /// Converts HSVA to RGBA first, then averages the RGB components.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    pub fn to_grey(&self) -> Grey<T> {
        let (red, green, blue, _) = self.rgb_components();
        Grey::new((red + green + blue) / T::from(3).unwrap())
    }

    /// Convert to `GreyAlpha`.
    ///
    /// Converts HSVA to RGBA first, then averages the RGB components.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    pub fn to_grey_alpha(&self) -> GreyAlpha<T> {
        let (red, green, blue, alpha) = self.rgb_components();
        GreyAlpha::new((red + green + blue) / T::from(3).unwrap(), alpha)
    }

    /// Convert to `Hsl`.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    pub fn to_hsl(&self) -> Hsl<T> {
        if self.value == T::zero() {
            return Hsl::new(self.hue, T::zero(), T::zero());
        }
        let lightness = self.value * (T::from(2).unwrap() - self.saturation) / T::from(2).unwrap();
        let saturation = if lightness == T::zero() || lightness == T::one() {
            T::zero()
        } else {
            (self.value - lightness) / lightness.min(T::one() - lightness)
        };
        Hsl::new(self.hue, saturation, lightness)
    }

    /// Convert to `Hsla`.
    #[inline]
    pub fn to_hsla(&self) -> Hsla<T> {
        let hsl = self.to_hsl();
        Hsla::new(hsl.hue(), hsl.saturation(), hsl.lightness(), self.alpha)
    }

    /// Convert to `Hsv` by discarding the alpha component.
    #[inline]
    pub fn to_hsv(&self) -> Hsv<T> {
        Hsv::new(self.hue, self.saturation, self.value)
    }

    /// Convert to `Rgb`.
    #[inline]
    pub fn to_rgb(&self) -> Rgb<T> {
        let (red, green, blue, _) = self.rgb_components();
        Rgb::new(red, green, blue)
    }

    /// Convert to `Rgba`.
    #[inline]
    pub fn to_rgba(&self) -> Rgba<T> {
        let (red, green, blue, alpha) = self.rgb_components();
        Rgba::new(red, green, blue, alpha)
    }

    /// Convert to `LabRgb`.
    #[inline]
    pub fn to_lab_rgb(&self) -> LabRgb<T> {
        let (red, green, blue, _) = self.rgb_components();
        LabRgb::from_rgb(red, green, blue)
    }

    /// Convert to `LabRgba`.
    #[inline]
    pub fn to_lab_rgba(&self) -> LabRgba<T> {
        let (red, green, blue, alpha) = self.rgb_components();
        LabRgba::from_rgba(red, green, blue, alpha)
    }
}
