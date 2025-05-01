use chromatic::Rgba;

// Test with single precision type.
#[test]
fn test_rgba_new_valid_f32() {
    let red = 0.25;
    let green = 0.50;
    let blue = 0.75;
    let alpha = 0.80;
    let rgba = Rgba::<f32>::new(red, green, blue, alpha);
    assert_eq!(rgba.red(), red);
    assert_eq!(rgba.green(), green);
    assert_eq!(rgba.blue(), blue);
    assert_eq!(rgba.alpha(), alpha);

    let red = 1.0 / 3.0;
    let green = 0.5;
    let blue = 2.0 / 3.0;
    let alpha = 1.0 / 4.0;
    let rgba = Rgba::<f32>::new(red, green, blue, alpha);
    assert_eq!(rgba.red(), red);
    assert_eq!(rgba.green(), green);
    assert_eq!(rgba.blue(), blue);
    assert_eq!(rgba.alpha(), alpha);
}

// Test with different double precision type.
#[test]
fn test_rgba_new_valid_f64() {
    let red = 0.25;
    let green = 0.50;
    let blue = 0.75;
    let alpha = 0.80;
    let rgba = Rgba::<f64>::new(red, green, blue, alpha);
    assert_eq!(rgba.red(), red);
    assert_eq!(rgba.green(), green);
    assert_eq!(rgba.blue(), blue);
    assert_eq!(rgba.alpha(), alpha);

    let red = 1.0 / 3.0;
    let green = 0.5;
    let blue = 2.0 / 3.0;
    let alpha = 1.0 / 4.0;
    let rgba = Rgba::<f64>::new(red, green, blue, alpha);
    assert_eq!(rgba.red(), red);
    assert_eq!(rgba.green(), green);
    assert_eq!(rgba.blue(), blue);
    assert_eq!(rgba.alpha(), alpha);
}

// Test with a small values.
#[test]
fn test_rgba_new_small_values() {
    let small_value = core::f32::EPSILON;
    let rgba = Rgba::<f32>::new(small_value, small_value, small_value, small_value);
    assert_eq!(rgba.red(), small_value);
    assert_eq!(rgba.green(), small_value);
    assert_eq!(rgba.blue(), small_value);
    assert_eq!(rgba.alpha(), small_value);

    let small_value = core::f64::EPSILON;
    let rgba = Rgba::<f64>::new(small_value, small_value, small_value, small_value);
    assert_eq!(rgba.red(), small_value);
    assert_eq!(rgba.green(), small_value);
    assert_eq!(rgba.blue(), small_value);
    assert_eq!(rgba.alpha(), small_value);
}

// Test with large values.
#[test]
fn test_rgba_new_large_values() {
    let large_value = 1.0 - core::f32::EPSILON;
    let rgba = Rgba::<f32>::new(large_value, large_value, large_value, large_value);
    assert_eq!(rgba.red(), large_value);
    assert_eq!(rgba.green(), large_value);
    assert_eq!(rgba.blue(), large_value);
    assert_eq!(rgba.alpha(), large_value);

    let large_value = 1.0 - core::f64::EPSILON;
    let rgba = Rgba::<f64>::new(large_value, large_value, large_value, large_value);
    assert_eq!(rgba.red(), large_value);
    assert_eq!(rgba.green(), large_value);
    assert_eq!(rgba.blue(), large_value);
    assert_eq!(rgba.alpha(), large_value);
}

// Test with boundary values.
#[test]
fn test_rgba_new_edge_cases() {
    let min_value = 0.0;
    let max_value = 1.0;

    // Test with black fully transparent
    let rgba = Rgba::<f32>::new(min_value, min_value, min_value, min_value);
    assert_eq!(rgba.red(), min_value);
    assert_eq!(rgba.green(), min_value);
    assert_eq!(rgba.blue(), min_value);
    assert_eq!(rgba.alpha(), min_value);

    // Test with white fully opaque
    let rgba = Rgba::<f32>::new(max_value, max_value, max_value, max_value);
    assert_eq!(rgba.red(), max_value);
    assert_eq!(rgba.green(), max_value);
    assert_eq!(rgba.blue(), max_value);
    assert_eq!(rgba.alpha(), max_value);

    // Test with red fully opaque
    let rgba = Rgba::<f32>::new(max_value, min_value, min_value, max_value);
    assert_eq!(rgba.red(), max_value);
    assert_eq!(rgba.green(), min_value);
    assert_eq!(rgba.blue(), min_value);
    assert_eq!(rgba.alpha(), max_value);

    // Test with semi-transparent blue
    let rgba = Rgba::<f32>::new(min_value, min_value, max_value, 0.5);
    assert_eq!(rgba.red(), min_value);
    assert_eq!(rgba.green(), min_value);
    assert_eq!(rgba.blue(), max_value);
    assert_eq!(rgba.alpha(), 0.5);
}

// Test red component out of bounds.
#[test]
#[should_panic(expected = "Red component")]
fn test_rgba_new_red_below_min() {
    Rgba::<f32>::new(-0.1, 0.5, 0.5, 0.5);
}

#[test]
#[should_panic(expected = "Red component")]
fn test_rgba_new_red_above_max() {
    Rgba::<f32>::new(1.1, 0.5, 0.5, 0.5);
}

// Test green component out of bounds.
#[test]
#[should_panic(expected = "Green component")]
fn test_rgba_new_green_below_min() {
    Rgba::<f32>::new(0.5, -0.1, 0.5, 0.5);
}

#[test]
#[should_panic(expected = "Green component")]
fn test_rgba_new_green_above_max() {
    Rgba::<f32>::new(0.5, 1.1, 0.5, 0.5);
}

// Test blue component out of bounds.
#[test]
#[should_panic(expected = "Blue component")]
fn test_rgba_new_blue_below_min() {
    Rgba::<f32>::new(0.5, 0.5, -0.1, 0.5);
}

#[test]
#[should_panic(expected = "Blue component")]
fn test_rgba_new_blue_above_max() {
    Rgba::<f32>::new(0.5, 0.5, 1.1, 0.5);
}

// Test alpha component out of bounds.
#[test]
#[should_panic(expected = "Alpha component")]
fn test_rgba_new_alpha_below_min() {
    Rgba::<f32>::new(0.5, 0.5, 0.5, -0.1);
}

#[test]
#[should_panic(expected = "Alpha component")]
fn test_rgba_new_alpha_above_max() {
    Rgba::<f32>::new(0.5, 0.5, 0.5, 1.1);
}
