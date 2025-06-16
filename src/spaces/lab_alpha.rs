//! Lab colour with transparency representation.

use num_traits::Float;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    error::{Result, safe_constant, validate_component_range, validate_unit_component},
    impl_transparent_colour, impl_transparent_convert, impl_transparent_display,
    spaces::{Grey, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz, XyzAlpha},
    traits::{Colour, Convert},
};

/// Lab with alpha channel.
#[derive(Debug, Clone, Copy)]
pub struct LabAlpha<T: Float + Send + Sync> {
    /// Base colour
    colour: Lab<T>,
    /// Alpha component in range [0, 1].
    alpha: T,
}

impl<T: Float + Send + Sync> LabAlpha<T> {
    /// Create a new `LabAlpha` instance.
    ///
    /// # Arguments
    ///
    /// * `l` - The L* component (lightness), must be in range [0, 100]
    /// * `a` - The a* component, must be in range [-128, 127]
    /// * `b` - The b* component, must be in range [-128, 127]
    /// * `alpha` - The alpha (transparency) component, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if any component is outside its valid range.
    pub fn new(l: T, a: T, b: T, alpha: T) -> Result<Self> {
        validate_unit_component(alpha, "alpha")?;

        Ok(Self {
            colour: Lab::new(l, a, b)?,
            alpha,
        })
    }

    /// Create a new `LabAlpha` instance from a `Lab` colour and an alpha component.
    ///
    /// # Arguments
    ///
    /// * `colour` - The base Lab colour
    /// * `alpha` - The alpha (transparency) component, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if the alpha component is outside the range [0, 1].
    fn new_colour_with_alpha(colour: Lab<T>, alpha: T) -> Result<Self> {
        validate_unit_component(alpha, "alpha")?;

        Ok(Self { colour, alpha })
    }

    /// Get the base `colour`.
    const fn colour(&self) -> &Lab<T> {
        &self.colour
    }

    /// Get the `lightness` component (L*).
    pub const fn lightness(&self) -> T {
        self.colour.lightness()
    }

    /// Get the `a_star` component (a*).
    pub const fn a_star(&self) -> T {
        self.colour.a_star()
    }

    /// Get the `b_star` component (b*).
    pub const fn b_star(&self) -> T {
        self.colour.b_star()
    }

    /// Get the `alpha` component.
    pub const fn alpha(&self) -> T {
        self.alpha
    }

    /// Set the `lightness` component.
    ///
    /// # Arguments
    ///
    /// * `l` - The new L* value, must be in range [0, 100]
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 100].
    pub fn set_lightness(&mut self, l: T) -> Result<()> {
        self.colour.set_lightness(l)
    }

    /// Set the `a_star` component.
    ///
    /// # Arguments
    ///
    /// * `a` - The new a* value, must be in range [-128, 127]
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [-128, 127].
    pub fn set_a_star(&mut self, a: T) -> Result<()> {
        self.colour.set_a_star(a)
    }

    /// Set the `b_star` component.
    ///
    /// # Arguments
    ///
    /// * `b` - The new b* value, must be in range [-128, 127]
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [-128, 127].
    pub fn set_b_star(&mut self, b: T) -> Result<()> {
        self.colour.set_b_star(b)
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
    /// * `l` - The L* component (lightness), must be in range [0, 100]
    /// * `a` - The a* component, must be in range [-128, 127]
    /// * `b` - The b* component, must be in range [-128, 127]
    /// * `alpha` - The alpha component, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if any component validation fails.
    pub fn set_components(&mut self, l: T, a: T, b: T, alpha: T) -> Result<()> {
        let max_lightness = safe_constant(100.0)?;
        let min_chroma = safe_constant(-128.0)?;
        let max_chroma = safe_constant(127.0)?;

        validate_component_range(l, "lightness", T::zero(), max_lightness)?;
        validate_component_range(a, "a*", min_chroma, max_chroma)?;
        validate_component_range(b, "b*", min_chroma, max_chroma)?;
        validate_unit_component(alpha, "alpha")?;

        // If all validations pass, update all components
        self.colour = Lab::new(l, a, b)?;
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
    ///
    /// # Errors
    ///
    /// Returns an error if mathematical operations fail during calculation.
    pub fn delta_e94(&self, other: &Self) -> Result<T> {
        self.colour.delta_e94(&other.colour)
    }
}

impl_transparent_colour!(LabAlpha<T>, Lab<T>, 3);
impl_transparent_convert!(LabAlpha<T>, Lab<T>);
impl_transparent_display!(LabAlpha<T>);
