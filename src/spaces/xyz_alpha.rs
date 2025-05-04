//! XYZ colour with transparency representation.

use num_traits::Float;

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    num::ParseIntError,
};

use crate::{
    Colour, Convert, Grey, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, ParseColourError, Rgb, RgbAlpha, Srgb,
    SrgbAlpha, Xyz, impl_transparent_colour, impl_transparent_convert, impl_transparent_display,
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
    #[inline]
    pub fn new(x: T, y: T, z: T, alpha: T) -> Self {
        debug_assert!(
            alpha >= T::zero() && alpha <= T::one(),
            "Alpha component must be in range [0, 1]."
        );
        Self {
            colour: Xyz::new(x, y, z),
            alpha,
        }
    }

    /// Create a new `XyzAlpha` instance from a `Xyz` colour and an alpha component.
    #[inline]
    fn new_colour_with_alpha(colour: Xyz<T>, alpha: T) -> Self {
        debug_assert!(
            alpha >= T::zero() && alpha <= T::one(),
            "Alpha component must be in range [0, 1]."
        );
        Self { colour, alpha }
    }

    /// Get the base `colour`.
    #[inline]
    const fn colour(&self) -> &Xyz<T> {
        &self.colour
    }

    /// Get the `x` component.
    #[inline]
    pub const fn x(&self) -> T {
        self.colour.x()
    }

    /// Get the `y` component.
    #[inline]
    pub const fn y(&self) -> T {
        self.colour.y()
    }

    /// Get the `z` component.
    #[inline]
    pub const fn z(&self) -> T {
        self.colour.z()
    }

    /// Get the `alpha` component.
    #[inline]
    pub const fn alpha(&self) -> T {
        self.alpha
    }

    /// Set the `x` component.
    #[inline]
    pub fn set_x(&mut self, x: T) {
        self.colour.set_x(x);
    }

    /// Set the `y` component.
    #[inline]
    pub fn set_y(&mut self, y: T) {
        self.colour.set_y(y);
    }

    /// Set the `z` component.
    #[inline]
    pub fn set_z(&mut self, z: T) {
        self.colour.set_z(z);
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

impl_transparent_colour!(XyzAlpha<T>, Xyz<T>, 3);
impl_transparent_convert!(XyzAlpha<T>, Xyz<T>);
impl_transparent_display!(XyzAlpha<T>);
