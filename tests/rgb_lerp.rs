use approx::assert_relative_eq;
use chromatic::{Colour, Rgb};

#[test]
fn test_rgb_lerp() {
    let rgb1 = Rgb::<f32>::new(0.0, 0.0, 0.0);
    let rgb2 = Rgb::<f32>::new(1.0, 1.0, 1.0);

    let rgb_mid = rgb1.lerp(&rgb2, 0.5);
    assert_relative_eq!(rgb_mid.r(), 0.5);
    assert_relative_eq!(rgb_mid.g(), 0.5);
    assert_relative_eq!(rgb_mid.b(), 0.5);

    let rgb_quarter = rgb1.lerp(&rgb2, 0.25);
    assert_relative_eq!(rgb_quarter.r(), 0.25);
    assert_relative_eq!(rgb_quarter.g(), 0.25);
    assert_relative_eq!(rgb_quarter.b(), 0.25);
}

#[test]
#[should_panic(expected = "Lerp factor must be between 0 and 1")]
fn test_rgb_lerp_below_range() {
    let rgb1 = Rgb::<f32>::new(0.0, 0.0, 0.0);
    let rgb2 = Rgb::<f32>::new(1.0, 1.0, 1.0);
    let _ = rgb1.lerp(&rgb2, -0.5);
}

#[test]
#[should_panic(expected = "Lerp factor must be between 0 and 1")]
fn test_rgb_lerp_above_range() {
    let rgb1 = Rgb::<f32>::new(0.0, 0.0, 0.0);
    let rgb2 = Rgb::<f32>::new(1.0, 1.0, 1.0);
    let _ = rgb1.lerp(&rgb2, 1.5);
}
