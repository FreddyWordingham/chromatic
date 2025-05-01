//! ## `Colours` Module
//!
//! This module provides different colour types.

mod grey;
mod grey_alpha;
mod lab_rgb;
mod lab_rgba;
mod lab_utils;
mod parse_colour_error;
mod rgb;
mod rgba;

pub use grey::Grey;
pub use grey_alpha::GreyAlpha;
pub use lab_rgb::LabRgb;
pub use lab_rgba::LabRgba;
pub use parse_colour_error::ParseColourError;
pub use rgb::Rgb;
pub use rgba::Rgba;
