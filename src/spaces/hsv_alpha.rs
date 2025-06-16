//! HSV colour with transparency representation.

use num_traits::Float;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    error::{Result, normalize_hue, validate_unit_component},
    impl_transparent_colour, impl_transparent_convert, impl_transparent_display,
    spaces::{Grey, GreyAlpha, Hsl, HslAlpha, Hsv, Lab, LabAlpha, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz, XyzAlpha},
    traits::{Colour, Convert},
};

/// HSV with alpha channel.
#[derive(Debug, Clone, Copy)]
pub struct HsvAlpha<T: Float + Send + Sync> {
    /// Base colour
    colour: Hsv<T>,
    /// Alpha component in range [0, 1].
    alpha: T,
}

impl<T: Float + Send + Sync> HsvAlpha<T> {
    /// Create a new `HsvAlpha` instance.
    ///
    /// # Arguments
    ///
    /// * `hue` - The hue in degrees, will be normalized to [0, 360)
    /// * `saturation` - The saturation, must be in range [0, 1]
    /// * `value` - The value (brightness), must be in range [0, 1]
    /// * `alpha` - The alpha (transparency) component, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if saturation, value, or alpha are outside [0, 1],
    /// or if hue normalization fails.
    pub fn new(hue: T, saturation: T, value: T, alpha: T) -> Result<Self> {
        validate_unit_component(alpha, "alpha")?;

        Ok(Self {
            colour: Hsv::new(hue, saturation, value)?,
            alpha,
        })
    }

    /// Create a new `HsvAlpha` instance from a `Hsv` colour and an alpha component.
    ///
    /// # Arguments
    ///
    /// * `colour` - The base HSV colour
    /// * `alpha` - The alpha (transparency) component, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if the alpha component is outside the range [0, 1].
    fn new_colour_with_alpha(colour: Hsv<T>, alpha: T) -> Result<Self> {
        validate_unit_component(alpha, "alpha")?;

        Ok(Self { colour, alpha })
    }

    /// Get the base `colour`.
    const fn colour(&self) -> &Hsv<T> {
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

    /// Get the `value` component.
    pub const fn value(&self) -> T {
        self.colour.value()
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

    /// Set the `value` component.
    ///
    /// # Arguments
    ///
    /// * `value` - The new value (brightness), must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 1].
    pub fn set_value(&mut self, value: T) -> Result<()> {
        self.colour.set_value(value)
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
    /// * `value` - The value (brightness), must be in range [0, 1]
    /// * `alpha` - The alpha component, must be in range [0, 1]
    ///
    /// # Errors
    ///
    /// Returns an error if any component validation fails.
    pub fn set_components(&mut self, hue: T, saturation: T, value: T, alpha: T) -> Result<()> {
        let normalized_hue = normalize_hue(hue)?;
        validate_unit_component(saturation, "saturation")?;
        validate_unit_component(value, "value")?;
        validate_unit_component(alpha, "alpha")?;

        // If all validations pass, update all components
        self.colour = Hsv::new(normalized_hue, saturation, value)?;
        self.alpha = alpha;
        Ok(())
    }
}

impl_transparent_colour!(HsvAlpha<T>, Hsv<T>, 3);
impl_transparent_convert!(HsvAlpha<T>, Hsv<T>);
impl_transparent_display!(HsvAlpha<T>);
