//! ## `lab_utils` Module
//!
//! This module provides utility functions for converting between different colour spaces, specifically RGB, Lab and XYZ.

use num_traits::Float;

/// Convert XYZ to Lab colour space.
///
/// Lab uses standard CIELAB ranges:
/// - L* is in [0, 100]
/// - a* is in [-128, 127]
/// - b* is in [-128, 127]
#[expect(
    clippy::many_single_char_names,
    reason = "The variables `xyz` and `lab` are idiomatic for colour spaces."
)]
#[expect(
    clippy::min_ident_chars,
    reason = "The variables `xyz` and `lab` are idiomatic for colour spaces."
)]
#[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
#[inline]
pub fn xyz_to_lab<T: Float>(xyz: &[T; 3]) -> [T; 3] {
    // Reference white (D65)
    let xn = T::from(0.95047).unwrap();
    let yn = T::from(1.0).unwrap();
    let zn = T::from(1.08883).unwrap();

    // Normalized XYZ
    let x = lab_f(xyz[0] / xn);
    let y = lab_f(xyz[1] / yn);
    let z = lab_f(xyz[2] / zn);

    // Calculate Lab components
    let l = T::from(116.0).unwrap() * y - T::from(16.0).unwrap();
    let a = T::from(500.0).unwrap() * (x - y);
    let b = T::from(200.0).unwrap() * (y - z);

    // Standard Lab ranges: L* [0,100], a* [-128,127], b* [-128,127]
    // Ensure a* and b* values stay within standard bounds
    let a_clamped = a.max(T::from(-128).unwrap()).min(T::from(127).unwrap());
    let b_clamped = b.max(T::from(-128).unwrap()).min(T::from(127).unwrap());

    [l, a_clamped, b_clamped]
}

/// Helper function for Lab conversion.
#[expect(
    clippy::min_ident_chars,
    reason = "The variable `t` for an interpolation factor is idiomatic."
)]
#[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
#[inline]
pub fn lab_f<T: Float>(t: T) -> T {
    let delta = T::from(6.0 / 29.0).unwrap();
    let delta_cubed = delta * delta * delta;

    if t > delta_cubed {
        t.powf(T::from(1.0 / 3.0).unwrap())
    } else {
        t / (T::from(3.0).unwrap() * delta * delta) + T::from(4.0 / 29.0).unwrap()
    }
}

/// Convert Lab to XYZ colour space.
///
/// Expects Lab in standard CIELAB ranges:
/// - L* in [0, 100]
/// - a* in [-128, 127]
/// - b* in [-128, 127]
#[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
#[inline]
pub fn lab_to_xyz<T: Float>(lab: &[T; 3]) -> [T; 3] {
    // Reference white (D65)
    let xn = T::from(0.95047).unwrap();
    let yn = T::from(1.0).unwrap();
    let zn = T::from(1.08883).unwrap();

    // Calculate intermediate values
    let fy = (lab[0] + T::from(16.0).unwrap()) / T::from(116.0).unwrap();
    let fx = fy + (lab[1] / T::from(500.0).unwrap());
    let fz = fy - (lab[2] / T::from(200.0).unwrap());

    // Convert to XYZ
    let x = lab_f_inv(fx) * xn;
    let y = lab_f_inv(fy) * yn;
    let z = lab_f_inv(fz) * zn;

    [x, y, z]
}

/// Inverse function for Lab conversion.
#[expect(
    clippy::min_ident_chars,
    reason = "The variable `t` for an interpolation factor is idiomatic."
)]
#[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
#[inline]
pub fn lab_f_inv<T: Float>(t: T) -> T {
    let delta = T::from(6.0 / 29.0).unwrap();

    if t > delta {
        t * t * t
    } else {
        T::from(3.0).unwrap() * delta * delta * (t - T::from(4.0 / 29.0).unwrap())
    }
}

/// Convert RGB array to XYZ colour space
#[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
#[inline]
pub fn rgb_to_xyz_components<T: Float>(rgb: &[T; 3]) -> [T; 3] {
    // Convert from gamma-corrected RGB to linear RGB
    let red = gamma_to_linear(rgb[0]);
    let green = gamma_to_linear(rgb[1]);
    let blue = gamma_to_linear(rgb[2]);

    // Convert to XYZ using sRGB standard matrix
    let x = red * T::from(0.4124564).unwrap() + green * T::from(0.3575761).unwrap() + blue * T::from(0.1804375).unwrap();
    let y = red * T::from(0.2126729).unwrap() + green * T::from(0.7151522).unwrap() + blue * T::from(0.0721750).unwrap();
    let z = red * T::from(0.0193339).unwrap() + green * T::from(0.1191920).unwrap() + blue * T::from(0.9503041).unwrap();

    [x, y, z]
}

/// Convert XYZ to RGB colour space components.
#[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
#[inline]
pub fn xyz_to_rgb_components<T: Float>(xyz: &[T; 3]) -> [T; 3] {
    // Convert XYZ to linear RGB using sRGB standard inverse matrix
    let red =
        xyz[0] * T::from(3.2404542).unwrap() - xyz[1] * T::from(1.5371385).unwrap() - xyz[2] * T::from(0.4985314).unwrap();
    let green =
        -xyz[0] * T::from(0.9692660).unwrap() + xyz[1] * T::from(1.8760108).unwrap() + xyz[2] * T::from(0.0415560).unwrap();
    let blue =
        xyz[0] * T::from(0.0556434).unwrap() - xyz[1] * T::from(0.2040259).unwrap() + xyz[2] * T::from(1.0572252).unwrap();

    // Convert from linear RGB to gamma-corrected RGB
    let red_gamma = linear_to_gamma(red);
    let green_gamma = linear_to_gamma(green);
    let blue_gamma = linear_to_gamma(blue);

    // Clamp values to [0, 1] range
    let red_clamped = red_gamma.max(T::zero()).min(T::one());
    let green_clamped = green_gamma.max(T::zero()).min(T::one());
    let blue_clamped = blue_gamma.max(T::zero()).min(T::one());

    [red_clamped, green_clamped, blue_clamped]
}

/// Convert gamma-corrected RGB to linear RGB.
#[expect(clippy::min_ident_chars, reason = "There is only a single variable `c`.")]
#[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
#[inline]
pub fn gamma_to_linear<T: Float>(c: T) -> T {
    if c <= T::from(0.04045).unwrap() {
        c / T::from(12.92).unwrap()
    } else {
        ((c + T::from(0.055).unwrap()) / T::from(1.055).unwrap()).powf(T::from(2.4).unwrap())
    }
}

/// Convert linear RGB to gamma-corrected RGB.
#[expect(clippy::min_ident_chars, reason = "There is only a single variable `c`.")]
#[expect(clippy::unwrap_used, reason = "Unwrap will not fail here.")]
#[inline]
pub fn linear_to_gamma<T: Float>(c: T) -> T {
    if c <= T::from(0.0031308).unwrap() {
        c * T::from(12.92).unwrap()
    } else {
        c.powf(T::from(1.0 / 2.4).unwrap()) * T::from(1.055).unwrap() - T::from(0.055).unwrap()
    }
}
