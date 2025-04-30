//! Red-Green-Blue colour representation.

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Add, Mul, Sub},
    str::FromStr,
};
use enterpolation::Merge;
use num_traits::Float;
use palette::{
    LinSrgb, Mix as _,
    num::{Arithmetics, Clamp, One, Real, Zero},
};

use crate::{Colour, ColourParseError};

/// RGB colour.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct Rgb<T: Float>(pub LinSrgb<T>);

impl<T: Float> Rgb<T> {
    /// Create a new `Rgb` instance.
    ///
    /// # Panics
    ///
    /// Panics if any of the components are not in the range [0, 1].
    #[inline]
    pub fn new(red: T, green: T, blue: T) -> Self {
        assert!(
            red >= <T as num_traits::Zero>::zero() && red <= <T as num_traits::One>::one(),
            "Red component must be between 0 and 1"
        );
        assert!(
            green >= <T as num_traits::Zero>::zero() && green <= <T as num_traits::One>::one(),
            "Green component must be between 0 and 1"
        );
        assert!(
            blue >= <T as num_traits::Zero>::zero() && blue <= <T as num_traits::One>::one(),
            "Blue component must be between 0 and 1"
        );
        Self(LinSrgb::new(red, green, blue))
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
}

impl<T: Float> Colour<T> for Rgb<T>
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

impl<T: Float + Real + Zero + One + Clamp> Merge<T> for Rgb<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Arithmetics,
{
    fn merge(self, other: Self, t: T) -> Self {
        self.lerp(&other, t)
    }
}

impl<T> FromStr for Rgb<T>
where
    T: Float,
{
    type Err = ColourParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.trim().trim_start_matches('#');
        // support "#RGB" or "#RRGGBB"
        let (r, g, b) = match hex.len() {
            3 => {
                // expand each nibble: "FAB" -> "FF","AA","BB"
                let chars: Vec<u8> = hex
                    .bytes()
                    .map(|c| u8::from_str_radix(std::str::from_utf8(&[c]).unwrap(), 16))
                    .collect::<Result<_, _>>()?;
                (chars[0] * 17, chars[1] * 17, chars[2] * 17)
            }
            6 => {
                let rgb = u32::from_str_radix(hex, 16)?;
                (((rgb >> 16) & 0xFF) as u8, ((rgb >> 8) & 0xFF) as u8, (rgb & 0xFF) as u8)
            }
            len => return Err(ColourParseError::InvalidLength(len)),
        };

        let scale = T::from(255).unwrap();
        let rt = T::from(r).unwrap() / scale;
        let gt = T::from(g).unwrap() / scale;
        let bt = T::from(b).unwrap() / scale;

        Ok(Rgb::new(rt, gt, bt))
    }
}

impl<T: Float> Display for Rgb<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let r = self.r().to_u8().unwrap();
        let g = self.g().to_u8().unwrap();
        let b = self.b().to_u8().unwrap();
        write!(f, "#{:02X}{:02X}{:02X}", r, g, b)
    }
}
