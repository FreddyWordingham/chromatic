//! RGB colour representation.

use num_traits::Float;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    config::PRINT_BLOCK,
    error::{ChromaticError, Result},
    spaces::{Grey, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, RgbAlpha, Srgb, SrgbAlpha, Xyz, XyzAlpha},
    traits::{Colour, Convert},
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
    /// Create a new `Rgb` instance with validation.
    ///
    /// # Errors
    ///
    /// Returns an error if any component is outside the range [0, 1].
    pub fn new(red: T, green: T, blue: T) -> Result<Self> {
        Self::validate_component(red, "red")?;
        Self::validate_component(green, "green")?;
        Self::validate_component(blue, "blue")?;

        Ok(Self { red, green, blue })
    }

    /// Validate a single component is in range [0, 1].
    fn validate_component(value: T, name: &str) -> Result<()> {
        if value < T::zero() || value > T::one() {
            return Err(ChromaticError::InvalidColour(format!(
                "{} component ({}) must be between 0 and 1",
                name,
                value.to_f64().unwrap_or(f64::NAN)
            )));
        }
        Ok(())
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

    /// Set the `red` component with validation.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 1].
    pub fn set_red(&mut self, red: T) -> Result<()> {
        Self::validate_component(red, "red")?;
        self.red = red;
        Ok(())
    }

    /// Set the `green` component with validation.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 1].
    pub fn set_green(&mut self, green: T) -> Result<()> {
        Self::validate_component(green, "green")?;
        self.green = green;
        Ok(())
    }

    /// Set the `blue` component with validation.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the range [0, 1].
    pub fn set_blue(&mut self, blue: T) -> Result<()> {
        Self::validate_component(blue, "blue")?;
        self.blue = blue;
        Ok(())
    }

    /// Set all components at once with validation.
    ///
    /// # Errors
    ///
    /// Returns an error if any component is outside the range [0, 1].
    pub fn set_components(&mut self, red: T, green: T, blue: T) -> Result<()> {
        Self::validate_component(red, "red")?;
        Self::validate_component(green, "green")?;
        Self::validate_component(blue, "blue")?;

        self.red = red;
        self.green = green;
        self.blue = blue;
        Ok(())
    }
}

impl<T: Float + Send + Sync> Colour<T, 3> for Rgb<T> {
    fn from_hex(hex: &str) -> Result<Self> {
        let components = hex
            .trim()
            .strip_prefix('#')
            .ok_or_else(|| ChromaticError::ColourParsing("Missing '#' prefix".to_string()))?;
        let mut chars = components.chars();
        let (red, green, blue) = match components.len() {
            // Short form: #RGB
            3 => {
                let r_digit = chars.next().unwrap();
                let g_digit = chars.next().unwrap();
                let b_digit = chars.next().unwrap();

                let red = u8::from_str_radix(&r_digit.to_string(), 16)
                    .map_err(|e| ChromaticError::ColourParsing(format!("Invalid hex digit: {e}")))?;
                let green = u8::from_str_radix(&g_digit.to_string(), 16)
                    .map_err(|e| ChromaticError::ColourParsing(format!("Invalid hex digit: {e}")))?;
                let blue = u8::from_str_radix(&b_digit.to_string(), 16)
                    .map_err(|e| ChromaticError::ColourParsing(format!("Invalid hex digit: {e}")))?;

                // Expand short form (e.g., #F00 becomes #FF0000)
                let scaled_red = T::from(red).ok_or_else(|| ChromaticError::Math("Failed to convert red value".to_string()))?
                    * T::from(17).unwrap()
                    / T::from(255).unwrap();
                let scaled_green = T::from(green)
                    .ok_or_else(|| ChromaticError::Math("Failed to convert green value".to_string()))?
                    * T::from(17).unwrap()
                    / T::from(255).unwrap();
                let scaled_blue = T::from(blue)
                    .ok_or_else(|| ChromaticError::Math("Failed to convert blue value".to_string()))?
                    * T::from(17).unwrap()
                    / T::from(255).unwrap();

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

                let red = u8::from_str_radix(&format!("{r1}{r2}"), 16)
                    .map_err(|e| ChromaticError::ColourParsing(format!("Invalid hex red: {e}")))?;
                let green = u8::from_str_radix(&format!("{g1}{g2}"), 16)
                    .map_err(|e| ChromaticError::ColourParsing(format!("Invalid hex green: {e}")))?;
                let blue = u8::from_str_radix(&format!("{b1}{b2}"), 16)
                    .map_err(|e| ChromaticError::ColourParsing(format!("Invalid hex blue: {e}")))?;

                let scaled_red = T::from(red).ok_or_else(|| ChromaticError::Math("Failed to convert red value".to_string()))?
                    / T::from(255).unwrap();
                let scaled_green = T::from(green)
                    .ok_or_else(|| ChromaticError::Math("Failed to convert green value".to_string()))?
                    / T::from(255).unwrap();
                let scaled_blue = T::from(blue)
                    .ok_or_else(|| ChromaticError::Math("Failed to convert blue value".to_string()))?
                    / T::from(255).unwrap();

                (scaled_red, scaled_green, scaled_blue)
            }
            _ => return Err(ChromaticError::ColourParsing("Invalid hex format".to_string())),
        };
        Self::new(red, green, blue)
    }

    fn to_hex(&self) -> Result<String> {
        let max = T::from(255_u8).unwrap();
        let red = (self.red * max).round().to_u8().unwrap();
        let green = (self.green * max).round().to_u8().unwrap();
        let blue = (self.blue * max).round().to_u8().unwrap();
        Ok(format!("#{red:02X}{green:02X}{blue:02X}"))
    }

    fn from_bytes(bytes: [u8; 3]) -> Result<Self> {
        let max = T::from(255_u8).unwrap();
        let red = T::from(bytes[0]).unwrap() / max;
        let green = T::from(bytes[1]).unwrap() / max;
        let blue = T::from(bytes[2]).unwrap() / max;
        Self::new(red, green, blue)
    }

    fn to_bytes(self) -> Result<[u8; 3]> {
        let max = T::from(255_u8).unwrap();
        let red = (self.red * max).round().to_u8().unwrap();
        let green = (self.green * max).round().to_u8().unwrap();
        let blue = (self.blue * max).round().to_u8().unwrap();
        Ok([red, green, blue])
    }

    /// Linear interpolate between two RGB colours.
    fn lerp(lhs: &Self, rhs: &Self, t: T) -> Result<Self> {
        if t < T::zero() || t > T::one() {
            return Err(ChromaticError::Interpolation(format!(
                "Interpolation factor ({}) must be between 0 and 1",
                t.to_f64().unwrap_or(f64::NAN)
            )));
        }

        Self::new(
            lhs.red * (T::one() - t) + rhs.red * t,
            lhs.green * (T::one() - t) + rhs.green * t,
            lhs.blue * (T::one() - t) + rhs.blue * t,
        )
    }
}

impl<T: Float + Send + Sync> Convert<T> for Rgb<T> {
    fn to_grey(&self) -> Result<Grey<T>> {
        Grey::new((self.red + self.green + self.blue) / T::from(3.0).unwrap())
    }

    fn to_grey_alpha(&self) -> Result<GreyAlpha<T>> {
        GreyAlpha::new((self.red + self.green + self.blue) / T::from(3.0).unwrap(), T::one())
    }

    fn to_hsl(&self) -> Result<Hsl<T>> {
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

    fn to_hsl_alpha(&self) -> Result<HslAlpha<T>> {
        let hsl = self.to_hsl()?;
        HslAlpha::new(hsl.hue(), hsl.saturation(), hsl.lightness(), T::one())
    }

    fn to_hsv(&self) -> Result<Hsv<T>> {
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

    fn to_hsv_alpha(&self) -> Result<HsvAlpha<T>> {
        let hsv = self.to_hsv()?;
        HsvAlpha::new(hsv.hue(), hsv.saturation(), hsv.value(), T::one())
    }

    fn to_lab(&self) -> Result<Lab<T>> {
        // Convert RGB to Lab via XYZ
        self.to_xyz()?.to_lab()
    }

    fn to_lab_alpha(&self) -> Result<LabAlpha<T>> {
        let lab = self.to_lab()?;
        LabAlpha::new(lab.lightness(), lab.a_star(), lab.b_star(), T::one())
    }

    fn to_rgb(&self) -> Result<Self> {
        Ok(*self)
    }

    fn to_rgb_alpha(&self) -> Result<RgbAlpha<T>> {
        RgbAlpha::new(self.red(), self.green(), self.blue(), T::one())
    }

    fn to_srgb(&self) -> Result<Srgb<T>> {
        // Convert from linear RGB to gamma-encoded sRGB
        let r_srgb = Srgb::gamma_encode(self.red);
        let g_srgb = Srgb::gamma_encode(self.green);
        let b_srgb = Srgb::gamma_encode(self.blue);

        Srgb::new(r_srgb, g_srgb, b_srgb)
    }

    fn to_srgb_alpha(&self) -> Result<SrgbAlpha<T>> {
        let srgb = self.to_srgb()?;
        SrgbAlpha::new(srgb.red(), srgb.green(), srgb.blue(), T::one())
    }

    fn to_xyz(&self) -> Result<Xyz<T>> {
        // Convert linear RGB to XYZ using the standard sRGB transform matrix
        // This matrix is for D65 reference white
        let x = self.red() * T::from(0.412_456_4).unwrap()
            + self.green() * T::from(0.357_576_1).unwrap()
            + self.blue() * T::from(0.180_437_5).unwrap();

        let y = self.red() * T::from(0.212_672_9).unwrap()
            + self.green() * T::from(0.715_152_2).unwrap()
            + self.blue() * T::from(0.072_175_0).unwrap();

        let z = self.red() * T::from(0.019_333_9).unwrap()
            + self.green() * T::from(0.119_192_0).unwrap()
            + self.blue() * T::from(0.950_304_1).unwrap();

        Xyz::new(x, y, z)
    }

    fn to_xyz_alpha(&self) -> Result<XyzAlpha<T>> {
        let xyz = self.to_xyz()?;
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
