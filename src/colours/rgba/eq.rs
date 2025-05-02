//! Compare two `Rgba` colours for equality.

use num_traits::Float;

use crate::{Colour as _, Rgba};

impl<T: Float> PartialEq for Rgba<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (self.red - other.red).abs() <= Self::tolerance()
            && (self.green - other.green).abs() <= Self::tolerance()
            && (self.blue - other.blue).abs() <= Self::tolerance()
            && (self.alpha - other.alpha).abs() <= Self::tolerance()
    }
}
