//! Colour conversion specific errors.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("Conversion from {from} to {to} failed: {reason}")]
    ConversionFailed { from: String, to: String, reason: String },

    #[error("Intermediate conversion produced invalid values")]
    InvalidIntermediateValue,

    #[error("Mathematical operation failed during conversion: {0}")]
    MathError(String),

    #[error("Precision loss during conversion from {from} to {to}")]
    PrecisionLoss { from: String, to: String },

    #[error("Colour space {0} not supported for this operation")]
    UnsupportedColourSpace(String),
}
