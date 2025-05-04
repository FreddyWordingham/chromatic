//! Lab colour representation.
//!
//! The Lab colour space (also known as CIELAB) is a colour space defined by the International
//! Commission on Illumination (CIE) in 1976. It expresses colour as three values:
//! - L* for perceptual lightness (0 to 100)
//! - a* from green (-) to red (+)
//! - b* from blue (-) to yellow (+)
//!
//! Lab is designed to be perceptually uniform, meaning a change of the same amount in a value
//! should produce a change of about the same visual importance.

use num_traits::Float;

mod colour;
mod convert;
mod fmt;

/// LAB colour representation.
#[derive(Debug, Clone, Copy)]
pub struct Lab<T: Float + Send + Sync> {
    /// Lightness component in range [0, 100].
    lightness: T,
    /// a component in range [-128, 127].
    a_star: T,
    /// b component in range [-128, 127].
    b_star: T,
}

impl<T: Float + Send + Sync> Lab<T> {
    /// Create a new `Lab` instance.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    #[inline]
    pub fn new(lightness: T, a_star: T, b_star: T) -> Self {
        debug_assert!(
            lightness >= T::zero() && lightness <= T::from(100.0).unwrap(),
            "Lightness component must be between 0 and 100."
        );
        debug_assert!(
            a_star >= T::from(-128.0).unwrap() && a_star <= T::from(127.0).unwrap(),
            "a component must be between -128 and 127."
        );
        debug_assert!(
            b_star >= T::from(-128.0).unwrap() && b_star <= T::from(127.0).unwrap(),
            "b component must be between -128 and 127."
        );
        Self {
            lightness,
            a_star,
            b_star,
        }
    }

    /// Get the `lightness` component (L*).
    #[inline]
    pub const fn lightness(&self) -> T {
        self.lightness
    }

    /// Get the `a_star` component (a*).
    #[inline]
    pub const fn a_star(&self) -> T {
        self.a_star
    }

    /// Get the `b_star` component (b*).
    #[inline]
    pub const fn b_star(&self) -> T {
        self.b_star
    }

    /// Set the `lightness` component (L).
    ///
    /// # Panics
    ///
    /// This function will not panic.
    #[inline]
    pub fn set_lightness(&mut self, lightness: T) {
        debug_assert!(
            lightness >= T::zero() && lightness <= T::from(100.0).unwrap(),
            "Lightness component must be between 0 and 100."
        );
        self.lightness = lightness;
    }

    /// Set the `a_star` component.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    #[inline]
    pub fn set_a_star(&mut self, a_star: T) {
        debug_assert!(
            a_star >= T::from(-128.0).unwrap() && a_star <= T::from(127.0).unwrap(),
            "a component must be between -128 and 127."
        );
        self.a_star = a_star;
    }

    /// Set the `b_star` component.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    #[inline]
    pub fn set_b_star(&mut self, b_star: T) {
        debug_assert!(
            b_star >= T::from(-128.0).unwrap() && b_star <= T::from(127.0).unwrap(),
            "b component must be between -128 and 127."
        );
        self.b_star = b_star;
    }

    /// Calculate perceptual colour difference in Lab space (CIE76 Delta E).
    /// The Delta E value indicates how different two colours appear, with values:
    /// - < 1.0: Not perceptible by human eyes
    /// - 1-2: Perceptible through close observation
    /// - 2-10: Perceptible at a glance
    /// - > 10: Colours are more similar than opposite
    #[inline]
    pub fn delta_e(&self, other: &Self) -> T {
        let dl = self.lightness - other.lightness;
        let da = self.a_star - other.a_star;
        let db = self.b_star - other.b_star;

        (dl * dl + da * da + db * db).sqrt()
    }

    /// Calculate perceptual colour difference using the improved CIE94 Delta E formula.
    /// This is more accurate than the basic `delta_e` method, especially for saturated colours.
    ///
    /// # Panics
    ///
    /// This function will not panic.
    #[inline]
    pub fn delta_e94(&self, other: &Self) -> T {
        // Weighting factors
        let k_l = T::one();
        let k_c = T::one();
        let k_h = T::one();
        let k1 = T::from(0.045).unwrap();
        let k2 = T::from(0.015).unwrap();

        // Calculate differences
        let delta_l = self.lightness - other.lightness;

        // Calculate C1, C2 (Chroma)
        let c1 = (self.a_star * self.a_star + self.b_star * self.b_star).sqrt();
        let c2 = (other.a_star * other.a_star + other.b_star * other.b_star).sqrt();

        // Calculate delta_c (difference in Chroma)
        let delta_c = c1 - c2;

        // Calculate delta_h (difference in Hue)
        let delta_a = self.a_star - other.a_star;
        let delta_b = self.b_star - other.b_star;
        let delta_h_squared = delta_a * delta_a + delta_b * delta_b - delta_c * delta_c;
        let delta_h = if delta_h_squared.is_sign_negative() {
            T::zero()
        } else {
            delta_h_squared.sqrt()
        };

        // Calculate the S_L, S_C, S_H scaling factors
        let s_l = T::one();
        let s_c = T::one() + k1 * c1;
        let s_h = T::one() + k2 * c1;

        // Calculate the final Delta E94
        let term1 = (delta_l / (k_l * s_l)).powi(2);
        let term2 = (delta_c / (k_c * s_c)).powi(2);
        let term3 = (delta_h / (k_h * s_h)).powi(2);

        (term1 + term2 + term3).sqrt()
    }
}
