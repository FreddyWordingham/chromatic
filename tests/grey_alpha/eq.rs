use chromatic::GreyAlpha;

#[test]
fn test_grey_alpha_equality() {
    let grey_alpha1 = GreyAlpha::<f32>::new(0.5, 0.5);
    let grey_alpha2 = GreyAlpha::<f32>::new(0.5, 0.5);
    let grey_alpha3 = GreyAlpha::<f32>::new(0.501, 0.5);
    let grey_alpha4 = GreyAlpha::<f32>::new(0.5, 0.501);
    let grey_alpha5 = GreyAlpha::<f32>::new(0.499, 0.499);

    // Exact equality
    assert_eq!(grey_alpha1, grey_alpha2);

    // Using the tolerance defined in the GreyAlpha impl (1/256)
    assert_eq!(grey_alpha1, grey_alpha3);
    assert_eq!(grey_alpha1, grey_alpha4);
    assert_eq!(grey_alpha1, grey_alpha5);

    // Beyond tolerance range
    let out_of_range1 = GreyAlpha::<f32>::new(0.5 + 1.0 / 128.0, 0.5);
    let out_of_range2 = GreyAlpha::<f32>::new(0.5, 0.5 + 1.0 / 128.0);

    assert_ne!(grey_alpha1, out_of_range1);
    assert_ne!(grey_alpha1, out_of_range2);
}

// Test edge cases around the tolerance boundary.
#[test]
fn test_grey_alpha_difference_boundary() {
    let tolerance = GreyAlpha::<f64>::tolerance();

    let base = GreyAlpha::<f64>::new(0.5, 0.5);

    // Test both components just within tolerance
    let just_within = GreyAlpha::<f64>::new(0.5 + tolerance * 0.99, 0.5 + tolerance * 0.99);
    assert_eq!(base, just_within);

    // Test one component just outside tolerance
    let grey_outside = GreyAlpha::<f64>::new(0.5 + tolerance * 1.01, 0.5);
    let alpha_outside = GreyAlpha::<f64>::new(0.5, 0.5 + tolerance * 1.01);

    assert_ne!(base, grey_outside);
    assert_ne!(base, alpha_outside);

    // Test both components outside tolerance
    let both_outside = GreyAlpha::<f64>::new(0.5 + tolerance * 1.01, 0.5 + tolerance * 1.01);
    assert_ne!(base, both_outside);
}

// Test equality at the boundaries.
#[test]
fn test_grey_alpha_boundary_values() {
    let zero_zero = GreyAlpha::<f32>::new(0.0, 0.0);
    let zero_one = GreyAlpha::<f32>::new(0.0, 1.0);
    let one_zero = GreyAlpha::<f32>::new(1.0, 0.0);
    let one_one = GreyAlpha::<f32>::new(1.0, 1.0);

    // Create values that are very close to the boundaries but within tolerance
    let almost_zero_zero = GreyAlpha::<f32>::new(1.0 / 512.0, 1.0 / 512.0);
    let almost_zero_one = GreyAlpha::<f32>::new(1.0 / 512.0, 1.0 - 1.0 / 512.0);
    let almost_one_zero = GreyAlpha::<f32>::new(1.0 - 1.0 / 512.0, 1.0 / 512.0);
    let almost_one_one = GreyAlpha::<f32>::new(1.0 - 1.0 / 512.0, 1.0 - 1.0 / 512.0);

    assert_eq!(zero_zero, almost_zero_zero);
    assert_eq!(zero_one, almost_zero_one);
    assert_eq!(one_zero, almost_one_zero);
    assert_eq!(one_one, almost_one_one);
}

// Test transitivity of equality.
#[test]
fn test_grey_alpha_transitivity() {
    let tolerance = GreyAlpha::<f64>::tolerance();

    let g1 = GreyAlpha::<f64>::new(0.5, 0.5);
    let g2 = GreyAlpha::<f64>::new(0.5 + tolerance * 0.5, 0.5 + tolerance * 0.5);
    let g3 = GreyAlpha::<f64>::new(0.5 + tolerance * 0.9, 0.5 + tolerance * 0.9);

    assert_eq!(g1, g2);
    assert_eq!(g2, g3);
    assert_eq!(g1, g3);
}
