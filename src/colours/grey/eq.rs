//! Compare two `Grey` colours for equality.

use num_traits::Float;

use crate::{Colour as _, Grey};

impl<T: Float> PartialEq for Grey<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (self.grey - other.grey).abs() <= Self::tolerance()
    }
}
