//! Implements the `Colour` trait for `Rgba`.

use core::{fmt::Display, ops::AddAssign};
use num_traits::Float;

use crate::{Colour, Rgba};

impl<T: Display + AddAssign + Float> Colour<T, 4> for Rgba<T> {
    #[inline]
    fn from_components(components: [T; 4]) -> Self {
        Self::new(components[0], components[1], components[2], components[3])
    }

    #[inline]
    fn components(&self) -> [T; 4] {
        [self.red, self.green, self.blue, self.alpha]
    }

    #[inline]
    fn set_components(&mut self, components: [T; 4]) {
        self.set_red(components[0]);
        self.set_green(components[1]);
        self.set_blue(components[2]);
        self.set_alpha(components[3]);
    }

    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn from_bytes(bytes: [u8; 4]) -> Self {
        let max = T::from(255_u8).unwrap();
        let red = T::from(bytes[0]).unwrap() / max;
        let green = T::from(bytes[1]).unwrap() / max;
        let blue = T::from(bytes[2]).unwrap() / max;
        let alpha = T::from(bytes[3]).unwrap() / max;
        Self::new(red, green, blue, alpha)
    }

    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    fn to_bytes(self) -> [u8; 4] {
        let max = T::from(255_u8).unwrap();
        let red = (self.red * max).round().to_u8().unwrap();
        let green = (self.green * max).round().to_u8().unwrap();
        let blue = (self.blue * max).round().to_u8().unwrap();
        let alpha = (self.alpha * max).round().to_u8().unwrap();
        [red, green, blue, alpha]
    }

    /// Linear interpolate between two `Rgba` colours.
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
            lhs.alpha() * (T::one() - t) + rhs.alpha() * t,
        )
    }
}
