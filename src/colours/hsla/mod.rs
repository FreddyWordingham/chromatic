//! HSL colour representation with transparency.

use num_traits::Float;

mod colour;
mod convert;
mod eq;
mod fmt;

/// HSL (Hue, Saturation, Lightness) colour representation with transparency.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Hsla<T: Float> {
    /// Hue component in range [0, 360], measured in degrees around the color wheel.
    /// 0 and 360 both represent red, 120 is green, 240 is blue.
    hue: T,
    /// Saturation component in range [0, 1], where 0 is grayscale and 1 is fully saturated.
    saturation: T,
    /// Lightness component in range [0, 1], where 0 is black, 0.5 is full color, and 1 is white.
    lightness: T,
    /// Alpha (transparency) component in range [0, 1].
    alpha: T,
}

impl<T: Float> Hsla<T> {
    /// Convert HSLA to RGBA components.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    #[inline]
    pub fn rgba_components(&self) -> (T, T, T, T) {
        // Delegate to HSL for the RGB conversion
        let hsl = crate::Hsl::new(self.hue, self.saturation, self.lightness);
        let (red, green, blue) = hsl.rgb_components();
        (red, green, blue, self.alpha)
    }
}

impl<T: Float> Hsla<T> {
    /// Create a new `Hsla` instance.
    ///
    /// # Panics
    ///
    /// Panics if hue is not in [0, 360] or if saturation, lightness, or alpha is not in [0, 1].
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    pub fn new(hue: T, saturation: T, lightness: T, alpha: T) -> Self {
        assert!(
            !(hue < T::zero() || hue > T::from(360).unwrap()),
            "Hue component must be between 0 and 360."
        );
        assert!(
            !(saturation < T::zero() || saturation > T::one()),
            "Saturation component must be between 0 and 1."
        );
        assert!(
            !(lightness < T::zero() || lightness > T::one()),
            "Lightness component must be between 0 and 1."
        );
        assert!(
            !(alpha < T::zero() || alpha > T::one()),
            "Alpha component must be between 0 and 1."
        );
        Self {
            hue,
            saturation,
            lightness,
            alpha,
        }
    }

    /// Convert RGBA components to HSLA.
    ///
    /// # Panics
    ///
    /// Function will not panic.
    #[inline]
    pub fn from_rgba(red: T, green: T, blue: T, alpha: T) -> Self {
        let hsl = crate::Hsl::from_rgb(red, green, blue);
        Self::new(hsl.hue(), hsl.saturation(), hsl.lightness(), alpha)
    }

    /// Get the hue component.
    #[inline]
    pub const fn hue(&self) -> T {
        self.hue
    }

    /// Get the saturation component.
    #[inline]
    pub const fn saturation(&self) -> T {
        self.saturation
    }

    /// Get the lightness component.
    #[inline]
    pub const fn lightness(&self) -> T {
        self.lightness
    }

    /// Get the alpha component.
    #[inline]
    pub const fn alpha(&self) -> T {
        self.alpha
    }

    /// Set the hue component.
    ///
    /// # Panics
    ///
    /// Panics if the value is not in [0, 360].
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    pub fn set_hue(&mut self, hue: T) {
        assert!(
            hue >= T::zero() && hue <= T::from(360).unwrap(),
            "Hue component must be between 0 and 360."
        );
        self.hue = hue;
    }

    /// Set the saturation component.
    ///
    /// # Panics
    ///
    /// Panics if the value is not in [0, 1].
    #[inline]
    pub fn set_saturation(&mut self, saturation: T) {
        assert!(
            saturation >= T::zero() && saturation <= T::one(),
            "Saturation component must be between 0 and 1."
        );
        self.saturation = saturation;
    }

    /// Set the lightness component.
    ///
    /// # Panics
    ///
    /// Panics if the value is not in [0, 1].
    #[inline]
    pub fn set_lightness(&mut self, lightness: T) {
        assert!(
            lightness >= T::zero() && lightness <= T::one(),
            "Lightness component must be between 0 and 1."
        );
        self.lightness = lightness;
    }

    /// Set the alpha component.
    ///
    /// # Panics
    ///
    /// Panics if the value is not in [0, 1].
    #[inline]
    pub fn set_alpha(&mut self, alpha: T) {
        assert!(
            alpha >= T::zero() && alpha <= T::one(),
            "Alpha component must be between 0 and 1."
        );
        self.alpha = alpha;
    }
}
