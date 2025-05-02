//! Compare two `Grey` colours for equality.

use core::{fmt::Display, ops::AddAssign};
use num_traits::Float;

use crate::{Colour as _, Grey};

impl<T: Display + AddAssign + Float> PartialEq for Grey<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (self.grey - other.grey).abs() <= Self::tolerance()
    }
}
