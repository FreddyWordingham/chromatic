use chromatic::{Colour, Rgb};

#[test]
fn test_rgb_equality() {
    let rgb1 = Rgb::<f32>::new(0.5, 0.5, 0.5);
    let rgb2 = Rgb::<f32>::new(0.5, 0.5, 0.5);
    let rgb3 = Rgb::<f32>::new(0.501, 0.5, 0.5);
    let rgb4 = Rgb::<f32>::new(0.5, 0.501, 0.5);
    let rgb5 = Rgb::<f32>::new(0.5, 0.5, 0.501);
    let rgb6 = Rgb::<f32>::new(0.499, 0.499, 0.499);

    // Exact equality
    assert_eq!(rgb1, rgb2);

    // Using the tolerance defined in the Rgb impl (1/256)
    assert_eq!(rgb1, rgb3);
    assert_eq!(rgb1, rgb4);
    assert_eq!(rgb1, rgb5);
    assert_eq!(rgb1, rgb6);

    // Beyond tolerance range
    let out_of_range1 = Rgb::<f32>::new(0.5 + 1.0 / 128.0, 0.5, 0.5);
    let out_of_range2 = Rgb::<f32>::new(0.5, 0.5 + 1.0 / 128.0, 0.5);
    let out_of_range3 = Rgb::<f32>::new(0.5, 0.5, 0.5 + 1.0 / 128.0);

    assert_ne!(rgb1, out_of_range1);
    assert_ne!(rgb1, out_of_range2);
    assert_ne!(rgb1, out_of_range3);
}

// Test edge cases around the tolerance boundary.
#[test]
fn test_rgb_difference_boundary() {
    let tolerance = Rgb::<f64>::tolerance();

    let base = Rgb::<f64>::new(0.5, 0.5, 0.5);

    // Test all components just within tolerance
    let just_within = Rgb::<f64>::new(0.5 + tolerance * 0.99, 0.5 + tolerance * 0.99, 0.5 + tolerance * 0.99);
    assert_eq!(base, just_within);

    // Test one component just outside tolerance
    let red_outside = Rgb::<f64>::new(0.5 + tolerance * 1.01, 0.5, 0.5);
    let green_outside = Rgb::<f64>::new(0.5, 0.5 + tolerance * 1.01, 0.5);
    let blue_outside = Rgb::<f64>::new(0.5, 0.5, 0.5 + tolerance * 1.01);

    assert_ne!(base, red_outside);
    assert_ne!(base, green_outside);
    assert_ne!(base, blue_outside);

    // Test all components outside tolerance
    let all_outside = Rgb::<f64>::new(0.5 + tolerance * 1.01, 0.5 + tolerance * 1.01, 0.5 + tolerance * 1.01);
    assert_ne!(base, all_outside);
}

// Test equality at the boundaries.
#[test]
fn test_rgb_boundary_values() {
    let black = Rgb::<f32>::new(0.0, 0.0, 0.0);
    let white = Rgb::<f32>::new(1.0, 1.0, 1.0);
    let red = Rgb::<f32>::new(1.0, 0.0, 0.0);
    let green = Rgb::<f32>::new(0.0, 1.0, 0.0);
    let blue = Rgb::<f32>::new(0.0, 0.0, 1.0);

    // Create values that are very close to the boundaries but within tolerance
    let almost_black = Rgb::<f32>::new(1.0 / 512.0, 1.0 / 512.0, 1.0 / 512.0);
    let almost_white = Rgb::<f32>::new(1.0 - 1.0 / 512.0, 1.0 - 1.0 / 512.0, 1.0 - 1.0 / 512.0);
    let almost_red = Rgb::<f32>::new(1.0 - 1.0 / 512.0, 1.0 / 512.0, 1.0 / 512.0);
    let almost_green = Rgb::<f32>::new(1.0 / 512.0, 1.0 - 1.0 / 512.0, 1.0 / 512.0);
    let almost_blue = Rgb::<f32>::new(1.0 / 512.0, 1.0 / 512.0, 1.0 - 1.0 / 512.0);

    assert_eq!(black, almost_black);
    assert_eq!(white, almost_white);
    assert_eq!(red, almost_red);
    assert_eq!(green, almost_green);
    assert_eq!(blue, almost_blue);
}

// Test transitivity of equality.
#[test]
fn test_rgb_transitivity() {
    let tolerance = Rgb::<f64>::tolerance();

    let r1 = Rgb::<f64>::new(0.5, 0.5, 0.5);
    let r2 = Rgb::<f64>::new(0.5 + tolerance * 0.5, 0.5 + tolerance * 0.5, 0.5 + tolerance * 0.5);
    let r3 = Rgb::<f64>::new(0.5 + tolerance * 0.9, 0.5 + tolerance * 0.9, 0.5 + tolerance * 0.9);

    assert_eq!(r1, r2);
    assert_eq!(r2, r3);
    assert_eq!(r1, r3);
}
