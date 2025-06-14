//! Numeric computation specific errors.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum NumericError {
    #[error("Type conversion failed from {from} to {to}: {reason}")]
    TypeConversionFailed { from: String, to: String, reason: String },

    #[error("Numeric overflow in operation: {operation}")]
    NumericOverflow { operation: String },

    #[error("Numeric underflow in operation: {operation}")]
    NumericUnderflow { operation: String },

    #[error("Division by zero in operation: {operation}")]
    DivisionByZero { operation: String },

    #[error("Invalid mathematical operation: {0}")]
    InvalidMathOperation(String),

    #[error("Floating point operation produced NaN")]
    NaNResult,

    #[error("Floating point operation produced infinity")]
    InfiniteResult,

    #[error("Required numeric bounds not available for type {0}")]
    MissingNumericBounds(String),
}
