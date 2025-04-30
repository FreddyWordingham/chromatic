//! Error types for `ColourMap` operations.

use thiserror::Error;

/// Error type for building and using `ColourMap`s.
#[derive(Debug, Clone, Error)]
#[non_exhaustive]
pub enum ColourMapError {
    /// Error when trying to insert a colour at an invalid position.
    #[error("Positions must be in range [0, 1]")]
    InvalidPositions,

    /// Error when trying to insert a colour at an existing position.
    #[error("Cannot insert a colour with an existing position")]
    DuplicatePositions,
}
