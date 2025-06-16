//! Error handling for the `Chromatic` library.

use num_traits::{Float, ToPrimitive};
use std::{any::type_name, fmt::Display};
use thiserror::Error;

mod colour_map;
mod colour_parsing;
mod conversion;
mod interpolation;
mod numeric;

pub use colour_map::ColourMapError;
pub use colour_parsing::ColourParsingError;
pub use conversion::ConversionError;
pub use interpolation::InterpolationError;
pub use numeric::NumericError;

/// Main error type for the Chromatic library.
///
/// This enum represents all possible errors that can occur during colour operations,
/// including parsing, conversion, interpolation, and colour map operations.
#[derive(Error, Debug)]
pub enum ChromaticError {
    /// Invalid colour value or component.
    ///
    /// This error occurs when creating or operating on colours with invalid parameters,
    /// such as components outside valid ranges, invalid hex strings, or malformed colour data.
    ///
    /// # Examples
    /// - RGB values outside [0, 1] range
    /// - Invalid hex colour strings like "#GGG" or "#12345"
    /// - Alpha values outside [0, 1] range
    /// - Lab lightness values outside [0, 100] range
    #[error("Invalid colour: {0}")]
    InvalidColour(String),

    /// Error parsing colour from string representation.
    ///
    /// This error is raised when converting string representations (hex, named colours, etc.)
    /// to colour objects, typically due to invalid format or out-of-range values.
    ///
    /// # Examples
    /// - Malformed hex strings: "#ZZZZZZ", "#12345G"
    /// - Missing '#' prefix in hex colours
    /// - Invalid colour component values in string format
    #[error("Colour parsing error: {0}")]
    ColourParsing(#[from] ColourParsingError),

    /// Colour space conversion failure.
    ///
    /// This error occurs when converting between colour spaces fails due to
    /// mathematical constraints, invalid intermediate values, or unsupported conversions.
    ///
    /// # Examples
    /// - Converting colours that result in values outside the target colour space
    /// - Mathematical operations that produce NaN or infinite values
    /// - Loss of precision during conversion chains
    #[error("Colour conversion error: {0}")]
    ColourConversion(#[from] ConversionError),

    /// Invalid interpolation parameters or operation.
    ///
    /// This error is raised when interpolation operations fail due to invalid parameters,
    /// mismatched colour types, or mathematical constraints.
    ///
    /// # Examples
    /// - Interpolation factor t outside [0, 1] range
    /// - Empty colour lists for mixing operations
    /// - Mismatched colour array and weight array lengths
    /// - Negative weights in colour mixing
    #[error("Interpolation error: {0}")]
    Interpolation(#[from] InterpolationError),

    /// `ColourMap` construction or sampling failure.
    ///
    /// This error occurs when creating colour maps with invalid parameters or
    /// when sampling operations fail due to invalid positions or empty maps.
    ///
    /// # Examples
    /// - Creating colour maps with empty colour arrays
    /// - Position arrays not matching colour array length
    /// - Sampling positions outside [0, 1] range
    /// - Non-ascending position values in colour maps
    #[error("Colour map error: {0}")]
    ColourMap(#[from] ColourMapError),

    /// Mathematical computation error.
    ///
    /// This error is raised when mathematical operations fail, such as type conversions
    /// between numeric types, overflow conditions, or when required mathematical
    /// bounds are unavailable.
    ///
    /// # Examples
    /// - Failed conversion between float types (f32 ↔ f64)
    /// - Numeric overflow or underflow in colour calculations
    /// - Invalid mathematical operations (division by zero, etc.)
    /// - Missing numeric bounds for custom float types
    #[error("Math error: {0}")]
    Math(#[from] NumericError),

    /// Terminal or display formatting error.
    ///
    /// This error occurs when attempting to display colours in terminal environments
    /// or when formatting operations fail due to unsupported features or invalid state.
    ///
    /// # Examples
    /// - Terminal doesn't support RGB colour codes
    /// - Invalid terminal size detection
    /// - Formatting operations on invalid colour states
    #[error("Display error: {0}")]
    Display(String),
}

/// Result type alias for the Chromatic library.
pub type Result<T> = std::result::Result<T, ChromaticError>;

impl From<ChromaticError> for std::fmt::Error {
    fn from(_: ChromaticError) -> Self {
        Self
    }
}

/// Safe conversion for common constants with detailed error context.
pub fn safe_constant<S: Copy + Send + Sync + Display + ToPrimitive, T: Send + Sync + Float>(value: S) -> Result<T> {
    T::from(value).ok_or_else(|| {
        NumericError::TypeConversionFailed {
            from: type_name::<S>().to_string(),
            to: type_name::<T>().to_string(),
            reason: format!("Failed to convert constant: {}", value),
        }
        .into()
    })
}

/// Validate a component is within a specified range with consistent error messages.
pub fn validate_component_range<T: Float + Send + Sync>(value: T, name: &str, min: T, max: T) -> Result<()> {
    if value < min || value > max {
        return Err(ChromaticError::InvalidColour(format!(
            "{} component ({}) must be between {} and {}",
            name,
            value.to_f64().unwrap_or(f64::NAN),
            min.to_f64().unwrap_or(f64::NAN),
            max.to_f64().unwrap_or(f64::NAN)
        )));
    }
    Ok(())
}

/// Validate a component is within [0, 1] range - the most common validation.
pub fn validate_unit_component<T: Float + Send + Sync>(value: T, name: &str) -> Result<()> {
    validate_component_range(value, name, T::zero(), T::one())
}

/// Validate an interpolation factor is within [0, 1] range.
pub fn validate_interpolation_factor<T: Float + Send + Sync>(t: T) -> Result<()> {
    if t < T::zero() || t > T::one() {
        return Err(InterpolationError::InvalidInterpolationFactor {
            factor: t.to_f64().unwrap_or(f64::NAN),
        }
        .into());
    }
    Ok(())
}

/// Convert a component to u8 with proper error handling.
pub fn component_to_u8<T: Float + Send + Sync>(value: T, name: &str, scale_factor: T) -> Result<u8> {
    let scaled = (value * scale_factor).round();
    scaled.to_u8().ok_or_else(|| {
        NumericError::TypeConversionFailed {
            from: type_name::<T>().to_string(),
            to: "u8".to_string(),
            reason: format!(
                "{} value {} is outside u8 range [0, 255]",
                name,
                scaled.to_f64().unwrap_or(f64::NAN)
            ),
        }
        .into()
    })
}

/// Convert a u8 component to the target float type with proper scaling.
pub fn u8_to_component<T: Float + Send + Sync>(value: u8, scale_factor: T) -> Result<T> {
    let converted = safe_constant::<u8, T>(value)?;
    Ok(converted / scale_factor)
}

/// Parse a hex string component (1 or 2 characters) to u8.
pub fn parse_hex_component(hex: &str, component_name: &str) -> Result<u8> {
    u8::from_str_radix(hex, 16).map_err(|source| {
        ColourParsingError::HexParseError {
            component: component_name.to_string(),
            source,
        }
        .into()
    })
}

/// Normalize hue to [0, 360) range with overflow protection.
pub fn normalize_hue<T: Float + Send + Sync>(mut hue: T) -> Result<T> {
    let f360 = safe_constant(360.0)?;

    // Handle potential infinite loops by limiting iterations
    let mut iterations = 0;
    const MAX_ITERATIONS: usize = 1000;

    while hue >= f360 && iterations < MAX_ITERATIONS {
        hue = hue - f360;
        iterations += 1;
    }

    iterations = 0;
    while hue < T::zero() && iterations < MAX_ITERATIONS {
        hue = hue + f360;
        iterations += 1;
    }

    if iterations >= MAX_ITERATIONS {
        return Err(NumericError::InvalidMathOperation(format!(
            "Hue normalization failed: value too large ({})",
            hue.to_f64().unwrap_or(f64::NAN)
        ))
        .into());
    }

    Ok(hue)
}

/// Helper for terminal color formatting that handles conversion errors.
pub fn format_terminal_color<T: Float + Send + Sync>(red: T, green: T, blue: T, symbol: char) -> Result<String> {
    let scale = safe_constant::<i32, T>(255)?;

    let r = component_to_u8(red, "red", scale)?;
    let g = component_to_u8(green, "green", scale)?;
    let b = component_to_u8(blue, "blue", scale)?;

    Ok(format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, symbol))
}
