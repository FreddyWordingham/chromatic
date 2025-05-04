//! HSV (Hue, Saturation, Value) colour representation.

use num_traits::Float;

mod colour;
mod convert;
mod fmt;

/// HSV colour representation.
#[derive(Debug, Clone, Copy)]
pub struct Hsv<T: Float + Send + Sync> {
    /// Hue component in degrees [0, 360).
    hue: T,
    /// Saturation component [0, 1].
    saturation: T,
    /// Value component [0, 1].
    value: T,
}

impl<T: Float + Send + Sync> Hsv<T> {
    /// Create a new `Hsv` instance.
    #[inline]
    pub fn new(mut hue: T, saturation: T, value: T) -> Self {
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
            !(value < T::zero() || value > T::one()),
            "Value component must be between 0 and 1."
        );
        Self { hue, saturation, value }
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

    /// Get the value component.
    #[inline]
    pub const fn value(&self) -> T {
        self.value
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

    /// Set the value component.
    #[inline]
    pub fn set_value(&mut self, value: T) {
        debug_assert!(
            value >= T::zero() && value <= T::one(),
            "Value component must be between 0 and 1."
        );
        self.value = value;
    }
}
