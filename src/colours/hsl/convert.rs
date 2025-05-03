//! Convert `Hsl` to other colour types.

use num_traits::Float;

use crate::{Grey, GreyAlpha, Hsl, Hsla, Hsv, Hsva, LabRgb, LabRgba, Rgb, Rgba};

impl<T: Float> Hsl<T> {
    /// Convert to `Grey`.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    pub fn to_grey(&self) -> Grey<T> {
        let (red, green, blue) = self.rgb_components();
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
        let (red, green, blue) = self.rgb_components();
        GreyAlpha::new((red + green + blue) / T::from(3).unwrap(), alpha)
    }

    /// Convert to `Hsla`.
    #[inline]
    pub fn to_hsla(&self, alpha: T) -> Hsla<T> {
        Hsla::new(self.hue, self.saturation, self.lightness, alpha)
    }

    /// Convert to `Hsv`.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    pub fn to_hsv(&self) -> Hsv<T> {
        // If lightness is 0, then HSV value is 0 (black)
        if self.lightness == T::zero() {
            return Hsv::new(self.hue, T::zero(), T::zero());
        }

        // If lightness is 1, then HSV value is 1 (white)
        if self.lightness == T::one() {
            return Hsv::new(self.hue, T::zero(), T::one());
        }

        // Calculate HSV value
        let value = self.lightness + self.saturation * self.lightness.min(T::one() - self.lightness);

        // Calculate HSV saturation
        let saturation = if value == T::zero() {
            T::zero()
        } else {
            T::from(2.0).unwrap() * (T::one() - self.lightness / value)
        };

        Hsv::new(self.hue, saturation, value)
    }

    /// Convert to `Hsva`.
    #[inline]
    pub fn to_hsva(&self, alpha: T) -> Hsva<T> {
        let hsv = self.to_hsv();
        Hsva::new(hsv.hue(), hsv.saturation(), hsv.value(), alpha)
    }

    /// Convert to `Rgb`.
    #[inline]
    pub fn to_rgb(&self) -> Rgb<T> {
        let (red, green, blue) = self.rgb_components();
        Rgb::new(red, green, blue)
    }

    /// Convert to `Rgba`.
    #[inline]
    pub fn to_rgba(&self, alpha: T) -> Rgba<T> {
        let (red, green, blue) = self.rgb_components();
        Rgba::new(red, green, blue, alpha)
    }

    /// Convert to `LabRgb`.
    #[inline]
    pub fn to_lab_rgb(&self) -> LabRgb<T> {
        let (red, green, blue) = self.rgb_components();
        LabRgb::from_rgb(red, green, blue)
    }

    /// Convert to `LabRgba`.
    #[inline]
    pub fn to_lab_rgba(&self, alpha: T) -> LabRgba<T> {
        let (red, green, blue) = self.rgb_components();
        LabRgba::from_rgba(red, green, blue, alpha)
    }
}
