use assert_approx_eq::assert_approx_eq;
use chromatic::{Colour, Rgb};

#[test]
fn test_rgb_creation() {
    let rgb = Rgb::new(0.5, 0.6, 0.7);

    assert_approx_eq!(rgb.red(), 0.5_f64);
    assert_approx_eq!(rgb.green(), 0.6);
    assert_approx_eq!(rgb.blue(), 0.7);
}

#[test]
fn test_rgb_setters() {
    let mut rgb = Rgb::new(0.1, 0.2, 0.3);

    rgb.set_red(0.4);
    rgb.set_green(0.5);
    rgb.set_blue(0.6);

    assert_approx_eq!(rgb.red(), 0.4_f64);
    assert_approx_eq!(rgb.green(), 0.5);
    assert_approx_eq!(rgb.blue(), 0.6);
}

#[test]
#[should_panic(expected = "Red component must be between 0 and 1")]
fn test_rgb_invalid_red_creation() {
    Rgb::new(1.5, 0.5, 0.5);
}

#[test]
#[should_panic(expected = "Green component must be between 0 and 1")]
fn test_rgb_invalid_green_creation() {
    Rgb::new(0.5, 1.5, 0.5);
}

#[test]
#[should_panic(expected = "Blue component must be between 0 and 1")]
fn test_rgb_invalid_blue_creation() {
    Rgb::new(0.5, 0.5, 1.5);
}

#[test]
#[should_panic(expected = "Red component must be between 0 and 1")]
fn test_rgb_invalid_red_setter() {
    let mut rgb = Rgb::new(0.5, 0.5, 0.5);
    rgb.set_red(-0.1);
}

#[test]
#[should_panic(expected = "Green component must be between 0 and 1")]
fn test_rgb_invalid_green_setter() {
    let mut rgb = Rgb::new(0.5, 0.5, 0.5);
    rgb.set_green(-0.1);
}

#[test]
#[should_panic(expected = "Blue component must be between 0 and 1")]
fn test_rgb_invalid_blue_setter() {
    let mut rgb = Rgb::new(0.5, 0.5, 0.5);
    rgb.set_blue(-0.1);
}

#[test]
fn test_rgb_components() {
    let rgb = Rgb::new(0.1, 0.2, 0.3);
    let components = rgb.components();

    assert_eq!(components.len(), 3);
    assert_approx_eq!(components[0], 0.1_f64);
    assert_approx_eq!(components[1], 0.2);
    assert_approx_eq!(components[2], 0.3);
}

#[test]
fn test_rgb_from_components() {
    let components = [0.4, 0.5, 0.6];
    let rgb = Rgb::from_components(components);

    assert_approx_eq!(rgb.red(), 0.4_f64);
    assert_approx_eq!(rgb.green(), 0.5);
    assert_approx_eq!(rgb.blue(), 0.6);
}

#[test]
fn test_rgb_set_components() {
    let mut rgb = Rgb::new(0.1, 0.2, 0.3);
    let new_components = [0.4, 0.5, 0.6];

    rgb.set_components(new_components);

    assert_approx_eq!(rgb.red(), 0.4_f64);
    assert_approx_eq!(rgb.green(), 0.5);
    assert_approx_eq!(rgb.blue(), 0.6);
}

#[test]
fn test_rgb_from_hex_long() {
    // Test standard hex format #RRGGBB
    let rgb = Rgb::<f32>::from_hex("#FF7F00").unwrap();

    assert_approx_eq!(rgb.red(), 1.0);
    assert_approx_eq!(rgb.green(), 0.5, 0.01); // Allow small rounding error
    assert_approx_eq!(rgb.blue(), 0.0);
}

#[test]
fn test_rgb_from_hex_short() {
    // Test shorthand hex format #RGB
    let rgb = Rgb::<f32>::from_hex("#F70").unwrap();

    assert_approx_eq!(rgb.red(), 1.0);
    assert_approx_eq!(rgb.green(), 0.467, 0.01); // 7/F = 7/15 ≈ 0.467
    assert_approx_eq!(rgb.blue(), 0.0);
}

#[test]
fn test_rgb_to_hex() {
    let rgb = Rgb::new(1.0, 0.5, 0.0);
    let hex = rgb.to_hex();

    assert_eq!(hex, "#FF8000");
}

#[test]
fn test_rgb_from_bytes() {
    let rgb = Rgb::<f32>::from_bytes([255, 128, 0]);

    assert_approx_eq!(rgb.red(), 1.0);
    assert_approx_eq!(rgb.green(), 0.502, 0.01); // 128/255 ≈ 0.502
    assert_approx_eq!(rgb.blue(), 0.0);
}

#[test]
fn test_rgb_to_bytes() {
    let rgb = Rgb::new(1.0, 0.5, 0.0);
    let bytes = rgb.to_bytes();

    assert_eq!(bytes, [255, 128, 0]);
}

#[test]
fn test_rgb_lerp() {
    let red = Rgb::new(1.0, 0.0, 0.0);
    let blue = Rgb::new(0.0, 0.0, 1.0);

    // Lerp 25% of the way from red to blue
    let purple25 = Rgb::lerp(&red, &blue, 0.25);
    assert_approx_eq!(purple25.red(), 0.75_f64);
    assert_approx_eq!(purple25.green(), 0.0);
    assert_approx_eq!(purple25.blue(), 0.25);

    // Lerp 50% of the way from red to blue
    let purple50 = Rgb::lerp(&red, &blue, 0.5);
    assert_approx_eq!(purple50.red(), 0.5);
    assert_approx_eq!(purple50.green(), 0.0);
    assert_approx_eq!(purple50.blue(), 0.5);

    // Lerp 75% of the way from red to blue
    let purple75 = Rgb::lerp(&red, &blue, 0.75);
    assert_approx_eq!(purple75.red(), 0.25);
    assert_approx_eq!(purple75.green(), 0.0);
    assert_approx_eq!(purple75.blue(), 0.75);
}

#[test]
#[should_panic(expected = "Interpolation factor must be in range [0, 1]")]
fn test_rgb_lerp_out_of_range_below() {
    let red = Rgb::new(1.0, 0.0, 0.0);
    let blue = Rgb::new(0.0, 0.0, 1.0);

    let _ = Rgb::lerp(&red, &blue, -0.1);
}

#[test]
#[should_panic(expected = "Interpolation factor must be in range [0, 1]")]
fn test_rgb_lerp_out_of_range_above() {
    let red = Rgb::new(1.0, 0.0, 0.0);
    let blue = Rgb::new(0.0, 0.0, 1.0);

    let _ = Rgb::lerp(&red, &blue, 1.1);
}

#[test]
fn test_rgb_mix() {
    let red = Rgb::new(1.0, 0.0, 0.0);
    let green = Rgb::new(0.0, 1.0, 0.0);
    let blue = Rgb::new(0.0, 0.0, 1.0);

    // Mix with equal weights
    let colours = vec![red, green, blue];
    let weights = vec![1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0];

    let mixed = Rgb::mix(&colours, &weights);

    assert_approx_eq!(mixed.red(), 1.0_f64 / 3.0);
    assert_approx_eq!(mixed.green(), 1.0 / 3.0);
    assert_approx_eq!(mixed.blue(), 1.0 / 3.0);

    // Mix with different weights
    let weights = vec![0.5, 0.3, 0.2];

    let mixed = Rgb::mix(&colours, &weights);

    assert_approx_eq!(mixed.red(), 0.5);
    assert_approx_eq!(mixed.green(), 0.3);
    assert_approx_eq!(mixed.blue(), 0.2);
}

#[test]
fn test_rgb_mix_single_color() {
    let red = Rgb::new(1.0, 0.0, 0.0);

    let colours = vec![red];
    let weights = vec![1.0];

    let mixed = Rgb::mix(&colours, &weights);

    assert_eq!(mixed, red);
}

#[test]
#[should_panic(expected = "Cannot mix an empty list of colours")]
fn test_rgb_mix_empty() {
    let colours: Vec<Rgb<f32>> = vec![];
    let weights: Vec<f32> = vec![];

    let _ = Rgb::mix(&colours, &weights);
}

#[test]
#[should_panic(expected = "Colours and weights must have the same length")]
fn test_rgb_mix_mismatched_lengths() {
    let red = Rgb::new(1.0, 0.0, 0.0);
    let green = Rgb::new(0.0, 1.0, 0.0);

    let colours = vec![red, green];
    let weights = vec![1.0];

    let _ = Rgb::mix(&colours, &weights);
}

#[test]
#[should_panic(expected = "Weights must be non-negative")]
fn test_rgb_mix_negative_weights() {
    let red = Rgb::new(1.0, 0.0, 0.0);
    let green = Rgb::new(0.0, 1.0, 0.0);

    let colours = vec![red, green];
    let weights = vec![1.0, -0.5];

    let _ = Rgb::mix(&colours, &weights);
}

#[test]
fn test_rgb_equality() {
    let rgb1 = Rgb::new(0.5, 0.6, 0.7);
    let rgb2 = Rgb::new(0.5, 0.6, 0.7);
    let rgb3 = Rgb::new(0.5, 0.6, 0.8);

    assert_eq!(rgb1, rgb2);
    assert_ne!(rgb1, rgb3);

    // Test with small differences within tolerance
    let tolerance = Rgb::<f32>::tolerance();
    let rgb4 = Rgb::new(0.5 + tolerance * 0.5, 0.6, 0.7);

    assert_eq!(rgb1, rgb4);

    // Test with small differences outside tolerance
    let rgb5 = Rgb::new(0.5 + tolerance * 1.5, 0.6, 0.7);

    assert_ne!(rgb1, rgb5);
}

#[test]
fn test_rgb_with_f64() {
    let rgb = Rgb::<f64>::new(0.5, 0.6, 0.7);

    assert_eq!(rgb.red(), 0.5);
    assert_eq!(rgb.green(), 0.6);
    assert_eq!(rgb.blue(), 0.7);
}
