//! ## `Colours` Module
//!
//! This module provides different colour types.

mod grey;
mod grey_alpha;
mod lab_rgb;
mod lab_rgba;
mod lab_utils;
mod rgb;
mod rgba;

pub use grey::{Grey, ParseGreyError};
pub use grey_alpha::{GreyAlpha, ParseGreyAlphaError};
pub use lab_rgb::LabRgb;
pub use lab_rgba::LabRgba;
pub use rgb::{ParseRgbError, Rgb};
pub use rgba::{ParseRgbaError, Rgba};
