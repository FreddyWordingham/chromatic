//! Implements the `Colour` trait for `LabRgb`.

use core::{fmt::Display, ops::AddAssign};
use num_traits::Float;

use crate::{
    Colour, LabRgb,
    colours::lab_utils::{rgb_to_xyz_components, xyz_to_lab},
};

impl<T: Display + AddAssign + Float> Colour<T, 3> for LabRgb<T> {
    #[inline]
    fn from_components(components: [T; 3]) -> Self {
        Self::new(components[0], components[1], components[2])
    }

    #[inline]
    fn components(&self) -> [T; 3] {
        // Convert Lab back to RGB for the external interface
        self.rgb_components()
    }

    #[inline]
    fn set_components(&mut self, components: [T; 3]) {
        // Convert the entire RGB array to Lab at once
        let xyz = rgb_to_xyz_components(&components);
        let lab = xyz_to_lab(&xyz);
        self.lightness = lab[0];
        self.a_axis = lab[1];
        self.b_axis = lab[2];
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
        let rgb = self.rgb_components();
        let max = T::from(255_u8).unwrap();
        let red = (rgb[0] * max).round().to_u8().unwrap();
        let green = (rgb[1] * max).round().to_u8().unwrap();
        let blue = (rgb[2] * max).round().to_u8().unwrap();
        [red, green, blue]
    }

    /// Linear interpolate between two RGB colours using Lab colour space for perceptual uniformity.
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

        // Direct interpolation in Lab space - the hot path is much simpler now!
        let l = lhs.lightness * (T::one() - t) + rhs.lightness * t;
        let a = lhs.a_axis * (T::one() - t) + rhs.a_axis * t;
        let b = lhs.b_axis * (T::one() - t) + rhs.b_axis * t;

        // Create result directly in Lab space
        Self {
            lightness: l,
            a_axis: a,
            b_axis: b,
        }
    }
}
