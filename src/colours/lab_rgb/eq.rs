//! Compare two `LabRgb` colours for equality.

use core::{fmt::Display, ops::AddAssign};
use num_traits::Float;

use crate::{Colour as _, LabRgb};

impl<T: Display + AddAssign + Float> PartialEq for LabRgb<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        // Compare Lab components directly
        (self.lightness - other.lightness).abs() <= Self::tolerance()
            && (self.a_axis - other.a_axis).abs() <= Self::tolerance()
            && (self.b_axis - other.b_axis).abs() <= Self::tolerance()
    }
}
