//! Red-Green-Blue colour with alpha channel representation.

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Add, Mul, Sub},
    str::FromStr,
};
use num_traits::Float;
use palette::{
    LinSrgba, Mix as _,
    num::{Arithmetics, Clamp, One, Real, Zero},
};

use crate::{Channel, Colour, ColourParseError};

/// Colour with alpha channel.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct Rgba<T: Float>(pub LinSrgba<T>);

impl<T: Float> Rgba<T> {
    /// Create a new `Rgba` instance.
    ///
    /// # Panics
    ///
    /// Panics if any of the components are not in the range [0, 1].
    #[inline]
    pub fn new(red: T, green: T, blue: T, alpha: T) -> Self {
        assert!(red >= T::zero() && red <= T::one(), "Red component must be between 0 and 1");
        assert!(
            green >= T::zero() && green <= T::one(),
            "Green component must be between 0 and 1"
        );
        assert!(
            blue >= T::zero() && blue <= T::one(),
            "Blue component must be between 0 and 1"
        );
        assert!(
            alpha >= T::zero() && alpha <= T::one(),
            "Alpha component must be between 0 and 1"
        );
        Self(LinSrgba::new(red, green, blue, alpha))
    }

    /// Get the red component.
    #[expect(clippy::missing_const_for_fn, reason = "This method can not be const.")]
    #[inline]
    pub fn r(&self) -> T {
        self.0.red
    }

    /// Get the green component.
    #[expect(clippy::missing_const_for_fn, reason = "This method can not be const.")]
    #[inline]
    pub fn g(&self) -> T {
        self.0.green
    }

    /// Get the blue component.
    #[expect(clippy::missing_const_for_fn, reason = "This method can not be const.")]
    #[inline]
    pub fn b(&self) -> T {
        self.0.blue
    }

    /// Get the alpha component.
    #[expect(clippy::missing_const_for_fn, reason = "This method can not be const.")]
    #[inline]
    pub fn a(&self) -> T {
        self.0.alpha
    }
}

impl<T: Float> Colour<T> for Rgba<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Real + Zero + One + Arithmetics + Clamp,
{
    #[expect(clippy::min_ident_chars, reason = "The variable `t` is commonly used in lerp functions.")]
    #[inline]
    fn lerp(&self, other: &Self, t: T) -> Self {
        assert!(
            t >= <T as num_traits::Zero>::zero() && t <= <T as num_traits::One>::one(),
            "Lerp factor must be between 0 and 1"
        );
        Self(self.0.mix(other.0, t))
    }
}

impl<T: Float + Channel> FromStr for Rgba<T> {
    type Err = ColourParseError;

    #[expect(
        clippy::min_ident_chars,
        reason = "The variable `s` is commonly used in string parsing functions."
    )]
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.trim().trim_start_matches('#');
        if hex.len() != 8 {
            return Err(ColourParseError::InvalidLength(hex.len()));
        }

        let rgba = u32::from_str_radix(hex, 16)?;
        let red = u8::try_from((rgba >> 24i32) & 0xFF)?;
        let green = u8::try_from((rgba >> 16i32) & 0xFF)?;
        let blue = u8::try_from((rgba >> 8i32) & 0xFF)?;
        let alpha = u8::try_from(rgba & 0xFF)?;

        Ok(Self::new(
            T::from_u8(red),
            T::from_u8(green),
            T::from_u8(blue),
            T::from_u8(alpha),
        ))
    }
}

impl<T: Float + Channel> Display for Rgba<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let r = self.r().to_u8().unwrap();
        let g = self.g().to_u8().unwrap();
        let b = self.b().to_u8().unwrap();
        let a = self.a().to_u8().unwrap();
        write!(f, "#{:02X}{:02X}{:02X}{:02X}", r, g, b, a)
    }
}
