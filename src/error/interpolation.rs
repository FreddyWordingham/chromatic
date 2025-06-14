//! Interpolation specific errors.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum InterpolationError {
    #[error("Interpolation factor {factor} is outside valid range [0, 1]")]
    InvalidInterpolationFactor { factor: f64 },

    #[error("Cannot interpolate between different colour types: {type1} and {type2}")]
    IncompatibleColourTypes { type1: String, type2: String },

    #[error("Empty colour list provided for mixing operation")]
    EmptyColourList,

    #[error("Colour and weight arrays have different lengths: {colours} colours, {weights} weights")]
    MismatchedArrayLengths { colours: usize, weights: usize },

    #[error("Negative weight {weight} at index {index}")]
    NegativeWeight { weight: f64, index: usize },

    #[error("Weight sum is zero or invalid")]
    InvalidWeightSum,
}
