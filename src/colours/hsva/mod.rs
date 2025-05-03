//! HSV colour representation with transparency.

use num_traits::Float;

mod colour;
mod convert;
mod eq;
mod fmt;

/// HSV (Hue, Saturation, Value, Alpha) colour representation.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Hsva<T: Float> {
    /// Hue component in range [0, 360], measured in degrees around the color wheel.
    /// 0 and 360 both represent red, 120 is green, 240 is blue.
    hue: T,
    /// Saturation component in range [0, 1], where 0 is grayscale and 1 is fully saturated.
    saturation: T,
    /// Value component in range [0, 1], where 0 is black and 1 is full brightness.
    value: T,
    /// Alpha (transparency) component in range [0, 1], where 0 is completely transparent and 1 is completely opaque.
    alpha: T,
}

impl<T: Float> Hsva<T> {
    /// Convert HSV to RGB components.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    #[expect(
        clippy::min_ident_chars,
        reason = "Variables `h`, `p`, `q`, and `t` are used in mathematical calculations."
    )]
    #[expect(clippy::unreachable, reason = "Unreachable due to modulo operation.")]
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    pub fn rgb_components(&self) -> (T, T, T, T) {
        if self.saturation == T::zero() {
            // If saturation is 0, the color is a shade of gray
            return (self.value, self.value, self.value, self.alpha);
        }

        // First normalize hue to [0, 6)
        let h = (self.hue / T::from(60).unwrap()) % T::from(6).unwrap();
        let sector = h.floor().to_u32().unwrap();
        let fractional = h - h.floor();

        // Calculate helper values
        let p = self.value * (T::one() - self.saturation);
        let q = self.value * (T::one() - self.saturation * fractional);
        let t = self.value * (T::one() - self.saturation * (T::one() - fractional));

        // Apply the correct formula based on the sector
        match sector {
            0 => (self.value, t, p, self.alpha),
            1 => (q, self.value, p, self.alpha),
            2 => (p, self.value, t, self.alpha),
            3 => (p, q, self.value, self.alpha),
            4 => (t, p, self.value, self.alpha),
            5 => (self.value, p, q, self.alpha),
            _ => unreachable!(), // Due to the modulo 6, this should never happen
        }
    }
}

impl<T: Float> Hsva<T> {
    /// Create a new `Hsva` instance.
    ///
    /// # Panics
    ///
    /// Panics if hue is not in [0, 360] or if saturation, value, or alpha is not in [0, 1].
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    pub fn new(hue: T, saturation: T, value: T, alpha: T) -> Self {
        assert!(
            !(hue < T::zero() || hue > T::from(360).unwrap()),
            "Hue component must be between 0 and 360."
        );
        assert!(
            !(saturation < T::zero() || saturation > T::one()),
            "Saturation component must be between 0 and 1."
        );
        assert!(
            !(value < T::zero() || value > T::one()),
            "Value component must be between 0 and 1."
        );
        assert!(
            !(alpha < T::zero() || alpha > T::one()),
            "Alpha component must be between 0 and 1."
        );
        Self {
            hue,
            saturation,
            value,
            alpha,
        }
    }

    /// Convert RGB components to HSVA.
    ///
    /// # Panics
    ///
    /// Function will not panic.
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[expect(clippy::min_ident_chars, reason = "The variable `h` for hue is idiomatic.")]
    #[inline]
    pub fn from_rgba(red: T, green: T, blue: T, alpha: T) -> Self {
        let max_val = red.max(green).max(blue);
        let min_val = red.min(green).min(blue);
        let delta = max_val - min_val;

        // Calculate hue
        let hue = if delta == T::zero() {
            T::zero() // Undefined, default to 0
        } else if max_val == red {
            let mut h = (green - blue) / delta;
            if h < T::zero() {
                h = h + T::from(6).unwrap();
            }
            h * T::from(60).unwrap()
        } else if max_val == green {
            ((blue - red) / delta + T::from(2).unwrap()) * T::from(60).unwrap()
        } else {
            ((red - green) / delta + T::from(4).unwrap()) * T::from(60).unwrap()
        };

        // Calculate saturation
        let saturation = if max_val == T::zero() { T::zero() } else { delta / max_val };

        // Value is simply the maximum of R, G, B
        let value = max_val;

        Self::new(hue, saturation, value, alpha)
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

    /// Get the value component.
    #[inline]
    pub const fn value(&self) -> T {
        self.value
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

    /// Set the value component.
    ///
    /// # Panics
    ///
    /// Panics if the value is not in [0, 1].
    #[inline]
    pub fn set_value(&mut self, value: T) {
        assert!(
            value >= T::zero() && value <= T::one(),
            "Value component must be between 0 and 1."
        );
        self.value = value;
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
