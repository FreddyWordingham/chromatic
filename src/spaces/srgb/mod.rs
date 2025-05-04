//! sRGB colour representation.

use num_traits::Float;

mod colour;
mod convert;
mod fmt;

/// sRGB colour representation.
///
/// sRGB is a standard RGB color space widely used in digital displays, image formats, and web content.
/// It uses a specific non-linear gamma encoding to represent colors in a way that's perceptually more
/// uniform than linear RGB.
#[derive(Debug, Clone, Copy)]
pub struct Srgb<T: Float + Send + Sync> {
    /// Red component in range [0, 1].
    red: T,
    /// Green component in range [0, 1].
    green: T,
    /// Blue component in range [0, 1].
    blue: T,
}

impl<T: Float + Send + Sync> Srgb<T> {
    /// Create a new `Srgb` instance.
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

    /// Apply the standard sRGB gamma encoding to a linear component.
    ///
    /// This converts a linear RGB value to an sRGB value using the standard
    /// piecewise encoding function specified in the sRGB standard.
    #[inline]
    pub fn gamma_encode(linear: T) -> T {
        if linear <= T::from(0.0031308).unwrap() {
            T::from(12.92).unwrap() * linear
        } else {
            T::from(1.055).unwrap() * linear.powf(T::from(1.0 / 2.4).unwrap()) - T::from(0.055).unwrap()
        }
    }

    /// Apply the standard sRGB gamma decoding to an sRGB component.
    ///
    /// This converts an sRGB value to a linear RGB value using the standard
    /// piecewise decoding function specified in the sRGB standard.
    #[inline]
    pub fn gamma_decode(srgb: T) -> T {
        if srgb <= T::from(0.04045).unwrap() {
            srgb / T::from(12.92).unwrap()
        } else {
            ((srgb + T::from(0.055).unwrap()) / T::from(1.055).unwrap()).powf(T::from(2.4).unwrap())
        }
    }
}
