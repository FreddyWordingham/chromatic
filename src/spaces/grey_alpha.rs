//! Monochrome colour with transparency representation.

use num_traits::Float;

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    num::ParseIntError,
};

use crate::{
    Colour, Convert, Grey, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, ParseColourError, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz,
    XyzAlpha, impl_transparent_colour, impl_transparent_convert, impl_transparent_display,
};

/// Grey with alpha channel.
#[derive(Debug, Clone, Copy)]
pub struct GreyAlpha<T: Float + Send + Sync> {
    /// Base colour
    colour: Grey<T>,
    /// Alpha component
    alpha: T,
}

impl<T: Float + Send + Sync> GreyAlpha<T> {
    /// Create a new `GreyAlpha` instance.
    #[inline]
    pub fn new(grey: T, alpha: T) -> Self {
        debug_assert!(
            alpha >= T::zero() && alpha <= T::one(),
            "Alpha component must be in range [0, 1]."
        );
        Self {
            colour: Grey::new(grey),
            alpha,
        }
    }

    /// Create a new `GreyAlpha` instance from a `Grey` colour and an alpha component.
    #[inline]
    fn new_colour_with_alpha(colour: Grey<T>, alpha: T) -> Self {
        debug_assert!(
            alpha >= T::zero() && alpha <= T::one(),
            "Alpha component must be in range [0, 1]."
        );
        Self { colour, alpha }
    }

    /// Get the base `colour`.
    #[inline]
    const fn colour(&self) -> &Grey<T> {
        &self.colour
    }

    /// Get the `grey` component.
    #[inline]
    pub const fn grey(&self) -> T {
        self.colour.grey()
    }

    /// Get the `alpha` component.
    #[inline]
    pub const fn alpha(&self) -> T {
        self.alpha
    }

    /// Set the `grey` component.
    #[inline]
    pub fn set_grey(&mut self, grey: T) {
        debug_assert!(
            grey >= T::zero() && grey <= T::one(),
            "Grey component must be in range [0, 1]."
        );
        self.colour.set_grey(grey);
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

impl_transparent_colour!(GreyAlpha<T>, Grey<T>, 1);
impl_transparent_convert!(GreyAlpha<T>, Grey<T>);
impl_transparent_display!(GreyAlpha<T>);
