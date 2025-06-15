//! sRGB colour with transparency representation.

use num_traits::Float;

use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    error::{ChromaticError, Result},
    impl_transparent_colour, impl_transparent_convert, impl_transparent_display,
    spaces::{Grey, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, Rgb, RgbAlpha, Srgb, Xyz, XyzAlpha},
    traits::{Colour, Convert},
};

/// sRGB with alpha channel.
#[derive(Debug, Clone, Copy)]
pub struct SrgbAlpha<T: Float + Send + Sync> {
    /// Base colour
    colour: Srgb<T>,
    /// Alpha component
    alpha: T,
}

impl<T: Float + Send + Sync> SrgbAlpha<T> {
    /// Create a new `SrgbAlpha` instance.
    pub fn new(red: T, green: T, blue: T, alpha: T) -> Result<Self> {
        Self::validate_component(alpha, "alpha")?;

        Ok(Self {
            colour: Srgb::new(red, green, blue)?,
            alpha,
        })
    }

    /// Create a new `SrgbAlpha` instance from a `Srgb` colour and an alpha component.
    fn new_colour_with_alpha(colour: Srgb<T>, alpha: T) -> Result<Self> {
        Self::validate_component(alpha, "alpha")?;

        Ok(Self { colour, alpha })
    }

    /// Validate a single component is in range [0, 1].
    fn validate_component(value: T, name: &str) -> Result<()> {
        if value < T::zero() || value > T::one() {
            return Err(ChromaticError::InvalidColour(format!(
                "{} component ({}) must be between 0 and 1",
                name,
                value.to_f64().unwrap_or(f64::NAN)
            )));
        }
        Ok(())
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
    pub fn set_red(&mut self, red: T) -> Result<()> {
        self.colour.set_red(red)
    }

    /// Set the `green` component.
    pub fn set_green(&mut self, green: T) -> Result<()> {
        self.colour.set_green(green)
    }

    /// Set the `blue` component.
    pub fn set_blue(&mut self, blue: T) -> Result<()> {
        self.colour.set_blue(blue)
    }

    /// Set the `alpha` component.
    pub fn set_alpha(&mut self, alpha: T) -> Result<()> {
        Self::validate_component(alpha, "alpha")?;
        self.alpha = alpha;
        Ok(())
    }
}

impl_transparent_colour!(SrgbAlpha<T>, Srgb<T>, 3);
impl_transparent_convert!(SrgbAlpha<T>, Srgb<T>);
impl_transparent_display!(SrgbAlpha<T>);
