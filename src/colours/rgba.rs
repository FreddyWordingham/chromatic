//! Red-Green-Blue colour with alpha channel representation.

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Add, Mul, Sub},
    str::FromStr,
};
use num_traits::{Float, FromPrimitive};
use palette::{
    LinSrgba, Mix as _,
    num::{Arithmetics, Clamp, One, Real, Zero},
};

use crate::{Colour, ColourParseError};

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

impl<T> FromStr for Rgba<T>
where
    T: Float + FromPrimitive,
{
    type Err = ColourParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.trim().trim_start_matches('#');
        // support "#RGBA" or "#RRGGBBAA"
        let (r, g, b, a) = match hex.len() {
            4 => {
                // one nibble per channel, expand with *17
                let nibbles: Vec<u8> = hex
                    .chars()
                    .map(|c| u8::from_str_radix(&c.to_string(), 16))
                    .collect::<Result<_, _>>()?;
                (
                    nibbles[0].saturating_mul(17),
                    nibbles[1].saturating_mul(17),
                    nibbles[2].saturating_mul(17),
                    nibbles[3].saturating_mul(17),
                )
            }
            8 => {
                // two hex digits per channel
                let rgba = u32::from_str_radix(hex, 16)?;
                (
                    ((rgba >> 24) & 0xFF) as u8,
                    ((rgba >> 16) & 0xFF) as u8,
                    ((rgba >> 8) & 0xFF) as u8,
                    (rgba & 0xFF) as u8,
                )
            }
            len => return Err(ColourParseError::InvalidLength(len)),
        };

        // scale into [0,1] in T
        let scale = T::from_u8(255).unwrap();
        let rt = T::from_u8(r).unwrap() / scale;
        let gt = T::from_u8(g).unwrap() / scale;
        let bt = T::from_u8(b).unwrap() / scale;
        let at = T::from_u8(a).unwrap() / scale;

        Ok(Rgba::new(rt, gt, bt, at))
    }
}

impl<T: Float> Display for Rgba<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let r = self.r().to_u8().unwrap();
        let g = self.g().to_u8().unwrap();
        let b = self.b().to_u8().unwrap();
        let a = self.a().to_u8().unwrap();
        write!(f, "#{:02X}{:02X}{:02X}{:02X}", r, g, b, a)
    }
}
