//! Compare two `LabRgba` colours for equality.

use num_traits::Float;

use crate::{Colour as _, LabRgba};

impl<T: Float> PartialEq for LabRgba<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        // Compare Lab components directly
        (self.lightness - other.lightness).abs() <= Self::tolerance()
            && (self.a_axis - other.a_axis).abs() <= Self::tolerance()
            && (self.b_axis - other.b_axis).abs() <= Self::tolerance()
            && (self.alpha - other.alpha).abs() <= Self::tolerance()
    }
}
