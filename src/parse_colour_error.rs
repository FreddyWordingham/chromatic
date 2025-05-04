//! Error types for parsing colours.

use std::num::ParseIntError;

/// Error parsing `Colour`s from string.
#[derive(Debug)]
#[non_exhaustive]
pub enum ParseColourError<E> {
    /// Error parsing float.
    ParseFloat(E),
    /// Error parsing hex string.
    ParseHex(ParseIntError),
    /// Value out of range.
    OutOfRange,
    /// Invalid format.
    InvalidFormat,
}
