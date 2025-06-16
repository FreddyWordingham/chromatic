//! Colours with transparency.

/// Macro to implement the `Colour` trait for transparent colour types.
#[macro_export]
macro_rules! impl_transparent_colour {
    ($type:ty, $base:ty, $base_components:literal) => {
        impl<T: Float + Send + Sync> Colour<T, { $base_components + 1 }> for $type {
            fn from_hex(hex: &str) -> Result<Self> {
                let hex = hex.trim();

                // Check for # prefix
                let components = hex
                    .strip_prefix('#')
                    .ok_or_else(|| $crate::error::ColourParsingError::MissingHexPrefix(hex.to_string()))?;

                let chars: Vec<char> = components.chars().collect();

                match chars.len() {
                    // Short form with alpha (e.g., #RGB + A for RGBA)
                    val if val == $base_components + 1 => {
                        // Parse the base colour part
                        let colour_part: String = chars[0..$base_components].iter().collect::<String>();
                        let colour_part = format!("#{}", colour_part);
                        let colour = <$base>::from_hex(&colour_part)?;

                        // Parse alpha (single hex digit)
                        let alpha_char = chars[$base_components];
                        let alpha_val = $crate::error::parse_hex_component(&alpha_char.to_string(), "alpha")?;

                        // Expand from single hex digit (e.g., F -> FF)
                        let expanded_alpha = alpha_val * 17;
                        let alpha = $crate::error::u8_to_component(expanded_alpha, $crate::error::safe_constant(255.0)?)?;

                        Self::new_colour_with_alpha(colour, alpha)
                    }
                    // Long form with alpha (e.g., #RRGGBB + AA for RGBA)
                    val if val == $base_components * 2 + 2 => {
                        // Parse the base colour part
                        let colour_part: String = chars[0..($base_components * 2)].iter().collect::<String>();
                        let colour_part = format!("#{}", colour_part);
                        let colour = <$base>::from_hex(&colour_part)?;

                        // Parse alpha (two hex digits)
                        let alpha_hex: String = chars[($base_components * 2)..].iter().collect();
                        let alpha_val = $crate::error::parse_hex_component(&alpha_hex, "alpha")?;

                        let alpha = $crate::error::u8_to_component(alpha_val, $crate::error::safe_constant(255.0)?)?;

                        Self::new_colour_with_alpha(colour, alpha)
                    }
                    _ => Err($crate::error::ColourParsingError::InvalidHexLength { actual: chars.len() }.into()),
                }
            }

            fn to_hex(&self) -> Result<String> {
                let scale = $crate::error::safe_constant(255.0)?;
                let colour_hex = self.colour().to_hex()?;
                let alpha = $crate::error::component_to_u8(self.alpha(), "alpha", scale)?;

                // Remove # from colour hex and add alpha
                let colour_part: String = colour_hex.chars().skip(1).collect();
                Ok(format!("#{}{:02X}", colour_part, alpha))
            }

            fn from_bytes(bytes: [u8; $base_components + 1]) -> Result<Self> {
                let scale = $crate::error::safe_constant(255.0)?;

                // Extract base colour bytes
                let mut base_bytes = [0_u8; $base_components];
                for i in 0..$base_components {
                    base_bytes[i] = bytes[i];
                }

                let colour = <$base>::from_bytes(base_bytes)?;
                let alpha = $crate::error::u8_to_component(bytes[$base_components], scale)?;

                Self::new_colour_with_alpha(colour, alpha)
            }

            fn to_bytes(self) -> Result<[u8; $base_components + 1]> {
                let scale = $crate::error::safe_constant(255.0)?;

                let base_bytes = self.colour().to_bytes()?;
                let alpha = $crate::error::component_to_u8(self.alpha(), "alpha", scale)?;

                // Create result array
                let mut result = [0_u8; $base_components + 1];

                // Copy base colour bytes
                for i in 0..$base_components {
                    result[i] = base_bytes[i];
                }

                // Add alpha byte
                result[$base_components] = alpha;

                Ok(result)
            }

            fn lerp(lhs: &Self, rhs: &Self, t: T) -> Result<Self> {
                $crate::error::validate_interpolation_factor(t)?;

                // Interpolate base colour
                let colour = <$base>::lerp(lhs.colour(), rhs.colour(), t)?;

                // Interpolate alpha
                let alpha = lhs.alpha() * (T::one() - t) + rhs.alpha() * t;

                Self::new_colour_with_alpha(colour, alpha)
            }
        }
    };
}

/// Macro to implement the `Convert` trait for transparent colour types.
#[macro_export]
macro_rules! impl_transparent_convert {
    ($type:ty, $base:ty) => {
        impl<T: Float + Send + Sync> Convert<T> for $type {
            fn to_grey(&self) -> $crate::error::Result<Grey<T>> {
                self.colour().to_grey()
            }

            fn to_grey_alpha(&self) -> $crate::error::Result<GreyAlpha<T>> {
                let grey = self.colour().to_grey()?;
                GreyAlpha::new(grey.grey(), self.alpha())
            }

            fn to_hsl(&self) -> $crate::error::Result<Hsl<T>> {
                self.colour().to_hsl()
            }

            fn to_hsl_alpha(&self) -> $crate::error::Result<HslAlpha<T>> {
                let hsl = self.colour().to_hsl()?;
                HslAlpha::new(hsl.hue(), hsl.saturation(), hsl.lightness(), self.alpha())
            }

            fn to_hsv(&self) -> $crate::error::Result<Hsv<T>> {
                self.colour().to_hsv()
            }

            fn to_hsv_alpha(&self) -> $crate::error::Result<HsvAlpha<T>> {
                let hsv = self.colour().to_hsv()?;
                HsvAlpha::new(hsv.hue(), hsv.saturation(), hsv.value(), self.alpha())
            }

            fn to_lab(&self) -> $crate::error::Result<Lab<T>> {
                self.colour().to_lab()
            }

            fn to_lab_alpha(&self) -> $crate::error::Result<LabAlpha<T>> {
                let lab = self.colour().to_lab()?;
                LabAlpha::new(lab.lightness(), lab.a_star(), lab.b_star(), self.alpha())
            }

            fn to_rgb(&self) -> $crate::error::Result<Rgb<T>> {
                self.colour().to_rgb()
            }

            fn to_rgb_alpha(&self) -> $crate::error::Result<RgbAlpha<T>> {
                let rgb = self.colour().to_rgb()?;
                RgbAlpha::new(rgb.red(), rgb.green(), rgb.blue(), self.alpha())
            }

            fn to_srgb(&self) -> $crate::error::Result<Srgb<T>> {
                self.colour().to_srgb()
            }

            fn to_srgb_alpha(&self) -> $crate::error::Result<SrgbAlpha<T>> {
                let srgb = self.colour().to_srgb()?;
                SrgbAlpha::new(srgb.red(), srgb.green(), srgb.blue(), self.alpha())
            }

            fn to_xyz(&self) -> $crate::error::Result<Xyz<T>> {
                self.colour().to_xyz()
            }

            fn to_xyz_alpha(&self) -> $crate::error::Result<XyzAlpha<T>> {
                let xyz = self.colour().to_xyz()?;
                XyzAlpha::new(xyz.x(), xyz.y(), xyz.z(), self.alpha())
            }
        }
    };
}

/// Macro to implement `Display` for transparent colour types.
#[macro_export]
macro_rules! impl_transparent_display {
    ($type:ty) => {
        impl<T: Float + Send + Sync> Display for $type {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
                write!(fmt, "{}", self.colour())
            }
        }
    };
}
