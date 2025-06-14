//! RGB colour with transparency representation.

use num_traits::Float;

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    num::ParseIntError,
};

use crate::{
    Colour, Convert, Grey, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, ParseColourError, Rgb, Srgb, SrgbAlpha, Xyz,
    XyzAlpha, impl_transparent_colour, impl_transparent_convert, impl_transparent_display,
};

/// RGB with alpha channel.
#[derive(Debug, Clone, Copy)]
pub struct RgbAlpha<T: Float + Send + Sync> {
    /// Base colour
    colour: Rgb<T>,
    /// Alpha component
    alpha: T,
}

impl<T: Float + Send + Sync> RgbAlpha<T> {
    /// Create a new `RgbAlpha` instance.
    pub fn new(red: T, green: T, blue: T, alpha: T) -> Self {
        debug_assert!(
            alpha >= T::zero() && alpha <= T::one(),
            "Alpha component must be in range [0, 1]."
        );
        Self {
            colour: Rgb::new(red, green, blue),
            alpha,
        }
    }

    /// Create a new `RgbAlpha` instance from a `Rgb` colour and an alpha component.
    fn new_colour_with_alpha(colour: Rgb<T>, alpha: T) -> Self {
        debug_assert!(
            alpha >= T::zero() && alpha <= T::one(),
            "Alpha component must be in range [0, 1]."
        );
        Self { colour, alpha }
    }

    /// Get the base `colour`.
    const fn colour(&self) -> &Rgb<T> {
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
    pub fn set_red(&mut self, red: T) {
        self.colour.set_red(red);
    }

    /// Set the `green` component.
    pub fn set_green(&mut self, green: T) {
        self.colour.set_green(green);
    }

    /// Set the `blue` component.
    pub fn set_blue(&mut self, blue: T) {
        self.colour.set_blue(blue);
    }

    /// Set the `alpha` component.
    pub fn set_alpha(&mut self, alpha: T) {
        debug_assert!(
            alpha >= T::zero() && alpha <= T::one(),
            "Alpha component must be in range [0, 1]."
        );
        self.alpha = alpha;
    }
}

impl_transparent_colour!(RgbAlpha<T>, Rgb<T>, 3);
impl_transparent_convert!(RgbAlpha<T>, Rgb<T>);
impl_transparent_display!(RgbAlpha<T>);
