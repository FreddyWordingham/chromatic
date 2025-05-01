use chromatic::Rgb;

// Test with single precision type.
#[test]
fn test_rgb_new_valid_f32() {
    let red = 0.25;
    let green = 0.50;
    let blue = 0.75;
    let rgb = Rgb::<f32>::new(red, green, blue);
    assert_eq!(rgb.red(), red);
    assert_eq!(rgb.green(), green);
    assert_eq!(rgb.blue(), blue);

    let red = 1.0 / 3.0;
    let green = 0.5;
    let blue = 2.0 / 3.0;
    let rgb = Rgb::<f32>::new(red, green, blue);
    assert_eq!(rgb.red(), red);
    assert_eq!(rgb.green(), green);
    assert_eq!(rgb.blue(), blue);
}

// Test with different double precision type.
#[test]
fn test_rgb_new_valid_f64() {
    let red = 0.25;
    let green = 0.50;
    let blue = 0.75;
    let rgb = Rgb::<f64>::new(red, green, blue);
    assert_eq!(rgb.red(), red);
    assert_eq!(rgb.green(), green);
    assert_eq!(rgb.blue(), blue);

    let red = 1.0 / 3.0;
    let green = 0.5;
    let blue = 2.0 / 3.0;
    let rgb = Rgb::<f64>::new(red, green, blue);
    assert_eq!(rgb.red(), red);
    assert_eq!(rgb.green(), green);
    assert_eq!(rgb.blue(), blue);
}

// Test with a small values.
#[test]
fn test_rgb_new_small_values() {
    let small_value = core::f32::EPSILON;
    let rgb = Rgb::<f32>::new(small_value, small_value, small_value);
    assert_eq!(rgb.red(), small_value);
    assert_eq!(rgb.green(), small_value);
    assert_eq!(rgb.blue(), small_value);

    let small_value = core::f64::EPSILON;
    let rgb = Rgb::<f64>::new(small_value, small_value, small_value);
    assert_eq!(rgb.red(), small_value);
    assert_eq!(rgb.green(), small_value);
    assert_eq!(rgb.blue(), small_value);
}

// Test with large values.
#[test]
fn test_rgb_new_large_values() {
    let large_value = 1.0 - core::f32::EPSILON;
    let rgb = Rgb::<f32>::new(large_value, large_value, large_value);
    assert_eq!(rgb.red(), large_value);
    assert_eq!(rgb.green(), large_value);
    assert_eq!(rgb.blue(), large_value);

    let large_value = 1.0 - core::f64::EPSILON;
    let rgb = Rgb::<f64>::new(large_value, large_value, large_value);
    assert_eq!(rgb.red(), large_value);
    assert_eq!(rgb.green(), large_value);
    assert_eq!(rgb.blue(), large_value);
}

// Test with boundary values.
#[test]
fn test_rgb_new_edge_cases() {
    let min_value = 0.0;
    let max_value = 1.0;

    // Test all combinations of min and max
    let rgb1 = Rgb::<f32>::new(min_value, min_value, min_value);
    let rgb2 = Rgb::<f32>::new(max_value, min_value, min_value);
    let rgb3 = Rgb::<f32>::new(min_value, max_value, min_value);
    let rgb4 = Rgb::<f32>::new(min_value, min_value, max_value);
    let rgb5 = Rgb::<f32>::new(max_value, max_value, min_value);
    let rgb6 = Rgb::<f32>::new(max_value, min_value, max_value);
    let rgb7 = Rgb::<f32>::new(min_value, max_value, max_value);
    let rgb8 = Rgb::<f32>::new(max_value, max_value, max_value);

    assert_eq!(rgb1.red(), min_value);
    assert_eq!(rgb1.green(), min_value);
    assert_eq!(rgb1.blue(), min_value);

    assert_eq!(rgb2.red(), max_value);
    assert_eq!(rgb2.green(), min_value);
    assert_eq!(rgb2.blue(), min_value);

    assert_eq!(rgb3.red(), min_value);
    assert_eq!(rgb3.green(), max_value);
    assert_eq!(rgb3.blue(), min_value);

    assert_eq!(rgb4.red(), min_value);
    assert_eq!(rgb4.green(), min_value);
    assert_eq!(rgb4.blue(), max_value);

    assert_eq!(rgb5.red(), max_value);
    assert_eq!(rgb5.green(), max_value);
    assert_eq!(rgb5.blue(), min_value);

    assert_eq!(rgb6.red(), max_value);
    assert_eq!(rgb6.green(), min_value);
    assert_eq!(rgb6.blue(), max_value);

    assert_eq!(rgb7.red(), min_value);
    assert_eq!(rgb7.green(), max_value);
    assert_eq!(rgb7.blue(), max_value);

    assert_eq!(rgb8.red(), max_value);
    assert_eq!(rgb8.green(), max_value);
    assert_eq!(rgb8.blue(), max_value);
}

// Test red component out of bounds.
#[test]
#[should_panic(expected = "Red component")]
fn test_rgb_new_red_below_min() {
    Rgb::<f32>::new(-0.1, 0.5, 0.5);
}

#[test]
#[should_panic(expected = "Red component")]
fn test_rgb_new_red_above_max() {
    Rgb::<f32>::new(1.1, 0.5, 0.5);
}

// Test green component out of bounds.
#[test]
#[should_panic(expected = "Green component")]
fn test_rgb_new_green_below_min() {
    Rgb::<f32>::new(0.5, -0.1, 0.5);
}

#[test]
#[should_panic(expected = "Green component")]
fn test_rgb_new_green_above_max() {
    Rgb::<f32>::new(0.5, 1.1, 0.5);
}

// Test blue component out of bounds.
#[test]
#[should_panic(expected = "Blue component")]
fn test_rgb_new_blue_below_min() {
    Rgb::<f32>::new(0.5, 0.5, -0.1);
}

#[test]
#[should_panic(expected = "Blue component")]
fn test_rgb_new_blue_above_max() {
    Rgb::<f32>::new(0.5, 0.5, 1.1);
}
