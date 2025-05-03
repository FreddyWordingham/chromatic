//! Compare two `Hsla` colours for equality.

use num_traits::Float;

use crate::{Colour as _, Hsla};

impl<T: Float> PartialEq for Hsla<T> {
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        // For saturation 0, hue does not matter (grayscale)
        if self.saturation <= Self::tolerance() && other.saturation <= Self::tolerance() {
            return (self.lightness - other.lightness).abs() <= Self::tolerance()
                && (self.alpha - other.alpha).abs() <= Self::tolerance();
        }

        // For very low or very high lightness, hue becomes less important
        if (self.lightness <= Self::tolerance() && other.lightness <= Self::tolerance())
            || (self.lightness >= T::one() - Self::tolerance() && other.lightness >= T::one() - Self::tolerance())
        {
            return (self.saturation - other.saturation).abs() <= Self::tolerance()
                && (self.lightness - other.lightness).abs() <= Self::tolerance()
                && (self.alpha - other.alpha).abs() <= Self::tolerance();
        }

        // For general comparison, handle the cyclic nature of hue
        let mut hue_diff = (self.hue - other.hue).abs();
        if hue_diff > T::from(180).unwrap() {
            hue_diff = T::from(360).unwrap() - hue_diff;
        }

        // Use a proportional tolerance for hue (1 degree is reasonable)
        hue_diff <= T::from(1).unwrap()
            && (self.saturation - other.saturation).abs() <= Self::tolerance()
            && (self.lightness - other.lightness).abs() <= Self::tolerance()
            && (self.alpha - other.alpha).abs() <= Self::tolerance()
    }
}
