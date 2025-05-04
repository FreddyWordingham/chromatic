//! Implements the `Colour` trait for `Xyz`.

use num_traits::Float;
use std::num::ParseIntError;

use crate::{Colour, Convert as _, ParseColourError, Srgb, Xyz};

impl<T: Float + Send + Sync> Colour<T, 3> for Xyz<T> {
    #[inline]
    fn from_hex(hex: &str) -> Result<Self, ParseColourError<ParseIntError>> {
        // Convert from hex to XYZ via sRGB
        // First parse the hex to sRGB
        let srgb = Srgb::from_hex(hex)?;

        // Then convert sRGB to XYZ
        Ok(srgb.to_xyz())
    }

    #[inline]
    fn to_hex(&self) -> String {
        // Convert to hex via sRGB
        self.to_srgb().to_hex()
    }

    #[inline]
    fn from_bytes(bytes: [u8; 3]) -> Self {
        // Convert from bytes to XYZ via sRGB
        Srgb::from_bytes(bytes).to_xyz()
    }

    #[inline]
    fn to_bytes(self) -> [u8; 3] {
        // Convert to bytes via sRGB
        self.to_srgb().to_bytes()
    }

    /// Linear interpolate between two XYZ colours.
    /// Note: Prefer Lab for perceptually uniform interpolation.
    #[inline]
    fn lerp(lhs: &Self, rhs: &Self, t: T) -> Self {
        debug_assert!(
            t >= T::zero() && t <= T::one(),
            "Interpolation factor must be in range [0, 1]."
        );
        Self::new(
            lhs.x * (T::one() - t) + rhs.x * t,
            lhs.y * (T::one() - t) + rhs.y * t,
            lhs.z * (T::one() - t) + rhs.z * t,
        )
    }
}
