//! HSV colour with transparency representation.

use num_traits::Float;

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    num::ParseIntError,
};

use crate::{
    Colour, Convert, Grey, GreyAlpha, Hsl, HslAlpha, Hsv, Lab, LabAlpha, ParseColourError, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz,
    XyzAlpha, impl_transparent_colour, impl_transparent_convert, impl_transparent_display,
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
    #[inline]
    pub fn new(hue: T, saturation: T, value: T, alpha: T) -> Self {
        debug_assert!(
            alpha >= T::zero() && alpha <= T::one(),
            "Alpha component must be in range [0, 1]."
        );
        Self {
            colour: Hsv::new(hue, saturation, value),
            alpha,
        }
    }

    /// Create a new `HsvAlpha` instance from a `Hsv` colour and an alpha component.
    #[inline]
    fn new_colour_with_alpha(colour: Hsv<T>, alpha: T) -> Self {
        debug_assert!(
            alpha >= T::zero() && alpha <= T::one(),
            "Alpha component must be in range [0, 1]."
        );
        Self { colour, alpha }
    }

    /// Get the base `colour`.
    #[inline]
    const fn colour(&self) -> &Hsv<T> {
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

    /// Get the `value` component.
    #[inline]
    pub const fn value(&self) -> T {
        self.colour.value()
    }

    /// Get the `alpha` component.
    #[inline]
    pub const fn alpha(&self) -> T {
        self.alpha
    }

    /// Set the `hue` component.
    #[inline]
    pub fn set_hue(&mut self, red: T) {
        self.colour.set_hue(red);
    }

    /// Set the `saturation` component.
    #[inline]
    pub fn set_saturation(&mut self, green: T) {
        self.colour.set_saturation(green);
    }

    /// Set the `value` component.
    #[inline]
    pub fn set_value(&mut self, blue: T) {
        self.colour.set_value(blue);
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

impl_transparent_colour!(HsvAlpha<T>, Hsv<T>, 3);
impl_transparent_convert!(HsvAlpha<T>, Hsv<T>);
impl_transparent_display!(HsvAlpha<T>);
