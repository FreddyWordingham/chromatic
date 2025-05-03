//! RGB colour representation with transparency using Lab colour space interpolation.

use num_traits::Float;

use crate::colours::lab_utils::{lab_to_xyz, rgb_to_xyz_components, xyz_to_lab, xyz_to_rgb_components};

mod colour;
mod convert;
mod eq;
mod fmt;

/// RGB colour representation with transparency using Lab colour space internally.
///
/// This uses the standard CIELAB color space where:
/// - L* ranges from 0 to 100 (lightness)
/// - a* ranges from -128 to +127 (green to red)
/// - b* ranges from -128 to +127 (blue to yellow)
/// - alpha ranges from 0 to 1 (transparency)
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct LabRgba<T: Float> {
    /// Lightness component (L*) in range [0, 100].
    lightness: T,
    /// A-axis component (a*) in range [-128, 127].
    a_axis: T,
    /// B-axis component (b*) in range [-128, 127].
    b_axis: T,
    /// Alpha (transparency) component in range [0, 1].
    alpha: T,
}

impl<T: Float> LabRgba<T> {
    /// Convert Lab components and alpha to RGB components.
    #[inline]
    fn rgb_components(&self) -> [T; 3] {
        // Normalize Lab values to the expected ranges for conversion functions
        let normalized_lab = [
            // lightness is already in correct range [0, 100]
            self.lightness,
            // a_axis and b_axis need to be in their proper ranges
            self.a_axis,
            self.b_axis,
        ];

        let xyz = lab_to_xyz(&normalized_lab);
        let rgb = xyz_to_rgb_components(&xyz);
        [rgb[0], rgb[1], rgb[2]]
    }
}

impl<T: Float> LabRgba<T> {
    /// Create a new `LabRgba` instance from Lab components.
    ///
    /// # Panics
    ///
    /// Panics if lightness is not in [0, 100], a_axis not in [-128, 127], b_axis not in [-128, 127],
    /// or alpha not in [0, 1].
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    pub fn new(lightness: T, a_axis: T, b_axis: T, alpha: T) -> Self {
        assert!(
            lightness >= T::zero() && lightness <= T::from(100).unwrap(),
            "Lightness component must be between 0 and 100."
        );
        assert!(
            a_axis >= T::from(-128).unwrap() && a_axis <= T::from(127).unwrap(),
            "A-axis component must be between -128 and 127."
        );
        assert!(
            b_axis >= T::from(-128).unwrap() && b_axis <= T::from(127).unwrap(),
            "B-axis component must be between -128 and 127."
        );
        assert!(
            alpha >= T::zero() && alpha <= T::one(),
            "Alpha component must be between 0 and 1."
        );
        Self {
            lightness,
            a_axis,
            b_axis,
            alpha,
        }
    }

    /// Create a new `LabRgba` instance from RGBA components.
    ///
    /// # Panics
    ///
    /// Panics if any RGB component is not in [0, 1].
    #[inline]
    pub fn from_rgba(red: T, green: T, blue: T, alpha: T) -> Self {
        assert!(!(red < T::zero() || red > T::one()), "Red component must be between 0 and 1.");
        assert!(
            !(green < T::zero() || green > T::one()),
            "Green component must be between 0 and 1."
        );
        assert!(
            !(blue < T::zero() || blue > T::one()),
            "Blue component must be between 0 and 1."
        );
        assert!(
            !(alpha < T::zero() || alpha > T::one()),
            "Alpha component must be between 0 and 1."
        );

        // Convert RGB to Lab
        let rgb = [red, green, blue];
        let xyz = rgb_to_xyz_components(&rgb);
        let lab = xyz_to_lab(&xyz);

        Self {
            lightness: lab[0],
            a_axis: lab[1],
            b_axis: lab[2],
            alpha,
        }
    }

    /// Get the red component.
    #[inline]
    pub fn red(&self) -> T {
        let rgba = self.rgb_components();
        rgba[0]
    }

    /// Get the green component.
    #[inline]
    pub fn green(&self) -> T {
        let rgba = self.rgb_components();
        rgba[1]
    }

    /// Get the blue component.
    #[inline]
    pub fn blue(&self) -> T {
        let rgba = self.rgb_components();
        rgba[2]
    }

    /// Get the alpha component.
    #[inline]
    pub const fn alpha(&self) -> T {
        self.alpha
    }

    /// Get the lightness (L*) component.
    #[inline]
    pub const fn lightness(&self) -> T {
        self.lightness
    }

    /// Get the a* component.
    #[inline]
    pub const fn a_axis(&self) -> T {
        self.a_axis
    }

    /// Get the b* component.
    #[inline]
    pub const fn b_axis(&self) -> T {
        self.b_axis
    }

    /// Set the red component.
    ///
    /// # Panics
    ///
    /// Panics if the value is not in [0, 1].
    #[inline]
    pub fn set_red(&mut self, red: T) {
        assert!(red >= T::zero() && red <= T::one(), "Red component must be between 0 and 1.");

        // Get current RGB values
        let rgb = self.rgb_components();

        // Update with new red value
        let new_rgb = [red, rgb[1], rgb[2]];

        // Convert back to Lab
        let xyz = rgb_to_xyz_components(&new_rgb);
        let lab = xyz_to_lab(&xyz);

        self.lightness = lab[0];
        self.a_axis = lab[1];
        self.b_axis = lab[2];
    }

    /// Set the green component.
    ///
    /// # Panics
    ///
    /// Panics if the value is not in [0, 1].
    #[inline]
    pub fn set_green(&mut self, green: T) {
        assert!(
            green >= T::zero() && green <= T::one(),
            "Green component must be between 0 and 1."
        );

        // Get current RGB values
        let rgb = self.rgb_components();

        // Update with new green value
        let new_rgb = [rgb[0], green, rgb[2]];

        // Convert back to Lab
        let xyz = rgb_to_xyz_components(&new_rgb);
        let lab = xyz_to_lab(&xyz);

        self.lightness = lab[0];
        self.a_axis = lab[1];
        self.b_axis = lab[2];
    }

    /// Set the blue component.
    ///
    /// # Panics
    ///
    /// Panics if the value is not in [0, 1].
    #[inline]
    pub fn set_blue(&mut self, blue: T) {
        assert!(
            blue >= T::zero() && blue <= T::one(),
            "Blue component must be between 0 and 1."
        );

        // Get current RGB values
        let rgb = self.rgb_components();

        // Update with new blue value
        let new_rgb = [rgb[0], rgb[1], blue];

        // Convert back to Lab
        let xyz = rgb_to_xyz_components(&new_rgb);
        let lab = xyz_to_lab(&xyz);

        self.lightness = lab[0];
        self.a_axis = lab[1];
        self.b_axis = lab[2];
    }

    /// Set the alpha component.
    ///
    /// # Panics
    ///
    /// Panics if the value is not in [0, 1].
    #[inline]
    pub fn set_alpha(&mut self, alpha: T) {
        assert!(
            alpha >= T::zero() && alpha <= T::one(),
            "Alpha component must be between 0 and 1."
        );
        self.alpha = alpha;
    }

    /// Set the lightness (L*) component.
    ///
    /// # Panics
    ///
    /// Panics if the value is not in [0, 100].
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    pub fn set_lightness(&mut self, lightness: T) {
        assert!(
            lightness >= T::zero() && lightness <= T::from(100).unwrap(),
            "Lightness component must be between 0 and 100."
        );
        self.lightness = lightness;
    }

    /// Set the a* component.
    ///
    /// # Panics
    ///
    /// Panics if the value is not in [-128, 127].
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    pub fn set_a_axis(&mut self, a_axis: T) {
        assert!(
            a_axis >= T::from(-128).unwrap() && a_axis <= T::from(127).unwrap(),
            "A-axis component must be between -128 and 127."
        );
        self.a_axis = a_axis;
    }

    /// Set the b* component.
    ///
    /// # Panics
    ///
    /// Panics if the value is not in [-128, 127].
    #[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
    #[inline]
    pub fn set_b_axis(&mut self, b_axis: T) {
        assert!(
            b_axis >= T::from(-128).unwrap() && b_axis <= T::from(127).unwrap(),
            "B-axis component must be between -128 and 127."
        );
        self.b_axis = b_axis;
    }
}
