//! Implements the `Colour` trait for `Hsl`.

use num_traits::Float;
use std::num::ParseIntError;

use crate::{Colour, Convert, Hsl, ParseColourError, Rgb};

impl<T: Float + Send + Sync> Colour<T, 3> for Hsl<T> {
    #[inline]
    fn from_hex(hex: &str) -> Result<Self, ParseColourError<ParseIntError>> {
        Ok(Rgb::from_hex(hex)?.to_hsl())
    }

    #[inline]
    fn to_hex(&self) -> String {
        self.to_rgb().to_hex()
    }

    #[inline]
    fn from_bytes(bytes: [u8; 3]) -> Self {
        Rgb::from_bytes(bytes).to_hsl()
    }

    #[inline]
    fn to_bytes(self) -> [u8; 3] {
        self.to_rgb().to_bytes()
    }

    /// Linear interpolate between two HSL colours.
    /// This uses the shortest path around the hue circle for interpolation.
    #[inline]
    fn lerp(lhs: &Self, rhs: &Self, t: T) -> Self {
        debug_assert!(
            t >= T::zero() && t <= T::one(),
            "Interpolation factor must be in range [0, 1]."
        );

        // For hue, we need special handling to ensure we take the shortest path around the color wheel
        let mut hue_diff = rhs.hue - lhs.hue;

        // If the difference is greater than 180 degrees, it's shorter to go the other way around the color wheel
        if hue_diff > T::from(180).unwrap() {
            hue_diff = hue_diff - T::from(360).unwrap();
        } else if hue_diff < T::from(-180).unwrap() {
            hue_diff = hue_diff + T::from(360).unwrap();
        }

        // Calculate the interpolated hue and ensure it stays in [0, 360] range
        let mut hue = lhs.hue + t * hue_diff;
        if hue < T::zero() {
            hue = hue + T::from(360).unwrap();
        } else if hue > T::from(360).unwrap() {
            hue = hue - T::from(360).unwrap();
        }

        // Linear interpolation for saturation and lightness
        let saturation = lhs.saturation * (T::one() - t) + rhs.saturation * t;
        let lightness = lhs.lightness * (T::one() - t) + rhs.lightness * t;

        Self::new(hue, saturation, lightness)
    }
}
