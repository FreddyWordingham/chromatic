//! Lab colour with transparency representation.

use num_traits::Float;

use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    error::{ChromaticError, Result},
    impl_transparent_colour, impl_transparent_convert, impl_transparent_display,
    spaces::{Grey, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz, XyzAlpha},
    traits::{Colour, Convert},
};

/// Lab with alpha channel.
#[derive(Debug, Clone, Copy)]
pub struct LabAlpha<T: Float + Send + Sync> {
    /// Base colour
    colour: Lab<T>,
    /// Alpha component
    alpha: T,
}

impl<T: Float + Send + Sync> LabAlpha<T> {
    /// Create a new `LabAlpha` instance.
    pub fn new(l: T, a: T, b: T, alpha: T) -> Result<Self> {
        Self::validate_component(alpha, "alpha")?;

        Ok(Self {
            colour: Lab::new(l, a, b)?,
            alpha,
        })
    }

    /// Create a new `LabAlpha` instance from a `Lab` colour and an alpha component.
    fn new_colour_with_alpha(colour: Lab<T>, alpha: T) -> Result<Self> {
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
    const fn colour(&self) -> &Lab<T> {
        &self.colour
    }

    /// Get the `lightness` component.
    pub const fn lightness(&self) -> T {
        self.colour.lightness()
    }

    /// Get the `a_star` component.
    pub const fn a_star(&self) -> T {
        self.colour.a_star()
    }

    /// Get the `b_star` component.
    pub const fn b_star(&self) -> T {
        self.colour.b_star()
    }

    /// Get the `alpha` component.
    pub const fn alpha(&self) -> T {
        self.alpha
    }

    /// Set the `lightness` component.
    pub fn set_lightness(&mut self, l: T) -> Result<()> {
        self.colour.set_lightness(l)
    }

    /// Set the `a_star` component.
    pub fn set_a_star(&mut self, a: T) -> Result<()> {
        self.colour.set_a_star(a)
    }

    /// Set the `b_star` component.
    pub fn set_b_star(&mut self, b: T) -> Result<()> {
        self.colour.set_b_star(b)
    }

    /// Set the `alpha` component.
    pub fn set_alpha(&mut self, alpha: T) -> Result<()> {
        Self::validate_component(alpha, "alpha")?;
        self.alpha = alpha;
        Ok(())
    }

    /// Calculate perceptual color difference in Lab space (CIE76 Delta E),
    /// ignoring the alpha channel.
    pub fn delta_e(&self, other: &Self) -> T {
        self.colour.delta_e(&other.colour)
    }

    /// Calculate perceptual color difference using the improved CIE94 Delta E formula,
    /// ignoring the alpha channel.
    pub fn delta_e94(&self, other: &Self) -> T {
        self.colour.delta_e94(&other.colour)
    }
}

impl_transparent_colour!(LabAlpha<T>, Lab<T>, 3);
impl_transparent_convert!(LabAlpha<T>, Lab<T>);
impl_transparent_display!(LabAlpha<T>);
