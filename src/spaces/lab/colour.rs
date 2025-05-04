//! Implements the `Colour` trait for `Lab`.

use core::num::ParseIntError;
use num_traits::Float;

use crate::{Colour, Convert, Lab, ParseColourError, Srgb};

impl<T: Float + Send + Sync> Colour<T, 3> for Lab<T> {
    #[inline]
    fn from_hex(hex: &str) -> Result<Self, ParseColourError<ParseIntError>> {
        // Convert from hex to Lab via sRGB and XYZ
        let srgb = Srgb::from_hex(hex)?;
        Ok(srgb.to_lab())
    }

    #[inline]
    fn to_hex(&self) -> String {
        // Convert to hex via sRGB
        self.to_srgb().to_hex()
    }

    #[inline]
    fn from_bytes(bytes: [u8; 3]) -> Self {
        // Convert from bytes to Lab via sRGB and XYZ
        Srgb::from_bytes(bytes).to_lab()
    }

    #[inline]
    fn to_bytes(self) -> [u8; 3] {
        // Convert to bytes via sRGB
        self.to_srgb().to_bytes()
    }

    /// Linear interpolate between two Lab colours.
    ///
    /// Lab is designed to be perceptually uniform, so linear interpolation
    /// in this space produces perceptually uniform gradients.
    #[inline]
    fn lerp(lhs: &Self, rhs: &Self, t: T) -> Self {
        debug_assert!(
            t >= T::zero() && t <= T::one(),
            "Interpolation factor must be in range [0, 1]."
        );
        Self::new(
            lhs.lightness * (T::one() - t) + rhs.lightness * t,
            lhs.a_star * (T::one() - t) + rhs.a_star * t,
            lhs.b_star * (T::one() - t) + rhs.b_star * t,
        )
    }
}
