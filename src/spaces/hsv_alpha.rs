//! HSV colour with transparency representation.

use num_traits::Float;

use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    error::{ChromaticError, Result},
    impl_transparent_colour, impl_transparent_convert, impl_transparent_display,
    spaces::{Grey, GreyAlpha, Hsl, HslAlpha, Hsv, Lab, LabAlpha, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz, XyzAlpha},
    traits::{Colour, Convert},
};

/// HSV with alpha channel.
#[derive(Debug, Clone, Copy)]
pub struct HsvAlpha<T: Float + Send + Sync> {
    /// Base colour
    colour: Hsv<T>,
    /// Alpha component
    alpha: T,
}

impl<T: Float + Send + Sync> HsvAlpha<T> {
    /// Create a new `HsvAlpha` instance.
    pub fn new(hue: T, saturation: T, value: T, alpha: T) -> Result<Self> {
        Self::validate_component(alpha, "alpha")?;

        Ok(Self {
            colour: Hsv::new(hue, saturation, value)?,
            alpha,
        })
    }

    /// Create a new `HsvAlpha` instance from a `Hsv` colour and an alpha component.
    fn new_colour_with_alpha(colour: Hsv<T>, alpha: T) -> Result<Self> {
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
    const fn colour(&self) -> &Hsv<T> {
        &self.colour
    }

    /// Get the `hue` component.
    pub const fn hue(&self) -> T {
        self.colour.hue()
    }

    /// Get the `saturation` component.
    pub const fn saturation(&self) -> T {
        self.colour.saturation()
    }

    /// Get the `value` component.
    pub const fn value(&self) -> T {
        self.colour.value()
    }

    /// Get the `alpha` component.
    pub const fn alpha(&self) -> T {
        self.alpha
    }

    /// Set the `hue` component.
    pub fn set_hue(&mut self, red: T) -> Result<()> {
        self.colour.set_hue(red)
    }

    /// Set the `saturation` component.
    pub fn set_saturation(&mut self, green: T) -> Result<()> {
        self.colour.set_saturation(green)
    }

    /// Set the `value` component.
    pub fn set_value(&mut self, blue: T) -> Result<()> {
        self.colour.set_value(blue)
    }

    /// Set the `alpha` component.
    pub fn set_alpha(&mut self, alpha: T) -> Result<()> {
        Self::validate_component(alpha, "alpha")?;
        self.alpha = alpha;
        Ok(())
    }
}

impl_transparent_colour!(HsvAlpha<T>, Hsv<T>, 3);
impl_transparent_convert!(HsvAlpha<T>, Hsv<T>);
impl_transparent_display!(HsvAlpha<T>);
