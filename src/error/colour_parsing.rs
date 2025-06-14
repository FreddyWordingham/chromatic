//! Colour parsing specific errors.

use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ColourParsingError {
    #[error("Invalid hex format: {0}")]
    InvalidHexFormat(String),

    #[error("Hex parsing failed: {0}")]
    HexParseError(#[from] ParseIntError),

    #[error("Component value out of range: {value} (expected {min}-{max})")]
    ComponentOutOfRange { value: f64, min: f64, max: f64 },

    #[error("Invalid colour format: {0}")]
    InvalidFormat(String),

    #[error("Missing required component: {0}")]
    MissingComponent(String),
}
