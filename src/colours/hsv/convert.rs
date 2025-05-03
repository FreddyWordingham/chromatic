//! Convert `Hsv` to other colour types.

use num_traits::Float;

use crate::{Grey, GreyAlpha, Hsv, Hsva, LabRgb, LabRgba, Rgb, Rgba};

impl<T: Float> Hsv<T> {
    /// Convert to `Grey`.
    ///
    /// Converts HSV to RGB first, then averages the RGB components.
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
    /// Converts HSV to RGB first, then averages the RGB components.
    /// Alpha is set to 1.0 (fully opaque).
    ///
    /// # Panics
    ///
    /// This function will not panic.
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    pub fn to_grey_alpha(&self) -> GreyAlpha<T> {
        let (red, green, blue) = self.rgb_components();
        GreyAlpha::new((red + green + blue) / T::from(3).unwrap(), T::one())
    }

    /// Convert to `Hsva`.
    #[inline]
    pub fn to_hsva(&self, alpha: T) -> Hsva<T> {
        Hsva::new(self.hue, self.saturation, self.value, alpha)
    }

    /// Convert to `Rgb`.
    #[inline]
    pub fn to_rgb(&self) -> Rgb<T> {
        let (red, green, blue) = self.rgb_components();
        Rgb::new(red, green, blue)
    }

    /// Convert to `Rgba`.
    ///
    /// Alpha is set to 1.0 (fully opaque).
    #[inline]
    pub fn to_rgba(&self) -> Rgba<T> {
        let (red, green, blue) = self.rgb_components();
        Rgba::new(red, green, blue, T::one())
    }

    /// Convert to `LabRgb`.
    #[inline]
    pub fn to_lab_rgb(&self) -> LabRgb<T> {
        let (red, green, blue) = self.rgb_components();
        LabRgb::new(red, green, blue)
    }

    /// Convert to `LabRgba`.
    ///
    /// Alpha is set to 1.0 (fully opaque).
    #[inline]
    pub fn to_lab_rgba(&self) -> LabRgba<T> {
        let (red, green, blue) = self.rgb_components();
        LabRgba::new(red, green, blue, T::one())
    }
}
