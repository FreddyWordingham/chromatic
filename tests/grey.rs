use assert_approx_eq::assert_approx_eq;
use chromatic::{Colour, Grey};

#[test]
#[should_panic(expected = "Interpolation factor must be in range [0, 1]")]
fn test_grey_lerp_out_of_range_below() {
    let black = Grey::new(0.0);
    let white = Grey::new(1.0);

    let _ = Grey::lerp(&black, &white, -0.1);
}

#[test]
#[should_panic(expected = "Interpolation factor must be in range [0, 1]")]
fn test_grey_lerp_out_of_range_above() {
    let black = Grey::new(0.0);
    let white = Grey::new(1.0);

    let _ = Grey::lerp(&black, &white, 1.1);
}

#[test]
fn test_grey_mix() {
    let black = Grey::new(0.0);
    let mid_grey = Grey::new(0.5);
    let white = Grey::new(1.0);

    // Mix with equal weights
    let colours = vec![black, mid_grey, white];
    let weights = vec![1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0];

    let mixed = Grey::mix(&colours, &weights);

    assert_approx_eq!(mixed.grey(), 0.5_f64);

    // Mix with different weights
    let weights = vec![0.25, 0.5, 0.25];

    let mixed = Grey::mix(&colours, &weights);

    assert_approx_eq!(mixed.grey(), 0.5);
}

#[test]
fn test_grey_mix_single_color() {
    let grey = Grey::new(0.4);

    let colours = vec![grey];
    let weights = vec![1.0];

    let mixed = Grey::mix(&colours, &weights);

    assert_eq!(mixed, grey);
}

#[test]
#[should_panic(expected = "Cannot mix an empty list of colours")]
fn test_grey_mix_empty() {
    let colours: Vec<Grey<f32>> = vec![];
    let weights: Vec<f32> = vec![];

    let _ = Grey::mix(&colours, &weights);
}

#[test]
#[should_panic(expected = "Colours and weights must have the same length")]
fn test_grey_mix_mismatched_lengths() {
    let black = Grey::new(0.0);
    let white = Grey::new(1.0);

    let colours = vec![black, white];
    let weights = vec![1.0];

    let _ = Grey::mix(&colours, &weights);
}

#[test]
#[should_panic(expected = "Weights must be non-negative")]
fn test_grey_mix_negative_weights() {
    let black = Grey::new(0.0);
    let white = Grey::new(1.0);

    let colours = vec![black, white];
    let weights = vec![1.0, -0.5];

    let _ = Grey::mix(&colours, &weights);
}

#[test]
fn test_grey_equality() {
    let grey1 = Grey::new(0.5);
    let grey2 = Grey::new(0.5);
    let grey3 = Grey::new(0.6);

    assert_eq!(grey1, grey2);
    assert_ne!(grey1, grey3);

    // Test with small differences within tolerance
    let tolerance = Grey::<f32>::tolerance();
    let grey4 = Grey::new(0.5 + tolerance * 0.5);

    assert_eq!(grey1, grey4);

    // Test with small differences outside tolerance
    let grey5 = Grey::new(0.5 + tolerance * 1.5);

    assert_ne!(grey1, grey5);
}

#[test]
fn test_grey_with_f64() {
    let grey = Grey::<f64>::new(0.5);

    assert_eq!(grey.grey(), 0.5);
}

#[test]
fn test_grey_conversions() {
    let grey = Grey::new(0.5);

    // To GreyAlpha
    let grey_alpha = grey.to_grey_alpha(0.8);
    assert_approx_eq!(grey_alpha.grey(), 0.5_f64);
    assert_approx_eq!(grey_alpha.alpha(), 0.8);

    // To Rgb
    let rgb = grey.to_rgb();
    assert_approx_eq!(rgb.red(), 0.5);
    assert_approx_eq!(rgb.green(), 0.5);
    assert_approx_eq!(rgb.blue(), 0.5);

    // To Rgba
    let rgba = grey.to_rgba(0.7);
    assert_approx_eq!(rgba.red(), 0.5);
    assert_approx_eq!(rgba.green(), 0.5);
    assert_approx_eq!(rgba.blue(), 0.5);
    assert_approx_eq!(rgba.alpha(), 0.7);

    // To LabRgb
    let lab_rgb = grey.to_lab_rgb();
    assert_approx_eq!(lab_rgb.red(), 0.5);
    assert_approx_eq!(lab_rgb.green(), 0.5);
    assert_approx_eq!(lab_rgb.blue(), 0.5);

    // To LabRgba
    let lab_rgba = grey.to_lab_rgba(0.6);
    assert_approx_eq!(lab_rgba.red(), 0.5);
    assert_approx_eq!(lab_rgba.green(), 0.5);
    assert_approx_eq!(lab_rgba.blue(), 0.5);
    assert_approx_eq!(lab_rgba.alpha(), 0.6);
}

#[test]
fn test_grey_creation() {
    let grey = Grey::new(0.5);

    assert_approx_eq!(grey.grey(), 0.5_f64);
}

#[test]
fn test_grey_setter() {
    let mut grey = Grey::new(0.3);

    grey.set_grey(0.7);

    assert_approx_eq!(grey.grey(), 0.7_f64);
}

#[test]
#[should_panic(expected = "Grey component must be between 0 and 1")]
fn test_grey_invalid_creation() {
    Grey::new(1.5);
}

#[test]
#[should_panic(expected = "Grey component must be between 0 and 1")]
fn test_grey_invalid_setter() {
    let mut grey = Grey::new(0.5);
    grey.set_grey(-0.1);
}

#[test]
fn test_grey_components() {
    let grey = Grey::new(0.4);
    let components = grey.components();

    assert_eq!(components.len(), 1);
    assert_approx_eq!(components[0], 0.4_f64);
}

#[test]
fn test_grey_from_components() {
    let components = [0.6];
    let grey = Grey::from_components(components);

    assert_approx_eq!(grey.grey(), 0.6_f64);
}

#[test]
fn test_grey_set_components() {
    let mut grey = Grey::new(0.3);
    let new_components = [0.8];

    grey.set_components(new_components);

    assert_approx_eq!(grey.grey(), 0.8_f64);
}

#[test]
fn test_grey_from_hex_long() {
    // Test standard hex format #GG
    let grey = Grey::<f32>::from_hex("#80").unwrap();

    assert_approx_eq!(grey.grey(), 0.5, 0.01); // Allow small rounding error
}

#[test]
fn test_grey_from_hex_short() {
    // Test shorthand hex format #G
    let grey = Grey::<f32>::from_hex("#8").unwrap();

    assert_approx_eq!(grey.grey(), 0.533, 0.01); // 8/F = 8/15 ≈ 0.533
}

#[test]
fn test_grey_to_hex() {
    let grey = Grey::new(0.5);
    let hex = grey.to_hex();

    assert_eq!(hex, "#80");
}

#[test]
fn test_grey_from_bytes() {
    let grey = Grey::<f32>::from_bytes([128]);

    assert_approx_eq!(grey.grey(), 0.502, 0.01); // 128/255 ≈ 0.502
}

#[test]
fn test_grey_to_bytes() {
    let grey = Grey::new(0.5);
    let bytes = grey.to_bytes();

    assert_eq!(bytes, [128]);
}

#[test]
fn test_grey_lerp() {
    let black = Grey::new(0.0);
    let white = Grey::new(1.0);

    // Lerp 25% of the way from black to white
    let grey25 = Grey::lerp(&black, &white, 0.25);
    assert_approx_eq!(grey25.grey(), 0.25_f64);

    // Lerp 50% of the way from black to white
    let grey50 = Grey::lerp(&black, &white, 0.5);
    assert_approx_eq!(grey50.grey(), 0.5);

    // Lerp 75% of the way from black to white
    let grey75 = Grey::lerp(&black, &white, 0.75);
    assert_approx_eq!(grey75.grey(), 0.75);
}
