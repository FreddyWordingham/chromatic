//! Compare two `GreyAlpha` colours for equality.

use core::{fmt::Display, ops::AddAssign};
use num_traits::Float;

use crate::{Colour as _, GreyAlpha};

impl<T: Display + AddAssign + Float> PartialEq for GreyAlpha<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (self.grey - other.grey).abs() <= Self::tolerance() && (self.alpha - other.alpha).abs() <= Self::tolerance()
    }
}
