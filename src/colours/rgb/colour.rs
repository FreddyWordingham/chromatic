//! Implements the `Colour` trait for `Rgb`.

use core::{fmt::Display, ops::AddAssign};
use num_traits::Float;

use crate::{Colour, Rgb};

impl<T: Display + AddAssign + Float> Colour<T, 3> for Rgb<T> {
    #[inline]
    fn from_components(components: [T; 3]) -> Self {
        Self::new(components[0], components[1], components[2])
    }

    #[inline]
    fn components(&self) -> [T; 3] {
        [self.red, self.green, self.blue]
    }

    #[inline]
    fn set_components(&mut self, components: [T; 3]) {
        self.set_red(components[0]);
        self.set_green(components[1]);
        self.set_blue(components[2]);
    }

    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn from_bytes(bytes: [u8; 3]) -> Self {
        let max = T::from(255_u8).unwrap();
        let red = T::from(bytes[0]).unwrap() / max;
        let green = T::from(bytes[1]).unwrap() / max;
        let blue = T::from(bytes[2]).unwrap() / max;
        Self::new(red, green, blue)
    }

    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn to_bytes(self) -> [u8; 3] {
        let max = T::from(255_u8).unwrap();
        let red = (self.red * max).round().to_u8().unwrap();
        let green = (self.green * max).round().to_u8().unwrap();
        let blue = (self.blue * max).round().to_u8().unwrap();
        [red, green, blue]
    }

    /// Linear interpolate between two RGB colours.
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
            lhs.red() * (T::one() - t) + rhs.red() * t,
            lhs.green() * (T::one() - t) + rhs.green() * t,
            lhs.blue() * (T::one() - t) + rhs.blue() * t,
        )
    }
}
