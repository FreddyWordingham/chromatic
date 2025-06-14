//! Error handling for the `Chromatic` library.

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

use thiserror::Error;

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
    ColourParsing(String),

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
    ColourConversion(String),

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
    Interpolation(String),

    /// ColourMap construction or sampling failure.
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
    ColourMap(String),

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
    Math(String),

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

// Implement From traits for automatic error conversion
impl From<ColourParsingError> for ChromaticError {
    fn from(err: ColourParsingError) -> Self {
        Self::ColourParsing(err.to_string())
    }
}

impl From<ConversionError> for ChromaticError {
    fn from(err: ConversionError) -> Self {
        Self::ColourConversion(err.to_string())
    }
}

impl From<InterpolationError> for ChromaticError {
    fn from(err: InterpolationError) -> Self {
        Self::Interpolation(err.to_string())
    }
}

impl From<ColourMapError> for ChromaticError {
    fn from(err: ColourMapError) -> Self {
        Self::ColourMap(err.to_string())
    }
}

impl From<NumericError> for ChromaticError {
    fn from(err: NumericError) -> Self {
        Self::Math(err.to_string())
    }
}
