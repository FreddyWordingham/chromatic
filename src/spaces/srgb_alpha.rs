//! sRGB colour with transparency representation.

use num_traits::Float;

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    num::ParseIntError,
};

use crate::{
    Colour, Convert, Grey, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, ParseColourError, Rgb, RgbAlpha, Srgb, Xyz,
    XyzAlpha, impl_transparent_colour, impl_transparent_convert, impl_transparent_display,
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
    #[inline]
    pub fn new(red: T, green: T, blue: T, alpha: T) -> Self {
        debug_assert!(
            alpha >= T::zero() && alpha <= T::one(),
            "Alpha component must be in range [0, 1]."
        );
        Self {
            colour: Srgb::new(red, green, blue),
            alpha,
        }
    }

    /// Create a new `SrgbAlpha` instance from a `Srgb` colour and an alpha component.
    #[inline]
    fn new_colour_with_alpha(colour: Srgb<T>, alpha: T) -> Self {
        debug_assert!(
            alpha >= T::zero() && alpha <= T::one(),
            "Alpha component must be in range [0, 1]."
        );
        Self { colour, alpha }
    }

    /// Get the base `colour`.
    #[inline]
    const fn colour(&self) -> &Srgb<T> {
        &self.colour
    }

    /// Get the `red` component.
    #[inline]
    pub const fn red(&self) -> T {
        self.colour.red()
    }

    /// Get the `green` component.
    #[inline]
    pub const fn green(&self) -> T {
        self.colour.green()
    }

    /// Get the `blue` component.
    #[inline]
    pub const fn blue(&self) -> T {
        self.colour.blue()
    }

    /// Get the `alpha` component.
    #[inline]
    pub const fn alpha(&self) -> T {
        self.alpha
    }

    /// Set the `red` component.
    #[inline]
    pub fn set_red(&mut self, red: T) {
        self.colour.set_red(red);
    }

    /// Set the `green` component.
    #[inline]
    pub fn set_green(&mut self, green: T) {
        self.colour.set_green(green);
    }

    /// Set the `blue` component.
    #[inline]
    pub fn set_blue(&mut self, blue: T) {
        self.colour.set_blue(blue);
    }

    /// Set the `alpha` component.
    #[inline]
    pub fn set_alpha(&mut self, alpha: T) {
        debug_assert!(
            alpha >= T::zero() && alpha <= T::one(),
            "Alpha component must be in range [0, 1]."
        );
        self.alpha = alpha;
    }
}

impl_transparent_colour!(SrgbAlpha<T>, Srgb<T>, 3);
impl_transparent_convert!(SrgbAlpha<T>, Srgb<T>);
impl_transparent_display!(SrgbAlpha<T>);
