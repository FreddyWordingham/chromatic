//! HSL colour with transparency representation.

use num_traits::Float;

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    num::ParseIntError,
};

use crate::{
    Colour, Convert, Grey, GreyAlpha, Hsl, Hsv, HsvAlpha, Lab, LabAlpha, ParseColourError, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz,
    XyzAlpha, impl_transparent_colour, impl_transparent_convert, impl_transparent_display,
};

/// HSL with alpha channel.
#[derive(Debug, Clone, Copy)]
pub struct HslAlpha<T: Float + Send + Sync> {
    /// Base colour
    colour: Hsl<T>,
    /// Alpha component
    alpha: T,
}

impl<T: Float + Send + Sync> HslAlpha<T> {
    /// Create a new `HslAlpha` instance.
    #[inline]
    pub fn new(hue: T, saturation: T, lightness: T, alpha: T) -> Self {
        debug_assert!(
            alpha >= T::zero() && alpha <= T::one(),
            "Alpha component must be in range [0, 1]."
        );
        Self {
            colour: Hsl::new(hue, saturation, lightness),
            alpha,
        }
    }

    /// Create a new `HslAlpha` instance from a `Hsl` colour and an alpha component.
    #[inline]
    fn new_colour_with_alpha(colour: Hsl<T>, alpha: T) -> Self {
        debug_assert!(
            alpha >= T::zero() && alpha <= T::one(),
            "Alpha component must be in range [0, 1]."
        );
        Self { colour, alpha }
    }

    /// Get the base `colour`.
    #[inline]
    const fn colour(&self) -> &Hsl<T> {
        &self.colour
    }

    /// Get the `hue` component.
    #[inline]
    pub const fn hue(&self) -> T {
        self.colour.hue()
    }

    /// Get the `saturation` component.
    #[inline]
    pub const fn saturation(&self) -> T {
        self.colour.saturation()
    }

    /// Get the `lightness` component.
    #[inline]
    pub const fn lightness(&self) -> T {
        self.colour.lightness()
    }

    /// Get the `alpha` component.
    #[inline]
    pub const fn alpha(&self) -> T {
        self.alpha
    }

    /// Set the `hue` component.
    #[inline]
    pub fn set_hue(&mut self, hue: T) {
        self.colour.set_hue(hue);
    }

    /// Set the `saturation` component.
    #[inline]
    pub fn set_saturation(&mut self, saturation: T) {
        self.colour.set_saturation(saturation);
    }

    /// Set the `lightness` component.
    #[inline]
    pub fn set_lightness(&mut self, lightness: T) {
        self.colour.set_lightness(lightness);
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

impl_transparent_colour!(HslAlpha<T>, Hsl<T>, 3);
impl_transparent_convert!(HslAlpha<T>, Hsl<T>);
impl_transparent_display!(HslAlpha<T>);
