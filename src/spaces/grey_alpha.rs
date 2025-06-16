//! Monochrome colour with transparency representation.

use num_traits::Float;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    error::{Result, validate_unit_component},
    impl_transparent_colour, impl_transparent_convert, impl_transparent_display,
    spaces::{Grey, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz, XyzAlpha},
    traits::{Colour, Convert},
};

/// Grey with alpha channel.
#[derive(Debug, Clone, Copy)]
pub struct GreyAlpha<T: Float + Send + Sync> {
    /// Base colour
    colour: Grey<T>,
    /// Alpha component in range [0, 1].
    alpha: T,
}

impl<T: Float + Send + Sync> GreyAlpha<T> {
    /// Create a new `GreyAlpha` instance.
    ///
    /// # Arguments
    ///
    /// * `grey` - The grey component, must be in range [0, 1]
    /// * `alpha` - The alpha (transparency) component, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if any component is outside the range [0, 1].
    pub fn new(grey: T, alpha: T) -> Result<Self> {
        validate_unit_component(alpha, "alpha")?;
        Ok(Self {
            colour: Grey::new(grey)?,
            alpha,
        })
    }

    /// Create a new `GreyAlpha` instance from a `Grey` colour and an alpha component.
    ///
    /// # Arguments
    ///
    /// * `colour` - The base grey colour
    /// * `alpha` - The alpha (transparency) component, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if the alpha component is outside the range [0, 1].
    fn new_colour_with_alpha(colour: Grey<T>, alpha: T) -> Result<Self> {
        validate_unit_component(alpha, "alpha")?;
        Ok(Self { colour, alpha })
    }

    /// Get the base `colour`.
    const fn colour(&self) -> &Grey<T> {
        &self.colour
    }

    /// Get the `grey` component.
    pub const fn grey(&self) -> T {
        self.colour.grey()
    }

    /// Get the `alpha` component.
    pub const fn alpha(&self) -> T {
        self.alpha
    }

    /// Set the `grey` component.
    ///
    /// # Arguments
    ///
    /// * `grey` - The new grey value, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 1].
    pub fn set_grey(&mut self, grey: T) -> Result<()> {
        self.colour.set_grey(grey)
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

    /// Set both components at once with validation.
    ///
    /// # Arguments
    ///
    /// * `grey` - The grey component, must be in range [0, 1]
    /// * `alpha` - The alpha component, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if any component is outside the range [0, 1].
    pub fn set_components(&mut self, grey: T, alpha: T) -> Result<()> {
        self.colour.set_grey(grey)?;
        validate_unit_component(alpha, "alpha")?;
        self.alpha = alpha;
        Ok(())
    }
}

impl_transparent_colour!(GreyAlpha<T>, Grey<T>, 1);
impl_transparent_convert!(GreyAlpha<T>, Grey<T>);
impl_transparent_display!(GreyAlpha<T>);
