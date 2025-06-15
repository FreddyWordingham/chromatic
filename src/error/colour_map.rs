//! `ColourMap` specific errors.

use thiserror::Error;

#[derive(Debug, Clone, Copy, Error)]
pub enum ColourMapError {
    #[error("Empty colour map: at least one colour required")]
    EmptyColourMap,

    #[error("Colour and position arrays have different lengths: {colours} colours, {positions} positions")]
    MismatchedArrayLengths { colours: usize, positions: usize },

    #[error("Position {position} at index {index} is outside valid range [0, 1]")]
    PositionOutOfRange { position: f64, index: usize },

    #[error("Positions are not in ascending order: position {pos1} at index {idx1} >= position {pos2} at index {idx2}")]
    NonAscendingPositions { pos1: f64, idx1: usize, pos2: f64, idx2: usize },

    #[error("Sampling position {position} is outside valid range [0, 1]")]
    InvalidSamplingPosition { position: f64 },

    #[error("ColourMap is empty and cannot be sampled")]
    EmptyMapSampling,

    #[error("Terminal width detection failed")]
    TerminalWidthDetectionFailed,
}
