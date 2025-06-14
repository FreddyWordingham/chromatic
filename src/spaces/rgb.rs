//! RGB colour representation.

use num_traits::Float;
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    num::ParseIntError,
};

use crate::{
    Colour, Convert, Grey, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, ParseColourError, RgbAlpha, Srgb, SrgbAlpha,
    Xyz, XyzAlpha, config::PRINT_BLOCK,
};

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

    /// Get the `red` component.
    pub const fn red(&self) -> T {
        self.red
    }

    /// Get the `green` component.
    pub const fn green(&self) -> T {
        self.green
    }

    /// Get the `blue` component.
    pub const fn blue(&self) -> T {
        self.blue
    }

    /// Set the `red` component.
    pub fn set_red(&mut self, red: T) {
        debug_assert!(red >= T::zero() && red <= T::one(), "Red component must be between 0 and 1.");
        self.red = red;
    }

    /// Set the `green` component.
    pub fn set_green(&mut self, green: T) {
        debug_assert!(
            green >= T::zero() && green <= T::one(),
            "Green component must be between 0 and 1."
        );
        self.green = green;
    }

    /// Set the `blue` component.
    pub fn set_blue(&mut self, blue: T) {
        debug_assert!(
            blue >= T::zero() && blue <= T::one(),
            "Blue component must be between 0 and 1."
        );
        self.blue = blue;
    }
}

impl<T: Float + Send + Sync> Colour<T, 3> for Rgb<T> {
    fn from_hex(hex: &str) -> Result<Self, ParseColourError<ParseIntError>> {
        let components = hex.trim().strip_prefix('#').ok_or(ParseColourError::InvalidFormat)?;
        let mut chars = components.chars();
        let (red, green, blue) = match components.len() {
            // Short form: #RGB
            3 => {
                let r_digit = chars.next().unwrap();
                let g_digit = chars.next().unwrap();
                let b_digit = chars.next().unwrap();

                let red = u8::from_str_radix(&r_digit.to_string(), 16).map_err(ParseColourError::ParseHex)?;
                let green = u8::from_str_radix(&g_digit.to_string(), 16).map_err(ParseColourError::ParseHex)?;
                let blue = u8::from_str_radix(&b_digit.to_string(), 16).map_err(ParseColourError::ParseHex)?;

                // Expand short form (e.g., #F00 becomes #FF0000)
                let scaled_red = T::from(red * 17).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                let scaled_green = T::from(green * 17).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                let scaled_blue = T::from(blue * 17).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();

                (scaled_red, scaled_green, scaled_blue)
            }
            // Long form: #RRGGBB
            6 => {
                let r1 = chars.next().unwrap().to_string();
                let r2 = chars.next().unwrap().to_string();
                let g1 = chars.next().unwrap().to_string();
                let g2 = chars.next().unwrap().to_string();
                let b1 = chars.next().unwrap().to_string();
                let b2 = chars.next().unwrap().to_string();

                let red = u8::from_str_radix(&format!("{r1}{r2}"), 16).map_err(ParseColourError::ParseHex)?;
                let green = u8::from_str_radix(&format!("{g1}{g2}"), 16).map_err(ParseColourError::ParseHex)?;
                let blue = u8::from_str_radix(&format!("{b1}{b2}"), 16).map_err(ParseColourError::ParseHex)?;

                let scaled_red = T::from(red).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                let scaled_green = T::from(green).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();
                let scaled_blue = T::from(blue).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();

                (scaled_red, scaled_green, scaled_blue)
            }
            _ => return Err(ParseColourError::InvalidFormat),
        };
        Ok(Self::new(red, green, blue))
    }

    fn to_hex(&self) -> String {
        let max = T::from(255_u8).unwrap();
        let red = (self.red * max).round().to_u8().unwrap();
        let green = (self.green * max).round().to_u8().unwrap();
        let blue = (self.blue * max).round().to_u8().unwrap();
        format!("#{red:02X}{green:02X}{blue:02X}")
    }

    fn from_bytes(bytes: [u8; 3]) -> Self {
        let max = T::from(255_u8).unwrap();
        let red = T::from(bytes[0]).unwrap() / max;
        let green = T::from(bytes[1]).unwrap() / max;
        let blue = T::from(bytes[2]).unwrap() / max;
        Self::new(red, green, blue)
    }

    fn to_bytes(self) -> [u8; 3] {
        let max = T::from(255_u8).unwrap();
        let red = (self.red * max).round().to_u8().unwrap();
        let green = (self.green * max).round().to_u8().unwrap();
        let blue = (self.blue * max).round().to_u8().unwrap();
        [red, green, blue]
    }

    /// Linear interpolate between two RGB colours.
    fn lerp(lhs: &Self, rhs: &Self, t: T) -> Self {
        debug_assert!(
            t >= T::zero() && t <= T::one(),
            "Interpolation factor must be in range [0, 1]."
        );
        Self::new(
            lhs.red * (T::one() - t) + rhs.red * t,
            lhs.green * (T::one() - t) + rhs.green * t,
            lhs.blue * (T::one() - t) + rhs.blue * t,
        )
    }
}

impl<T: Float + Send + Sync> Convert<T> for Rgb<T> {
    fn to_grey(&self) -> Grey<T> {
        Grey::new((self.red + self.green + self.blue) / T::from(3.0).unwrap())
    }

    fn to_grey_alpha(&self) -> GreyAlpha<T> {
        GreyAlpha::new((self.red + self.green + self.blue) / T::from(3.0).unwrap(), T::one())
    }

    fn to_hsl(&self) -> Hsl<T> {
        let r = self.red();
        let g = self.green();
        let b = self.blue();

        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let delta = max - min;

        // Calculate lightness
        let lightness = (max + min) / T::from(2.0).unwrap();

        // If max equals min, the color is a shade of gray (no hue or saturation)
        if delta.abs() < T::epsilon() {
            return Hsl::new(T::zero(), T::zero(), lightness);
        }

        // Calculate saturation
        let saturation = if lightness <= T::from(0.5).unwrap() {
            delta / (max + min)
        } else {
            delta / (T::from(2.0).unwrap() - max - min)
        };

        // Calculate hue
        let hue = if r.abs_sub(max).abs() < T::epsilon() {
            // Red is max
            let segment = (g - b) / delta;
            let shift = T::zero();
            let segment_6 = segment / T::from(6.0).unwrap();

            // If green is less than blue, add 1.0 (360 degrees)
            if g < b {
                T::from(360.0).unwrap() + shift + segment_6 * T::from(360.0).unwrap()
            } else {
                shift + segment_6 * T::from(360.0).unwrap()
            }
        } else if g.abs_sub(max).abs() < T::epsilon() {
            // Green is max
            let segment = (b - r) / delta;
            let shift = T::from(1.0 / 3.0).unwrap();
            let segment_6 = segment / T::from(6.0).unwrap();

            shift + segment_6 * T::from(360.0).unwrap()
        } else {
            // Blue is max
            let segment = (r - g) / delta;
            let shift = T::from(2.0 / 3.0).unwrap();
            let segment_6 = segment / T::from(6.0).unwrap();

            shift + segment_6 * T::from(360.0).unwrap()
        };

        // Make sure hue is in the range [0, 360)
        let mut normalized_hue = hue;
        while normalized_hue >= T::from(360.0).unwrap() {
            normalized_hue = normalized_hue - T::from(360.0).unwrap();
        }
        while normalized_hue < T::zero() {
            normalized_hue = normalized_hue + T::from(360.0).unwrap();
        }

        Hsl::new(normalized_hue, saturation, lightness)
    }

    fn to_hsl_alpha(&self) -> HslAlpha<T> {
        let hsl = self.to_hsl();
        HslAlpha::new(hsl.hue(), hsl.saturation(), hsl.lightness(), T::one())
    }

    fn to_hsv(&self) -> Hsv<T> {
        let r = self.red();
        let g = self.green();
        let b = self.blue();

        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let delta = max - min;

        let value = max;

        let zero = T::zero();
        let sixty = T::from(60.0).unwrap();
        let two = T::from(2.0).unwrap();
        let four = T::from(4.0).unwrap();
        let six = T::from(6.0).unwrap();

        let saturation = if max == zero { zero } else { delta / max };

        let hue = if delta == zero {
            zero
        } else if max == r {
            let mut h = (g - b) / delta;
            if h < zero {
                h = h + six;
            }
            h * sixty
        } else if max == g {
            ((b - r) / delta + two) * sixty
        } else {
            ((r - g) / delta + four) * sixty
        };

        Hsv::new(hue, saturation, value)
    }

    fn to_hsv_alpha(&self) -> HsvAlpha<T> {
        let hsv = self.to_hsv();
        HsvAlpha::new(hsv.hue(), hsv.saturation(), hsv.value(), T::one())
    }

    fn to_lab(&self) -> Lab<T> {
        // Convert RGB to Lab via XYZ
        self.to_xyz().to_lab()
    }

    fn to_lab_alpha(&self) -> LabAlpha<T> {
        let lab = self.to_lab();
        LabAlpha::new(lab.lightness(), lab.a_star(), lab.b_star(), T::one())
    }

    fn to_rgb(&self) -> Self {
        *self
    }

    fn to_rgb_alpha(&self) -> RgbAlpha<T> {
        RgbAlpha::new(self.red(), self.green(), self.blue(), T::one())
    }

    fn to_srgb(&self) -> Srgb<T> {
        // Convert from linear RGB to gamma-encoded sRGB
        let r_srgb = Srgb::gamma_encode(self.red);
        let g_srgb = Srgb::gamma_encode(self.green);
        let b_srgb = Srgb::gamma_encode(self.blue);

        Srgb::new(r_srgb, g_srgb, b_srgb)
    }

    fn to_srgb_alpha(&self) -> SrgbAlpha<T> {
        let srgb = self.to_srgb();
        SrgbAlpha::new(srgb.red(), srgb.green(), srgb.blue(), T::one())
    }

    fn to_xyz(&self) -> Xyz<T> {
        // Convert linear RGB to XYZ using the standard sRGB transform matrix
        // This matrix is for D65 reference white
        let x = self.red() * T::from(0.4124564).unwrap()
            + self.green() * T::from(0.3575761).unwrap()
            + self.blue() * T::from(0.1804375).unwrap();

        let y = self.red() * T::from(0.2126729).unwrap()
            + self.green() * T::from(0.7151522).unwrap()
            + self.blue() * T::from(0.0721750).unwrap();

        let z = self.red() * T::from(0.0193339).unwrap()
            + self.green() * T::from(0.1191920).unwrap()
            + self.blue() * T::from(0.9503041).unwrap();

        Xyz::new(x, y, z)
    }

    fn to_xyz_alpha(&self) -> XyzAlpha<T> {
        let xyz = self.to_xyz();
        XyzAlpha::new(xyz.x(), xyz.y(), xyz.z(), T::one())
    }
}

impl<T: Float + Send + Sync> Display for Rgb<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        let max = T::from(255_i32).unwrap();
        let red = (self.red * max).round().to_u8().unwrap();
        let green = (self.green * max).round().to_u8().unwrap();
        let blue = (self.blue * max).round().to_u8().unwrap();
        write!(fmt, "\x1b[38;2;{red};{green};{blue}m{PRINT_BLOCK}\x1b[0m")
    }
}
