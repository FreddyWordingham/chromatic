//! # `Chromatic`
//!
//! `Chromatic` is a simple library for building and sampling colour maps.

#![deny(clippy::all)]
#![deny(clippy::cargo)]
#![deny(clippy::complexity)]
#![deny(clippy::correctness)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::restriction)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]
#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "Alphabetical ordering is not always the most readable."
)]
#![allow(clippy::arithmetic_side_effects, reason = "Too restrictive for this crate.")]
#![allow(clippy::blanket_clippy_restriction_lints, reason = "Prefer more lints.")]
#![allow(clippy::default_numeric_fallback, reason = "Numeric type fallback should not be required.")]
#![allow(clippy::float_arithmetic, reason = "Too restrictive for this crate.")]
#![allow(clippy::implicit_return, reason = "Implicit returns are idiomatic in Rust.")]
#![allow(clippy::indexing_slicing, reason = "Too restrictive for this crate.")]
#![allow(
    clippy::min_ident_chars,
    reason = "Whilst short variable names are not always ideal they are often clear in context."
)]
#![allow(
    clippy::missing_trait_methods,
    reason = "Traits should be able to provide default method implementations."
)]
#![allow(clippy::mod_module_files, reason = "Prefer to use mod.rs files for consistency.")]
#![allow(clippy::pub_use, reason = "It is intended to expose some types at the crate level.")]
#![allow(
    clippy::pub_with_shorthand,
    reason = "Rustfmt automatically shortens pub(in crate) to pub(crate)."
)]
#![allow(clippy::question_mark_used, reason = "The question mark operator is idiomatic in Rust.")]
#![allow(
    clippy::separated_literal_suffix,
    reason = "Must chose between separated and unseparated literal suffixes."
)]
#![allow(clippy::unwrap_in_result, reason = "In some cases unwrap can be guaranteed to succeed.")]
#![allow(clippy::unwrap_used, reason = "In some cases unwrap can be guaranteed to succeed.")]
#![allow(clippy::std_instead_of_core, reason = "Prefer std for consistency.")]
#![allow(
    clippy::unreadable_literal,
    reason = "Prefer no underscores in numeric literals for consistency."
)]
#![allow(clippy::else_if_without_else, reason = "Eliding final else is idiomatic in Rust.")]

mod colour;
mod colour_map;
mod config;
mod convert;
mod parse_colour_error;
mod spaces;

pub use colour::Colour;
pub use colour_map::ColourMap;
pub use convert::Convert;
pub use parse_colour_error::ParseColourError;
pub use spaces::{Grey, GreyAlpha, Hsl, HslAlpha, Hsv, HsvAlpha, Lab, LabAlpha, Rgb, RgbAlpha, Srgb, SrgbAlpha, Xyz, XyzAlpha};
