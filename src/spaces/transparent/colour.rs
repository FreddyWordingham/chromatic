//! Implements the `Colour` trait for transparent colour types.

/// Macro to implement the `Colour` trait for transparent colour types.
#[macro_export]
macro_rules! impl_transparent_colour {
    ($type:ty, $base:ty, $base_components:expr) => {
        impl<T: Float + Send + Sync> Colour<T, { $base_components + 1 }> for $type {
            #[inline]
            fn from_hex(hex: &str) -> Result<Self, ParseColourError<ParseIntError>> {
                let components = hex.trim().strip_prefix('#').ok_or(ParseColourError::InvalidFormat)?;
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
                        let alpha_val = u8::from_str_radix(&alpha_char.to_string(), 16).map_err(ParseColourError::ParseHex)?;

                        // Expand from single hex digit (e.g., F -> FF)
                        let alpha = T::from(alpha_val * 17).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();

                        Ok(Self::new_colour_with_alpha(colour, alpha))
                    }
                    // Long form with alpha (e.g., #RRGGBB + AA for RGBA)
                    val if val == $base_components * 2 + 2 => {
                        // Parse the base colour part
                        let colour_part: String = chars[0..($base_components * 2)].iter().collect::<String>();
                        let colour_part = format!("#{}", colour_part);
                        let colour = <$base>::from_hex(&colour_part)?;

                        // Parse alpha (two hex digits)
                        let alpha_hex: String = chars[($base_components * 2)..].iter().collect();
                        let alpha_val = u8::from_str_radix(&alpha_hex, 16).map_err(ParseColourError::ParseHex)?;

                        let alpha = T::from(alpha_val).ok_or(ParseColourError::OutOfRange)? / T::from(255).unwrap();

                        Ok(Self::new_colour_with_alpha(colour, alpha))
                    }
                    _ => Err(ParseColourError::InvalidFormat),
                }
            }

            #[inline]
            fn to_hex(&self) -> String {
                let max = T::from(255_u8).unwrap();
                let colour_hex = self.colour().to_hex();
                let alpha = (self.alpha() * max).round().to_u8().unwrap();

                // Remove # from colour hex and add alpha
                let colour_part: String = colour_hex.chars().skip(1).collect();
                format!("#{}{:02X}", colour_part, alpha)
            }

            #[inline]
            fn from_bytes(bytes: [u8; $base_components + 1]) -> Self {
                let max = T::from(255_u8).unwrap();

                // Extract base colour bytes
                let mut base_bytes = [0_u8; $base_components];
                for i in 0..$base_components {
                    base_bytes[i] = bytes[i];
                }

                let colour = <$base>::from_bytes(base_bytes);
                let alpha = T::from(bytes[$base_components]).unwrap() / max;

                Self::new_colour_with_alpha(colour, alpha)
            }

            #[inline]
            fn to_bytes(self) -> [u8; $base_components + 1] {
                let max = T::from(255_u8).unwrap();
                let base_bytes = self.colour().to_bytes();
                let alpha = (self.alpha() * max).round().to_u8().unwrap();

                // Create result array
                let mut result = [0_u8; $base_components + 1];

                // Copy base colour bytes
                for i in 0..$base_components {
                    result[i] = base_bytes[i];
                }

                // Add alpha byte
                result[$base_components] = alpha;

                result
            }

            #[inline]
            fn lerp(lhs: &Self, rhs: &Self, t: T) -> Self {
                debug_assert!(
                    t >= T::zero() && t <= T::one(),
                    "Interpolation factor must be in range [0, 1]."
                );

                // Interpolate base colour
                let colour = <$base>::lerp(lhs.colour(), rhs.colour(), t);

                // Interpolate alpha
                let alpha = lhs.alpha() * (T::one() - t) + rhs.alpha() * t;

                Self::new_colour_with_alpha(colour, alpha)
            }
        }
    };
}
