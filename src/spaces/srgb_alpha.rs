//! sRGB colour with transparency representation.

use num_traits::Float;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    error::{Result, validate_unit_component},
    impl_transparent_colour, impl_transparent_convert, impl_transparent_display,
    spaces::{Grey, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, Rgb, RgbAlpha, Srgb, Xyz, XyzAlpha},
    traits::{Colour, Convert},
};

/// sRGB with alpha channel.
#[derive(Debug, Clone, Copy)]
pub struct SrgbAlpha<T: Float + Send + Sync> {
    /// Base colour
    colour: Srgb<T>,
    /// Alpha component in range [0, 1].
    alpha: T,
}

impl<T: Float + Send + Sync> SrgbAlpha<T> {
    /// Create a new `SrgbAlpha` instance.
    ///
    /// # Arguments
    ///
    /// * `red` - The red component, must be in range [0, 1]
    /// * `green` - The green component, must be in range [0, 1]
    /// * `blue` - The blue component, must be in range [0, 1]
    /// * `alpha` - The alpha (transparency) component, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if any component is outside the range [0, 1].
    pub fn new(red: T, green: T, blue: T, alpha: T) -> Result<Self> {
        validate_unit_component(alpha, "alpha")?;

        Ok(Self {
            colour: Srgb::new(red, green, blue)?,
            alpha,
        })
    }

    /// Create a new `SrgbAlpha` instance from a `Srgb` colour and an alpha component.
    ///
    /// # Arguments
    ///
    /// * `colour` - The base sRGB colour
    /// * `alpha` - The alpha (transparency) component, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if the alpha component is outside the range [0, 1].
    fn new_colour_with_alpha(colour: Srgb<T>, alpha: T) -> Result<Self> {
        validate_unit_component(alpha, "alpha")?;

        Ok(Self { colour, alpha })
    }

    /// Get the base `colour`.
    const fn colour(&self) -> &Srgb<T> {
        &self.colour
    }

    /// Get the `red` component.
    pub const fn red(&self) -> T {
        self.colour.red()
    }

    /// Get the `green` component.
    pub const fn green(&self) -> T {
        self.colour.green()
    }

    /// Get the `blue` component.
    pub const fn blue(&self) -> T {
        self.colour.blue()
    }

    /// Get the `alpha` component.
    pub const fn alpha(&self) -> T {
        self.alpha
    }

    /// Set the `red` component.
    ///
    /// # Arguments
    ///
    /// * `red` - The new red value, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 1].
    pub fn set_red(&mut self, red: T) -> Result<()> {
        self.colour.set_red(red)
    }

    /// Set the `green` component.
    ///
    /// # Arguments
    ///
    /// * `green` - The new green value, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 1].
    pub fn set_green(&mut self, green: T) -> Result<()> {
        self.colour.set_green(green)
    }

    /// Set the `blue` component.
    ///
    /// # Arguments
    ///
    /// * `blue` - The new blue value, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 1].
    pub fn set_blue(&mut self, blue: T) -> Result<()> {
        self.colour.set_blue(blue)
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
    /// * `red` - The red component, must be in range [0, 1]
    /// * `green` - The green component, must be in range [0, 1]
    /// * `blue` - The blue component, must be in range [0, 1]
    /// * `alpha` - The alpha component, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if any component validation fails.
    pub fn set_components(&mut self, red: T, green: T, blue: T, alpha: T) -> Result<()> {
        validate_unit_component(red, "red")?;
        validate_unit_component(green, "green")?;
        validate_unit_component(blue, "blue")?;
        validate_unit_component(alpha, "alpha")?;

        // If all validations pass, update all components
        self.colour = Srgb::new(red, green, blue)?;
        self.alpha = alpha;
        Ok(())
    }
}

impl_transparent_colour!(SrgbAlpha<T>, Srgb<T>, 3);
impl_transparent_convert!(SrgbAlpha<T>, Srgb<T>);
impl_transparent_display!(SrgbAlpha<T>);
