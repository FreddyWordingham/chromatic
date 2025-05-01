//! ## `Colours` Module
//!
//! This module provides different colour types.

mod grey;
mod grey_alpha;
mod rgb;
mod rgba;

pub use grey::{Grey, ParseGreyError};
pub use grey_alpha::{GreyAlpha, ParseGreyAlphaError};
pub use rgb::{ParseRgbError, Rgb};
pub use rgba::{ParseRgbaError, Rgba};
