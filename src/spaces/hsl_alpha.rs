//! HSL colour with transparency representation.

use num_traits::Float;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    error::{Result, normalize_hue, validate_unit_component},
    impl_transparent_colour, impl_transparent_convert, impl_transparent_display,
    spaces::{Grey, GreyAlpha, Hsl, Hsv, HsvAlpha, Lab, LabAlpha, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz, XyzAlpha},
    traits::{Colour, Convert},
};

/// HSL with alpha channel.
#[derive(Debug, Clone, Copy)]
pub struct HslAlpha<T: Float + Send + Sync> {
    /// Base colour
    colour: Hsl<T>,
    /// Alpha component in range [0, 1].
    alpha: T,
}

impl<T: Float + Send + Sync> HslAlpha<T> {
    /// Create a new `HslAlpha` instance.
    ///
    /// # Arguments
    ///
    /// * `hue` - The hue in degrees, will be normalized to [0, 360)
    /// * `saturation` - The saturation, must be in range [0, 1]
    /// * `lightness` - The lightness, must be in range [0, 1]
    /// * `alpha` - The alpha (transparency) component, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if saturation, lightness, or alpha are outside [0, 1],
    /// or if hue normalization fails.
    pub fn new(hue: T, saturation: T, lightness: T, alpha: T) -> Result<Self> {
        validate_unit_component(alpha, "alpha")?;

        Ok(Self {
            colour: Hsl::new(hue, saturation, lightness)?,
            alpha,
        })
    }

    /// Create a new `HslAlpha` instance from a `Hsl` colour and an alpha component.
    ///
    /// # Arguments
    ///
    /// * `colour` - The base HSL colour
    /// * `alpha` - The alpha (transparency) component, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if the alpha component is outside the range [0, 1].
    fn new_colour_with_alpha(colour: Hsl<T>, alpha: T) -> Result<Self> {
        validate_unit_component(alpha, "alpha")?;

        Ok(Self { colour, alpha })
    }

    /// Get the base `colour`.
    const fn colour(&self) -> &Hsl<T> {
        &self.colour
    }

    /// Get the `hue` component in degrees [0, 360).
    pub const fn hue(&self) -> T {
        self.colour.hue()
    }

    /// Get the `saturation` component.
    pub const fn saturation(&self) -> T {
        self.colour.saturation()
    }

    /// Get the `lightness` component.
    pub const fn lightness(&self) -> T {
        self.colour.lightness()
    }

    /// Get the `alpha` component.
    pub const fn alpha(&self) -> T {
        self.alpha
    }

    /// Set the `hue` component.
    ///
    /// # Arguments
    ///
    /// * `hue` - The new hue in degrees, will be normalized to [0, 360)
    ///
    /// # Errors
    ///
    /// Returns an error if hue normalization fails.
    pub fn set_hue(&mut self, hue: T) -> Result<()> {
        self.colour.set_hue(hue)
    }

    /// Set the `saturation` component.
    ///
    /// # Arguments
    ///
    /// * `saturation` - The new saturation, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 1].
    pub fn set_saturation(&mut self, saturation: T) -> Result<()> {
        self.colour.set_saturation(saturation)
    }

    /// Set the `lightness` component.
    ///
    /// # Arguments
    ///
    /// * `lightness` - The new lightness, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 1].
    pub fn set_lightness(&mut self, lightness: T) -> Result<()> {
        self.colour.set_lightness(lightness)
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
    /// * `hue` - The hue in degrees, will be normalized to [0, 360)
    /// * `saturation` - The saturation, must be in range [0, 1]
    /// * `lightness` - The lightness, must be in range [0, 1]
    /// * `alpha` - The alpha component, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if any component validation fails.
    pub fn set_components(&mut self, hue: T, saturation: T, lightness: T, alpha: T) -> Result<()> {
        let normalized_hue = normalize_hue(hue)?;
        validate_unit_component(saturation, "saturation")?;
        validate_unit_component(lightness, "lightness")?;
        validate_unit_component(alpha, "alpha")?;

        // If all validations pass, update all components
        self.colour = Hsl::new(normalized_hue, saturation, lightness)?;
        self.alpha = alpha;
        Ok(())
    }
}

impl_transparent_colour!(HslAlpha<T>, Hsl<T>, 3);
impl_transparent_convert!(HslAlpha<T>, Hsl<T>);
impl_transparent_display!(HslAlpha<T>);
