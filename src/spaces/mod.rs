//! This module provides implementations of various colour spaces.

mod grey;
mod grey_alpha;
mod hsl;
mod hsl_alpha;
mod hsv;
mod hsv_alpha;
mod lab;
mod lab_alpha;
mod rgb;
mod rgb_alpha;
mod srgb;
mod srgb_alpha;
mod transparent;
mod xyz;
mod xyz_alpha;

pub use grey::Grey;
pub use grey_alpha::GreyAlpha;
pub use hsl::Hsl;
pub use hsl_alpha::HslAlpha;
pub use hsv::Hsv;
pub use hsv_alpha::HsvAlpha;
pub use lab::Lab;
pub use lab_alpha::LabAlpha;
pub use rgb::Rgb;
pub use rgb_alpha::RgbAlpha;
pub use srgb::Srgb;
pub use srgb_alpha::SrgbAlpha;
pub use xyz::Xyz;
pub use xyz_alpha::XyzAlpha;
