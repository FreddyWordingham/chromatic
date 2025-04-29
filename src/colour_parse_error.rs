use thiserror::Error;

/// Error type for parsing colour strings.
#[derive(Debug, Clone, Copy, Error)]
#[non_exhaustive]
pub enum ColourParseError {
    /// Invalid length of hex string, must be 2, 4, 6, or 8.
    #[error("Invalid hex length: {0}")]
    InvalidLength(usize),

    /// Hex strings must only contain digits and letters a-f (case insensitive).
    #[error("Invalid hex characters")]
    InvalidHex,

    ///
    #[error("Failed to convert number")]
    ConversionFailed,

    ///
    #[error("Failed to create gradient")]
    GradientCreationFailed,

    ///
    #[error("Invalid positions: must be in range [0,1] and strictly ascending")]
    InvalidPositions,

    ///
    #[error("Cannot build colour map with different colour types")]
    MixedColourTypes,
}
