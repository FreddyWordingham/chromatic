//! RGB colour representation with transparency.

use core::{fmt::Display, ops::AddAssign};
use num_traits::Float;

mod colour;
mod convert;
mod eq;
mod fmt;

/// RGB colour representation with transparency.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Rgba<T: Float> {
    /// Red component.
    red: T,
    /// Green component.
    green: T,
    /// Blue component.
    blue: T,
    /// Alpha component.
    alpha: T,
}

impl<T: Display + AddAssign + Float> Rgba<T> {
    /// Create a new `Rgba` instance.
    ///
    /// # Panics
    ///
    /// Panics if any component is not in [0, 1].
    #[inline]
    pub fn new(red: T, green: T, blue: T, alpha: T) -> Self {
        assert!(!(red < T::zero() || red > T::one()), "Red component {red} out of [0, 1].");
        assert!(
            !(green < T::zero() || green > T::one()),
            "Green component {green} out of [0, 1]."
        );
        assert!(!(blue < T::zero() || blue > T::one()), "Blue component {blue} out of [0, 1].");
        assert!(
            !(alpha < T::zero() || alpha > T::one()),
            "Alpha component {alpha} out of [0, 1]."
        );
        Self { red, green, blue, alpha }
    }

    /// Get the red component.
    #[inline]
    pub const fn red(&self) -> T {
        self.red
    }

    /// Get the green component.
    #[inline]
    pub const fn green(&self) -> T {
        self.green
    }

    /// Get the blue component.
    #[inline]
    pub const fn blue(&self) -> T {
        self.blue
    }

    /// Get the alpha component.
    #[inline]
    pub const fn alpha(&self) -> T {
        self.alpha
    }

    /// Set the red component.
    ///
    /// # Panics
    ///
    /// Panics if the value is not in [0, 1].
    #[inline]
    pub fn set_red(&mut self, red: T) {
        assert!(red >= T::zero() && red <= T::one(), "Red component must be between 0 and 1.");
        self.red = red;
    }

    /// Set the green component.
    ///
    /// # Panics
    ///
    /// Panics if the value is not in [0, 1].
    #[inline]
    pub fn set_green(&mut self, green: T) {
        assert!(
            green >= T::zero() && green <= T::one(),
            "Green component must be between 0 and 1."
        );
        self.green = green;
    }

    /// Set the blue component.
    ///
    /// # Panics
    ///
    /// Panics if the value is not in [0, 1].
    #[inline]
    pub fn set_blue(&mut self, blue: T) {
        assert!(
            blue >= T::zero() && blue <= T::one(),
            "Blue component must be between 0 and 1."
        );
        self.blue = blue;
    }

    /// Set the alpha component.
    ///
    /// # Panics
    ///
    /// Panics if the value is not in [0, 1].
    #[inline]
    pub fn set_alpha(&mut self, alpha: T) {
        assert!(
            alpha >= T::zero() && alpha <= T::one(),
            "Alpha component must be between 0 and 1."
        );
        self.alpha = alpha;
    }
}
