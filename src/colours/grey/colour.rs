//! Implements the `Colour` trait for `Grey`.

use core::{fmt::Display, ops::AddAssign};
use num_traits::Float;

use crate::{Colour, Grey};

impl<T: Display + AddAssign + Float> Colour<T, 1> for Grey<T> {
    #[inline]
    fn from_components(components: [T; 1]) -> Self {
        Self::new(components[0])
    }

    #[inline]
    fn components(&self) -> [T; 1] {
        [self.grey]
    }

    #[inline]
    fn set_components(&mut self, components: [T; 1]) {
        self.set_grey(components[0]);
    }

    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn from_bytes(bytes: [u8; 1]) -> Self {
        let max = T::from(255_u8).unwrap();
        let value = T::from(bytes[0]).unwrap() / max;
        Self::new(value)
    }

    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn to_bytes(self) -> [u8; 1] {
        let max = T::from(255_u8).unwrap();
        let value = (self.grey * max).round().to_u8().unwrap();
        [value]
    }

    /// Linear interpolate between two greys.
    ///
    /// # Panics
    ///
    /// Panics if the interpolation factor is not in [0, 1].
    #[expect(
        clippy::min_ident_chars,
        reason = "The variable `t` for an interpolation factor is idiomatic."
    )]
    #[inline]
    fn lerp(lhs: &Self, rhs: &Self, t: T) -> Self {
        assert!(t >= T::zero() && t <= T::one(), "Interpolation factor {t} out of [0, 1].");
        Self::new(lhs.grey() * (T::one() - t) + rhs.grey() * t)
    }
}
