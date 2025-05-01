//! ## `Colours` Module
//!
//! This module provides different colour types.

mod grey;
mod grey_alpha;

pub use grey::{Grey, ParseGreyError};
pub use grey_alpha::{GreyAlpha, ParseGreyAlphaError};
