//! Compare two `Hsva` colours for equality.

use num_traits::Float;

use crate::{Colour as _, Hsva};

impl<T: Float> PartialEq for Hsva<T> {
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        // For saturation 0, hue does not matter
        if self.saturation <= Self::tolerance() && other.saturation <= Self::tolerance() {
            return (self.value - other.value).abs() <= Self::tolerance()
                && (self.alpha - other.alpha).abs() <= Self::tolerance();
        }

        // For very low or very high values, hue becomes less important
        if (self.value <= Self::tolerance() && other.value <= Self::tolerance())
            || (self.value >= T::one() - Self::tolerance() && other.value >= T::one() - Self::tolerance())
        {
            return (self.saturation - other.saturation).abs() <= Self::tolerance()
                && (self.value - other.value).abs() <= Self::tolerance()
                && (self.alpha - other.alpha).abs() <= Self::tolerance();
        }

        // For general comparison, we need to handle the cyclic nature of hue
        let mut hue_diff = (self.hue - other.hue).abs();
        if hue_diff > T::from(180).unwrap() {
            hue_diff = T::from(360).unwrap() - hue_diff;
        }

        // Use a proportional tolerance for hue (1 degree is reasonable)
        hue_diff <= T::from(1).unwrap()
            && (self.saturation - other.saturation).abs() <= Self::tolerance()
            && (self.value - other.value).abs() <= Self::tolerance()
            && (self.alpha - other.alpha).abs() <= Self::tolerance()
    }
}
