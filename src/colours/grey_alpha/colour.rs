//! Implements the `Colour` trait for `GreyAlpha`.

use core::{fmt::Display, ops::AddAssign};
use num_traits::Float;

use crate::{Colour, GreyAlpha};

impl<T: Display + AddAssign + Float> Colour<T, 2> for GreyAlpha<T> {
    #[inline]
    fn from_components(components: [T; 2]) -> Self {
        Self::new(components[0], components[1])
    }

    #[inline]
    fn components(&self) -> [T; 2] {
        [self.grey, self.alpha]
    }

    #[inline]
    fn set_components(&mut self, components: [T; 2]) {
        self.set_grey(components[0]);
        self.set_alpha(components[1]);
    }

    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn from_bytes(bytes: [u8; 2]) -> Self {
        let max = T::from(255_u8).unwrap();
        let grey = T::from(bytes[0]).unwrap() / max;
        let alpha = T::from(bytes[1]).unwrap() / max;
        Self::new(grey, alpha)
    }

    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn to_bytes(self) -> [u8; 2] {
        let max = T::from(255_u8).unwrap();
        let grey = (self.grey * max).round().to_u8().unwrap();
        let alpha = (self.alpha * max).round().to_u8().unwrap();
        [grey, alpha]
    }

    /// Linear interpolate between two `GreyAlpha`s.
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
        Self::new(
            lhs.grey() * (T::one() - t) + rhs.grey() * t,
            lhs.alpha() * (T::one() - t) + rhs.alpha() * t,
        )
    }
}
