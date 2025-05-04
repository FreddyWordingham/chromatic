//! HSL (Hue, Saturation, Lightness) colour representation.

use num_traits::Float;

mod colour;
mod convert;
mod fmt;

/// HSL colour representation.
#[derive(Debug, Clone, Copy)]
pub struct Hsl<T: Float + Send + Sync> {
    /// Hue component in degrees [0, 360).
    hue: T,
    /// Saturation component [0, 1].
    saturation: T,
    /// Lightness component [0, 1].
    lightness: T,
}

impl<T: Float + Send + Sync> Hsl<T> {
    /// Create a new `Hsl` instance.
    #[inline]
    pub fn new(mut hue: T, saturation: T, lightness: T) -> Self {
        // Normalize hue to be within [0, 360).
        let hue = {
            let f360 = T::from(360.0).unwrap();
            while hue >= f360 {
                hue = hue - f360;
            }
            while hue < T::zero() {
                hue = hue + f360;
            }
            hue
        };

        debug_assert!(
            hue >= T::zero() && hue < T::from(360.0).unwrap(),
            "Hue component must be between 0 and 360."
        );
        debug_assert!(
            !(saturation < T::zero() || saturation > T::one()),
            "Saturation component must be between 0 and 1."
        );
        debug_assert!(
            !(lightness < T::zero() || lightness > T::one()),
            "Lightness component must be between 0 and 1."
        );
        Self {
            hue,
            saturation,
            lightness,
        }
    }

    /// Get the hue component in degrees [0, 360).
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

    /// Set the hue component in degrees [0, 360).
    #[inline]
    pub fn set_hue(&mut self, mut hue: T) {
        // Normalize hue to be within [0, 360)
        let f360 = T::from(360.0).unwrap();
        while hue >= f360 {
            hue = hue - f360;
        }
        while hue < T::zero() {
            hue = hue + f360;
        }

        debug_assert!(
            hue >= T::zero() && hue < T::from(360.0).unwrap(),
            "Hue component must be between 0 and 360."
        );
        self.hue = hue;
    }

    /// Set the saturation component.
    #[inline]
    pub fn set_saturation(&mut self, saturation: T) {
        debug_assert!(
            saturation >= T::zero() && saturation <= T::one(),
            "Saturation component must be between 0 and 1."
        );
        self.saturation = saturation;
    }

    /// Set the lightness component.
    #[inline]
    pub fn set_lightness(&mut self, lightness: T) {
        debug_assert!(
            lightness >= T::zero() && lightness <= T::one(),
            "Lightness component must be between 0 and 1."
        );
        self.lightness = lightness;
    }
}
