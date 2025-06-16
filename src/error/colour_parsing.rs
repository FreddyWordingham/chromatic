//! Colour parsing specific errors.

use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ColourParsingError {
    #[error("Invalid hex format: expected format like #RGB, #RRGGBB, #RGBA, or #RRGGBBAA, got '{0}'")]
    InvalidHexFormat(String),

    #[error("Missing '#' prefix in hex colour string: '{0}'")]
    MissingHexPrefix(String),

    #[error("Hex parsing failed for component '{component}': {source}")]
    HexParseError {
        component: String,
        #[source]
        source: ParseIntError,
    },

    #[error("Component value {value} out of range for {component} (expected {min}-{max})")]
    ComponentOutOfRange {
        component: String,
        value: f64,
        min: f64,
        max: f64,
    },

    #[error("Invalid colour format: {0}")]
    InvalidFormat(String),

    #[error("Missing required component: {0}")]
    MissingComponent(String),

    #[error("Hex string length {actual} is invalid (expected 1, 2, 3, 4, 6, or 8 characters after #)")]
    InvalidHexLength { actual: usize },
}
