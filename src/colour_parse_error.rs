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
    ///
    #[error("Invalid hex length: {0}, expected 2, 4, 6 or 8 digits")]
    InvalidLength(usize),

    ///
    #[error("Invalid hex (parse error): {0}")]
    InvalidHex(#[from] ParseIntError),

    /// Checked numeric conversion failed (should never happen)
    #[error("Component overflow: {0}")]
    ConversionFailed(#[from] TryFromIntError),

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
