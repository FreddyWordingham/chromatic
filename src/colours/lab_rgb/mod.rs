//! RGB colour representation using Lab colour space interpolation.

use num_traits::Float;

use crate::colours::lab_utils::{lab_to_xyz, rgb_to_xyz_components, xyz_to_lab, xyz_to_rgb_components};

mod colour;
mod convert;
mod eq;
mod fmt;

/// RGB colour representation using Lab colour space internally.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct LabRgb<T: Float> {
    /// Lightness component (L*).
    lightness: T,
    /// A-axis component (a*).
    a_axis: T,
    /// B-axis component (b*).
    b_axis: T,
}

impl<T: Float> LabRgb<T> {
    /// Convert Lab components to RGB
    #[inline]
    fn rgb_components(&self) -> [T; 3] {
        let lab = [self.lightness, self.a_axis, self.b_axis];
        let xyz = lab_to_xyz(&lab);
        xyz_to_rgb_components(&xyz)
    }
}

impl<T: Float> LabRgb<T> {
    /// Create a new `LabRgb` instance from Lab components.
    ///
    /// # Panics
    ///
    /// Panics if any component is not in [0, 1].
    #[inline]
    pub fn new(lightness: T, a_axis: T, b_axis: T) -> Self {
        assert!(
            lightness >= T::zero() && lightness <= T::one(),
            "Lightness component must be between 0 and 1."
        );
        assert!(
            a_axis >= T::zero() && a_axis <= T::one(),
            "A-axis component must be between 0 and 1."
        );
        assert!(
            b_axis >= T::zero() && b_axis <= T::one(),
            "B-axis component must be between 0 and 1."
        );
        Self {
            lightness,
            a_axis,
            b_axis,
        }
    }

    /// Create a new `LabRgb` instance.
    ///
    /// # Panics
    ///
    /// Panics if any component is not in [0, 1].
    #[inline]
    pub fn from_rgb(red: T, green: T, blue: T) -> Self {
        assert!(!(red < T::zero() || red > T::one()), "Red component must be between 0 and 1.");
        assert!(
            !(green < T::zero() || green > T::one()),
            "Green component must be between 0 and 1."
        );
        assert!(
            !(blue < T::zero() || blue > T::one()),
            "Blue component must be between 0 and 1."
        );

        // Convert RGB to Lab
        let rgb = [red, green, blue];
        let xyz = rgb_to_xyz_components(&rgb);
        let lab = xyz_to_lab(&xyz);

        Self {
            lightness: lab[0],
            a_axis: lab[1],
            b_axis: lab[2],
        }
    }

    /// Get the red component.
    #[inline]
    pub fn red(&self) -> T {
        let rgb = self.rgb_components();
        rgb[0]
    }

    /// Get the green component.
    #[inline]
    pub fn green(&self) -> T {
        let rgb = self.rgb_components();
        rgb[1]
    }

    /// Get the blue component.
    #[inline]
    pub fn blue(&self) -> T {
        let rgb = self.rgb_components();
        rgb[2]
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
}
