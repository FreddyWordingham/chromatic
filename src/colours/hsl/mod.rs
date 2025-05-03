//! HSL colour representation.

use num_traits::Float;

mod colour;
mod convert;
mod eq;
mod fmt;

/// HSL (Hue, Saturation, Lightness) colour representation.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Hsl<T: Float> {
    /// Hue component in range [0, 360], measured in degrees around the color wheel.
    /// 0 and 360 both represent red, 120 is green, 240 is blue.
    hue: T,
    /// Saturation component in range [0, 1], where 0 is grayscale and 1 is fully saturated.
    saturation: T,
    /// Lightness component in range [0, 1], where 0 is black, 0.5 is full color, and 1 is white.
    lightness: T,
}

impl<T: Float> Hsl<T> {
    /// Convert HSL to RGB components.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    #[expect(clippy::min_ident_chars, reason = "Variables are used in mathematical calculations.")]
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    pub fn rgb_components(&self) -> (T, T, T) {
        // If saturation is 0, the color is a shade of gray
        if self.saturation == T::zero() {
            return (self.lightness, self.lightness, self.lightness);
        }

        // Helper function to convert hue to RGB
        let hue_to_rgb = |n1: T, n2: T, h: T| -> T {
            // Normalize hue to [0, 1]
            let hue = if h < T::zero() {
                h + T::one()
            } else if h > T::one() {
                h - T::one()
            } else {
                h
            };

            // Apply the conversion formula
            if hue < T::from(1.0 / 6.0).unwrap() {
                n1 + (n2 - n1) * T::from(6.0).unwrap() * hue
            } else if hue < T::from(0.5).unwrap() {
                n2
            } else if hue < T::from(2.0 / 3.0).unwrap() {
                n1 + (n2 - n1) * (T::from(2.0 / 3.0).unwrap() - hue) * T::from(6.0).unwrap()
            } else {
                n1
            }
        };

        // Calculate helper values
        let n2 = if self.lightness <= T::from(0.5).unwrap() {
            self.lightness * (T::one() + self.saturation)
        } else {
            self.lightness + self.saturation - self.lightness * self.saturation
        };

        let n1 = T::from(2.0).unwrap() * self.lightness - n2;

        // Convert hue to [0, 1] range for the conversion function
        let h = self.hue / T::from(360.0).unwrap();

        // Calculate RGB components
        let r = hue_to_rgb(n1, n2, h + T::from(1.0 / 3.0).unwrap());
        let g = hue_to_rgb(n1, n2, h);
        let b = hue_to_rgb(n1, n2, h - T::from(1.0 / 3.0).unwrap());

        (r, g, b)
    }
}

impl<T: Float> Hsl<T> {
    /// Create a new `Hsl` instance.
    ///
    /// # Panics
    ///
    /// Panics if hue is not in [0, 360] or if saturation or lightness is not in [0, 1].
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    pub fn new(hue: T, saturation: T, lightness: T) -> Self {
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
        Self {
            hue,
            saturation,
            lightness,
        }
    }

    /// Convert RGB components to HSL.
    ///
    /// # Panics
    ///
    /// Function will not panic.
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[expect(clippy::min_ident_chars, reason = "The variable `h` for hue is idiomatic.")]
    #[inline]
    pub fn from_rgb(red: T, green: T, blue: T) -> Self {
        let max_val = red.max(green).max(blue);
        let min_val = red.min(green).min(blue);
        let delta = max_val - min_val;

        // Calculate lightness
        let lightness = (max_val + min_val) / T::from(2.0).unwrap();

        // Calculate hue
        let hue = if delta == T::zero() {
            T::zero() // Undefined, default to 0
        } else if max_val == red {
            let mut h = (green - blue) / delta;
            if h < T::zero() {
                h = h + T::from(6.0).unwrap();
            }
            h * T::from(60.0).unwrap()
        } else if max_val == green {
            ((blue - red) / delta + T::from(2.0).unwrap()) * T::from(60.0).unwrap()
        } else {
            ((red - green) / delta + T::from(4.0).unwrap()) * T::from(60.0).unwrap()
        };

        // Calculate saturation
        let saturation = if lightness == T::zero() || lightness == T::one() {
            T::zero()
        } else if lightness <= T::from(0.5).unwrap() {
            delta / (max_val + min_val)
        } else {
            delta / (T::from(2.0).unwrap() - max_val - min_val)
        };

        Self::new(hue, saturation, lightness)
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
}
