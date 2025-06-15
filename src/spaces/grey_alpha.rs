//! Monochrome colour with transparency representation.

use num_traits::Float;

use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    error::{ChromaticError, Result},
    impl_transparent_colour, impl_transparent_convert, impl_transparent_display,
    spaces::{Grey, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz, XyzAlpha},
    traits::{Colour, Convert},
};

/// Grey with alpha channel.
#[derive(Debug, Clone, Copy)]
pub struct GreyAlpha<T: Float + Send + Sync> {
    /// Base colour
    colour: Grey<T>,
    /// Alpha component
    alpha: T,
}

impl<T: Float + Send + Sync> GreyAlpha<T> {
    /// Create a new `GreyAlpha` instance.
    pub fn new(grey: T, alpha: T) -> Result<Self> {
        Self::validate_component(alpha, "alpha")?;
        Ok(Self {
            colour: Grey::new(grey)?,
            alpha,
        })
    }

    /// Create a new `GreyAlpha` instance from a `Grey` colour and an alpha component.
    fn new_colour_with_alpha(colour: Grey<T>, alpha: T) -> Result<Self> {
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
    pub fn set_grey(&mut self, grey: T) -> Result<()> {
        Self::validate_component(grey, "grey")?;
        self.colour.set_grey(grey)
    }

    /// Set the `alpha` component.
    pub fn set_alpha(&mut self, alpha: T) -> Result<()> {
        Self::validate_component(alpha, "alpha")?;
        self.alpha = alpha;
        Ok(())
    }
}

impl_transparent_colour!(GreyAlpha<T>, Grey<T>, 1);
impl_transparent_convert!(GreyAlpha<T>, Grey<T>);
impl_transparent_display!(GreyAlpha<T>);
