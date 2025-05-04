//! RGB colour representation.

use num_traits::Float;

mod colour;
mod convert;
mod fmt;

/// RGB colour representation.
#[derive(Debug, Clone, Copy)]
pub struct Rgb<T: Float + Send + Sync> {
    /// Red component.
    red: T,
    /// Green component.
    green: T,
    /// Blue component.
    blue: T,
}

impl<T: Float + Send + Sync> Rgb<T> {
    /// Create a new `Rgb` instance.
    #[inline]
    pub fn new(red: T, green: T, blue: T) -> Self {
        debug_assert!(!(red < T::zero() || red > T::one()), "Red component must be between 0 and 1.");
        debug_assert!(
            !(green < T::zero() || green > T::one()),
            "Green component must be between 0 and 1."
        );
        debug_assert!(
            !(blue < T::zero() || blue > T::one()),
            "Blue component must be between 0 and 1."
        );
        Self { red, green, blue }
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

    /// Set the red component.
    #[inline]
    pub fn set_red(&mut self, red: T) {
        debug_assert!(red >= T::zero() && red <= T::one(), "Red component must be between 0 and 1.");
        self.red = red;
    }

    /// Set the green component.
    #[inline]
    pub fn set_green(&mut self, green: T) {
        debug_assert!(
            green >= T::zero() && green <= T::one(),
            "Green component must be between 0 and 1."
        );
        self.green = green;
    }

    /// Set the blue component.
    #[inline]
    pub fn set_blue(&mut self, blue: T) {
        debug_assert!(
            blue >= T::zero() && blue <= T::one(),
            "Blue component must be between 0 and 1."
        );
        self.blue = blue;
    }
}
