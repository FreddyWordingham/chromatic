//! XYZ colour with transparency representation.

use num_traits::Float;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    error::{Result, validate_unit_component},
    impl_transparent_colour, impl_transparent_convert, impl_transparent_display,
    spaces::{Grey, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz},
    traits::{Colour, Convert},
};

/// XYZ with alpha channel.
#[derive(Debug, Clone, Copy)]
pub struct XyzAlpha<T: Float + Send + Sync> {
    /// Base colour
    colour: Xyz<T>,
    /// Alpha component in range [0, 1].
    alpha: T,
}

impl<T: Float + Send + Sync> XyzAlpha<T> {
    /// Create a new `XyzAlpha` instance.
    ///
    /// # Arguments
    ///
    /// * `x` - The X component, must be in range [0, 1]
    /// * `y` - The Y component (luminance), must be in range [0, 1]
    /// * `z` - The Z component, must be in range [0, 1]
    /// * `alpha` - The alpha (transparency) component, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if any component is outside the range [0, 1].
    pub fn new(x: T, y: T, z: T, alpha: T) -> Result<Self> {
        validate_unit_component(alpha, "alpha")?;

        Ok(Self {
            colour: Xyz::new(x, y, z)?,
            alpha,
        })
    }

    /// Create a new `XyzAlpha` instance from a `Xyz` colour and an alpha component.
    ///
    /// # Arguments
    ///
    /// * `colour` - The base XYZ colour
    /// * `alpha` - The alpha (transparency) component, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if the alpha component is outside the range [0, 1].
    fn new_colour_with_alpha(colour: Xyz<T>, alpha: T) -> Result<Self> {
        validate_unit_component(alpha, "alpha")?;

        Ok(Self { colour, alpha })
    }

    /// Get the base `colour`.
    const fn colour(&self) -> &Xyz<T> {
        &self.colour
    }

    /// Get the `x` component.
    pub const fn x(&self) -> T {
        self.colour.x()
    }

    /// Get the `y` component (luminance).
    pub const fn y(&self) -> T {
        self.colour.y()
    }

    /// Get the `z` component.
    pub const fn z(&self) -> T {
        self.colour.z()
    }

    /// Get the `alpha` component.
    pub const fn alpha(&self) -> T {
        self.alpha
    }

    /// Set the `x` component.
    ///
    /// # Arguments
    ///
    /// * `x` - The new X value, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 1].
    pub fn set_x(&mut self, x: T) -> Result<()> {
        self.colour.set_x(x)
    }

    /// Set the `y` component (luminance).
    ///
    /// # Arguments
    ///
    /// * `y` - The new Y value, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 1].
    pub fn set_y(&mut self, y: T) -> Result<()> {
        self.colour.set_y(y)
    }

    /// Set the `z` component.
    ///
    /// # Arguments
    ///
    /// * `z` - The new Z value, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 1].
    pub fn set_z(&mut self, z: T) -> Result<()> {
        self.colour.set_z(z)
    }

    /// Set the `alpha` component.
    ///
    /// # Arguments
    ///
    /// * `alpha` - The new alpha value, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 1].
    pub fn set_alpha(&mut self, alpha: T) -> Result<()> {
        validate_unit_component(alpha, "alpha")?;
        self.alpha = alpha;
        Ok(())
    }

    /// Set all components at once with validation.
    ///
    /// # Arguments
    ///
    /// * `x` - The X component, must be in range [0, 1]
    /// * `y` - The Y component (luminance), must be in range [0, 1]
    /// * `z` - The Z component, must be in range [0, 1]
    /// * `alpha` - The alpha component, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if any component validation fails.
    pub fn set_components(&mut self, x: T, y: T, z: T, alpha: T) -> Result<()> {
        validate_unit_component(x, "x")?;
        validate_unit_component(y, "y")?;
        validate_unit_component(z, "z")?;
        validate_unit_component(alpha, "alpha")?;

        // If all validations pass, update all components
        self.colour = Xyz::new(x, y, z)?;
        self.alpha = alpha;
        Ok(())
    }

    /// Get XYZ values relative to D65 reference white.
    /// Returns (X/Xn, Y/Yn, Z/Zn)
    ///
    /// # Errors
    ///
    /// Returns an error if reference white calculation fails.
    pub fn relative_to_white(&self) -> Result<(T, T, T)> {
        self.colour.relative_to_white()
    }

    /// Calculate perceptual colour difference in XYZ space (simple Euclidean distance),
    /// ignoring the alpha channel.
    /// Note: This is not an ideal colour difference metric - consider using Lab with Delta E metrics for better results.
    pub fn distance(&self, other: &Self) -> T {
        self.colour.distance(&other.colour)
    }
}

impl_transparent_colour!(XyzAlpha<T>, Xyz<T>, 3);
impl_transparent_convert!(XyzAlpha<T>, Xyz<T>);
impl_transparent_display!(XyzAlpha<T>);
