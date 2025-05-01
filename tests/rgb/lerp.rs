use chromatic::{Colour, Rgb};

// Tests for the lerp (linear interpolation) method.
#[test]
fn test_rgb_lerp_basic() {
    let black = Rgb::<f32>::new(0.0, 0.0, 0.0);
    let white = Rgb::<f32>::new(1.0, 1.0, 1.0);

    // Should be middle gray (0.5, 0.5, 0.5)
    let middle = Rgb::<f32>::lerp(&black, &white, 0.5);
    assert_eq!(middle.red(), 0.5);
    assert_eq!(middle.green(), 0.5);
    assert_eq!(middle.blue(), 0.5);

    // Should be 1/4 of the way from black to white
    let quarter = Rgb::<f32>::lerp(&black, &white, 0.25);
    assert_eq!(quarter.red(), 0.25);
    assert_eq!(quarter.green(), 0.25);
    assert_eq!(quarter.blue(), 0.25);

    // Should be 3/4 of the way from black to white
    let three_quarters = Rgb::<f32>::lerp(&black, &white, 0.75);
    assert_eq!(three_quarters.red(), 0.75);
    assert_eq!(three_quarters.green(), 0.75);
    assert_eq!(three_quarters.blue(), 0.75);
}

#[test]
fn test_rgb_lerp_edge_cases() {
    let rgb1 = Rgb::<f32>::new(0.2, 0.3, 0.4);
    let rgb2 = Rgb::<f32>::new(0.8, 0.7, 0.6);

    // When t = 0, result should equal the first value
    let result = Rgb::<f32>::lerp(&rgb1, &rgb2, 0.0);
    assert_eq!(result, rgb1);

    // When t = 1, result should equal the second value
    let result = Rgb::<f32>::lerp(&rgb1, &rgb2, 1.0);
    assert_eq!(result, rgb2);
}

#[test]
fn test_rgb_lerp_precision() {
    // Test with different precision types
    let rgb1_f32 = Rgb::<f32>::new(0.1, 0.2, 0.3);
    let rgb2_f32 = Rgb::<f32>::new(0.9, 0.8, 0.7);
    let result_f32 = Rgb::<f32>::lerp(&rgb1_f32, &rgb2_f32, 0.4);

    let rgb1_f64 = Rgb::<f64>::new(0.1, 0.2, 0.3);
    let rgb2_f64 = Rgb::<f64>::new(0.9, 0.8, 0.7);
    let result_f64 = Rgb::<f64>::lerp(&rgb1_f64, &rgb2_f64, 0.4);

    // Calculate expected values:
    // Red: 0.1 + 0.4*(0.9-0.1) = 0.1 + 0.32 = 0.42
    // Green: 0.2 + 0.4*(0.8-0.2) = 0.2 + 0.24 = 0.44
    // Blue: 0.3 + 0.4*(0.7-0.3) = 0.3 + 0.16 = 0.46
    assert!((result_f32.red() - 0.42).abs() < Rgb::<f32>::tolerance());
    assert!((result_f32.green() - 0.44).abs() < Rgb::<f32>::tolerance());
    assert!((result_f32.blue() - 0.46).abs() < Rgb::<f32>::tolerance());

    assert!((result_f64.red() - 0.42).abs() < Rgb::<f64>::tolerance());
    assert!((result_f64.green() - 0.44).abs() < Rgb::<f64>::tolerance());
    assert!((result_f64.blue() - 0.46).abs() < Rgb::<f64>::tolerance());
}

#[test]
fn test_rgb_lerp_inverse() {
    // Test lerping in both directions
    let rgb1 = Rgb::<f64>::new(0.2, 0.3, 0.4);
    let rgb2 = Rgb::<f64>::new(0.8, 0.7, 0.6);

    // Lerp from rgb1 to rgb2 with t = 0.3
    let result1 = Rgb::<f64>::lerp(&rgb1, &rgb2, 0.3);
    // Expected red: 0.2 + 0.3*(0.8-0.2) = 0.2 + 0.18 = 0.38
    // Expected green: 0.3 + 0.3*(0.7-0.3) = 0.3 + 0.12 = 0.42
    // Expected blue: 0.4 + 0.3*(0.6-0.4) = 0.4 + 0.06 = 0.46
    assert!((result1.red() - 0.38).abs() < Rgb::<f64>::tolerance());
    assert!((result1.green() - 0.42).abs() < Rgb::<f64>::tolerance());
    assert!((result1.blue() - 0.46).abs() < Rgb::<f64>::tolerance());

    // Lerp from rgb2 to rgb1 with t = 0.7
    let result2 = Rgb::<f64>::lerp(&rgb2, &rgb1, 0.7);
    // Expected red: 0.8 + 0.7*(0.2-0.8) = 0.8 - 0.42 = 0.38
    // Expected green: 0.7 + 0.7*(0.3-0.7) = 0.7 - 0.28 = 0.42
    // Expected blue: 0.6 + 0.7*(0.4-0.6) = 0.6 - 0.14 = 0.46
    assert!((result2.red() - 0.38).abs() < Rgb::<f64>::tolerance());
    assert!((result2.green() - 0.42).abs() < Rgb::<f64>::tolerance());
    assert!((result2.blue() - 0.46).abs() < Rgb::<f64>::tolerance());

    // These should be equivalent
    assert_eq!(result1, result2);
}

#[test]
#[should_panic(expected = "Interpolation factor")]
fn test_rgb_lerp_t_below_min() {
    let rgb1 = Rgb::<f32>::new(0.3, 0.3, 0.3);
    let rgb2 = Rgb::<f32>::new(0.7, 0.7, 0.7);

    // This should panic because t is negative
    let _ = Rgb::<f32>::lerp(&rgb1, &rgb2, -0.1);
}

#[test]
#[should_panic(expected = "Interpolation factor")]
fn test_rgb_lerp_t_above_max() {
    let rgb1 = Rgb::<f32>::new(0.3, 0.3, 0.3);
    let rgb2 = Rgb::<f32>::new(0.7, 0.7, 0.7);

    // This should panic because t is greater than 1
    let _ = Rgb::<f32>::lerp(&rgb1, &rgb2, 1.1);
}

#[test]
fn test_rgb_lerp_different_components() {
    // Test lerping between colors with large differences in individual components
    let red = Rgb::<f32>::new(1.0, 0.0, 0.0);
    let blue = Rgb::<f32>::new(0.0, 0.0, 1.0);

    // Halfway should be purple (0.5, 0.0, 0.5)
    let purple = Rgb::<f32>::lerp(&red, &blue, 0.5);
    assert_eq!(purple.red(), 0.5);
    assert_eq!(purple.green(), 0.0);
    assert_eq!(purple.blue(), 0.5);

    // Test with more complex colors
    let orange = Rgb::<f64>::new(1.0, 0.5, 0.0);
    let teal = Rgb::<f64>::new(0.0, 0.5, 0.5);

    // Lerp at t = 0.25
    let result = Rgb::<f64>::lerp(&orange, &teal, 0.25);
    // Expected red: 1.0 + 0.25*(-1.0) = 0.75
    // Expected green: 0.5 + 0.25*(0.0) = 0.5
    // Expected blue: 0.0 + 0.25*(0.5) = 0.125
    assert!((result.red() - 0.75).abs() < Rgb::<f64>::tolerance());
    assert!((result.green() - 0.5).abs() < Rgb::<f64>::tolerance());
    assert!((result.blue() - 0.125).abs() < Rgb::<f64>::tolerance());
}
