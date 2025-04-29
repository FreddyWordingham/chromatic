//! Error types for color parsing operations.

use core::num::{ParseIntError, TryFromIntError};
use thiserror::Error;

/// Error type for parsing colour strings.
#[derive(Debug, Clone, Error)]
#[expect(
    variant_size_differences,
    reason = "Different sized variants is acceptable for this error type."
)]
#[non_exhaustive]
pub enum ColourParseError {
    /// Error when hex string has incorrect length.
    #[error("Invalid hex length: {0}, expected 2, 4, 6 or 8 digits")]
    InvalidLength(usize),

    /// Error when hex string contains invalid characters.
    #[error("Invalid hex (parse error): {0}")]
    InvalidHex(#[from] ParseIntError),

    /// Checked numeric conversion failed (should never happen)
    #[error("Component overflow: {0}")]
    ConversionFailed(#[from] TryFromIntError),

    /// Error when gradient creation fails.
    #[error("Failed to create gradient")]
    GradientCreationFailed,

    /// Error when positions are invalid for gradient creation.
    #[error("Invalid positions: must be in range [0,1] and strictly ascending")]
    InvalidPositions,

    /// Error when trying to build a colour map with different colour types.
    #[error("Cannot build colour map with different colour types")]
    MixedColourTypes,
}
