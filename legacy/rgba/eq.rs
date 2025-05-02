use chromatic::{Colour, Rgba};

#[test]
fn test_rgba_equality() {
    let rgba1 = Rgba::<f32>::new(0.5, 0.5, 0.5, 0.5);
    let rgba2 = Rgba::<f32>::new(0.5, 0.5, 0.5, 0.5);
    let rgba3 = Rgba::<f32>::new(0.501, 0.5, 0.5, 0.5);
    let rgba4 = Rgba::<f32>::new(0.5, 0.501, 0.5, 0.5);
    let rgba5 = Rgba::<f32>::new(0.5, 0.5, 0.501, 0.5);
    let rgba6 = Rgba::<f32>::new(0.5, 0.5, 0.5, 0.501);
    let rgba7 = Rgba::<f32>::new(0.499, 0.499, 0.499, 0.499);

    // Exact equality
    assert_eq!(rgba1, rgba2);

    // Using the tolerance defined in the Rgba impl (1/256)
    assert_eq!(rgba1, rgba3);
    assert_eq!(rgba1, rgba4);
    assert_eq!(rgba1, rgba5);
    assert_eq!(rgba1, rgba6);
    assert_eq!(rgba1, rgba7);

    // Beyond tolerance range
    let out_of_range1 = Rgba::<f32>::new(0.5 + 1.0 / 128.0, 0.5, 0.5, 0.5);
    let out_of_range2 = Rgba::<f32>::new(0.5, 0.5 + 1.0 / 128.0, 0.5, 0.5);
    let out_of_range3 = Rgba::<f32>::new(0.5, 0.5, 0.5 + 1.0 / 128.0, 0.5);
    let out_of_range4 = Rgba::<f32>::new(0.5, 0.5, 0.5, 0.5 + 1.0 / 128.0);

    assert_ne!(rgba1, out_of_range1);
    assert_ne!(rgba1, out_of_range2);
    assert_ne!(rgba1, out_of_range3);
    assert_ne!(rgba1, out_of_range4);
}

// Test edge cases around the tolerance boundary.
#[test]
fn test_rgba_difference_boundary() {
    let tolerance = Rgba::<f64>::tolerance();

    let base = Rgba::<f64>::new(0.5, 0.5, 0.5, 0.5);

    // Test all components just within tolerance
    let just_within = Rgba::<f64>::new(
        0.5 + tolerance * 0.99,
        0.5 + tolerance * 0.99,
        0.5 + tolerance * 0.99,
        0.5 + tolerance * 0.99,
    );
    assert_eq!(base, just_within);

    // Test one component just outside tolerance
    let red_outside = Rgba::<f64>::new(0.5 + tolerance * 1.01, 0.5, 0.5, 0.5);
    let green_outside = Rgba::<f64>::new(0.5, 0.5 + tolerance * 1.01, 0.5, 0.5);
    let blue_outside = Rgba::<f64>::new(0.5, 0.5, 0.5 + tolerance * 1.01, 0.5);
    let alpha_outside = Rgba::<f64>::new(0.5, 0.5, 0.5, 0.5 + tolerance * 1.01);

    assert_ne!(base, red_outside);
    assert_ne!(base, green_outside);
    assert_ne!(base, blue_outside);
    assert_ne!(base, alpha_outside);

    // Test all components outside tolerance
    let all_outside = Rgba::<f64>::new(
        0.5 + tolerance * 1.01,
        0.5 + tolerance * 1.01,
        0.5 + tolerance * 1.01,
        0.5 + tolerance * 1.01,
    );
    assert_ne!(base, all_outside);
}

// Test equality at the boundaries.
#[test]
fn test_rgba_boundary_values() {
    let black_transparent = Rgba::<f32>::new(0.0, 0.0, 0.0, 0.0);
    let white_opaque = Rgba::<f32>::new(1.0, 1.0, 1.0, 1.0);
    let red_opaque = Rgba::<f32>::new(1.0, 0.0, 0.0, 1.0);
    let green_semi = Rgba::<f32>::new(0.0, 1.0, 0.0, 0.5);
    let blue_transparent = Rgba::<f32>::new(0.0, 0.0, 1.0, 0.0);

    // Create values that are very close to the boundaries but within tolerance
    let almost_black_transparent = Rgba::<f32>::new(1.0 / 512.0, 1.0 / 512.0, 1.0 / 512.0, 1.0 / 512.0);
    let almost_white_opaque = Rgba::<f32>::new(1.0 - 1.0 / 512.0, 1.0 - 1.0 / 512.0, 1.0 - 1.0 / 512.0, 1.0 - 1.0 / 512.0);
    let almost_red_opaque = Rgba::<f32>::new(1.0 - 1.0 / 512.0, 1.0 / 512.0, 1.0 / 512.0, 1.0 - 1.0 / 512.0);
    let almost_green_semi = Rgba::<f32>::new(1.0 / 512.0, 1.0 - 1.0 / 512.0, 1.0 / 512.0, 0.5 + 1.0 / 512.0);
    let almost_blue_transparent = Rgba::<f32>::new(1.0 / 512.0, 1.0 / 512.0, 1.0 - 1.0 / 512.0, 1.0 / 512.0);

    assert_eq!(black_transparent, almost_black_transparent);
    assert_eq!(white_opaque, almost_white_opaque);
    assert_eq!(red_opaque, almost_red_opaque);
    assert_eq!(green_semi, almost_green_semi);
    assert_eq!(blue_transparent, almost_blue_transparent);
}

// Test transitivity of equality.
#[test]
fn test_rgba_transitivity() {
    let tolerance = Rgba::<f64>::tolerance();

    let r1 = Rgba::<f64>::new(0.5, 0.5, 0.5, 0.5);
    let r2 = Rgba::<f64>::new(
        0.5 + tolerance * 0.5,
        0.5 + tolerance * 0.5,
        0.5 + tolerance * 0.5,
        0.5 + tolerance * 0.5,
    );
    let r3 = Rgba::<f64>::new(
        0.5 + tolerance * 0.9,
        0.5 + tolerance * 0.9,
        0.5 + tolerance * 0.9,
        0.5 + tolerance * 0.9,
    );

    assert_eq!(r1, r2);
    assert_eq!(r2, r3);
    assert_eq!(r1, r3);
}
