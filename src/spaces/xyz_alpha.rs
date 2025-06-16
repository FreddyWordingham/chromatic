//! XYZ colour with transparency representation.

use num_traits::Float;

use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    error::{ChromaticError, Result},
    impl_transparent_colour, impl_transparent_convert, impl_transparent_display,
    spaces::{Grey, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz},
    traits::{Colour, Convert},
};

/// XYZ with alpha channel.
#[derive(Debug, Clone, Copy)]
pub struct XyzAlpha<T: Float + Send + Sync> {
    /// Base colour
    colour: Xyz<T>,
    /// Alpha component
    alpha: T,
}

impl<T: Float + Send + Sync> XyzAlpha<T> {
    /// Create a new `XyzAlpha` instance.
    pub fn new(x: T, y: T, z: T, alpha: T) -> Result<Self> {
        Self::validate_component(alpha, "alpha")?;

        Ok(Self {
            colour: Xyz::new(x, y, z)?,
            alpha,
        })
    }

    /// Create a new `XyzAlpha` instance from a `Xyz` colour and an alpha component.
    fn new_colour_with_alpha(colour: Xyz<T>, alpha: T) -> Result<Self> {
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
    const fn colour(&self) -> &Xyz<T> {
        &self.colour
    }

    /// Get the `x` component.
    pub const fn x(&self) -> T {
        self.colour.x()
    }

    /// Get the `y` component.
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
    pub fn set_x(&mut self, x: T) -> Result<()> {
        self.colour.set_x(x)
    }

    /// Set the `y` component.
    pub fn set_y(&mut self, y: T) -> Result<()> {
        self.colour.set_y(y)
    }

    /// Set the `z` component.
    pub fn set_z(&mut self, z: T) -> Result<()> {
        self.colour.set_z(z)
    }

    /// Set the `alpha` component.
    pub fn set_alpha(&mut self, alpha: T) -> Result<()> {
        Self::validate_component(alpha, "alpha")?;
        self.alpha = alpha;
        Ok(())
    }
}

impl_transparent_colour!(XyzAlpha<T>, Xyz<T>, 3);
impl_transparent_convert!(XyzAlpha<T>, Xyz<T>);
impl_transparent_display!(XyzAlpha<T>);
