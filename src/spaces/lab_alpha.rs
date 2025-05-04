//! Lab colour with transparency representation.

use num_traits::Float;

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    num::ParseIntError,
};

use crate::{
    Colour, Convert, Grey, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, ParseColourError, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz,
    XyzAlpha, impl_transparent_colour, impl_transparent_convert, impl_transparent_display,
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
    #[inline]
    pub fn new(l: T, a: T, b: T, alpha: T) -> Self {
        debug_assert!(
            alpha >= T::zero() && alpha <= T::one(),
            "Alpha component must be in range [0, 1]."
        );
        Self {
            colour: Lab::new(l, a, b),
            alpha,
        }
    }

    /// Create a new `LabAlpha` instance from a `Lab` colour and an alpha component.
    #[inline]
    fn new_colour_with_alpha(colour: Lab<T>, alpha: T) -> Self {
        debug_assert!(
            alpha >= T::zero() && alpha <= T::one(),
            "Alpha component must be in range [0, 1]."
        );
        Self { colour, alpha }
    }

    /// Get the base colour.
    #[inline]
    const fn colour(&self) -> &Lab<T> {
        &self.colour
    }

    /// Get the `lightness` component.
    #[inline]
    pub const fn lightness(&self) -> T {
        self.colour.lightness()
    }

    /// Get the `a_star` component.
    #[inline]
    pub const fn a_star(&self) -> T {
        self.colour.a_star()
    }

    /// Get the `b_star` component.
    #[inline]
    pub const fn b_star(&self) -> T {
        self.colour.b_star()
    }

    /// Get the `alpha` component.
    #[inline]
    pub const fn alpha(&self) -> T {
        self.alpha
    }

    /// Set the `lightness` component.
    #[inline]
    pub fn set_lightness(&mut self, l: T) {
        self.colour.set_lightness(l);
    }

    /// Set the `a_star` component.
    #[inline]
    pub fn set_a_star(&mut self, a: T) {
        self.colour.set_a_star(a);
    }

    /// Set the `b_star` component.
    #[inline]
    pub fn set_b_star(&mut self, b: T) {
        self.colour.set_b_star(b);
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

    /// Calculate perceptual color difference in Lab space (CIE76 Delta E),
    /// ignoring the alpha channel.
    #[inline]
    pub fn delta_e(&self, other: &Self) -> T {
        self.colour.delta_e(&other.colour)
    }

    /// Calculate perceptual color difference using the improved CIE94 Delta E formula,
    /// ignoring the alpha channel.
    #[inline]
    pub fn delta_e94(&self, other: &Self) -> T {
        self.colour.delta_e94(&other.colour)
    }
}

impl_transparent_colour!(LabAlpha<T>, Lab<T>, 3);
impl_transparent_convert!(LabAlpha<T>, Lab<T>);
impl_transparent_display!(LabAlpha<T>);
