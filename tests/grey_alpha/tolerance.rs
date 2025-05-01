use chromatic::GreyAlpha;

// Test the tolerance value for `GreyAlpha`.
#[test]
fn test_grey_alpha_tolerance_value() {
    let base_grey = 0.5;
    let base_alpha = 0.5;
    let base = GreyAlpha::<f64>::new(base_grey, base_alpha);

    // Expected tolerance is 1/256
    let tolerance = GreyAlpha::<f64>::tolerance();

    // Test grey values just inside the tolerance range
    let just_under_grey = GreyAlpha::<f64>::new(base_grey - tolerance * 0.99, base_alpha);
    let just_over_grey = GreyAlpha::<f64>::new(base_grey + tolerance * 0.99, base_alpha);

    // These should be considered equal
    assert_eq!(base, just_under_grey);
    assert_eq!(base, just_over_grey);

    // Test alpha values just inside the tolerance range
    let just_under_alpha = GreyAlpha::<f64>::new(base_grey, base_alpha - tolerance * 0.99);
    let just_over_alpha = GreyAlpha::<f64>::new(base_grey, base_alpha + tolerance * 0.99);

    // These should be considered equal
    assert_eq!(base, just_under_alpha);
    assert_eq!(base, just_over_alpha);

    // Test both components just inside the tolerance range
    let just_within_both = GreyAlpha::<f64>::new(base_grey - tolerance * 0.99, base_alpha - tolerance * 0.99);
    assert_eq!(base, just_within_both);

    // Test values just outside the tolerance range for grey
    let outside_under_grey = GreyAlpha::<f64>::new(base_grey - tolerance * 1.01, base_alpha);
    let outside_over_grey = GreyAlpha::<f64>::new(base_grey + tolerance * 1.01, base_alpha);

    // These should be considered not equal
    assert_ne!(base, outside_under_grey);
    assert_ne!(base, outside_over_grey);

    // Test values just outside the tolerance range for alpha
    let outside_under_alpha = GreyAlpha::<f64>::new(base_grey, base_alpha - tolerance * 1.01);
    let outside_over_alpha = GreyAlpha::<f64>::new(base_grey, base_alpha + tolerance * 1.01);

    // These should be considered not equal
    assert_ne!(base, outside_under_alpha);
    assert_ne!(base, outside_over_alpha);
}

// Test tolerance behavior near boundaries.
#[test]
fn test_grey_alpha_tolerance_edge_cases() {
    // Near zero values
    let zero_zero = GreyAlpha::<f64>::new(0.0, 0.0);
    let tolerance = GreyAlpha::<f64>::tolerance();

    // Test near zero for grey component
    let near_zero_grey_within = GreyAlpha::<f64>::new(tolerance * 0.99, 0.0);
    let near_zero_grey_outside = GreyAlpha::<f64>::new(tolerance * 1.01, 0.0);

    assert_eq!(zero_zero, near_zero_grey_within);
    assert_ne!(zero_zero, near_zero_grey_outside);

    // Test near zero for alpha component
    let near_zero_alpha_within = GreyAlpha::<f64>::new(0.0, tolerance * 0.99);
    let near_zero_alpha_outside = GreyAlpha::<f64>::new(0.0, tolerance * 1.01);

    assert_eq!(zero_zero, near_zero_alpha_within);
    assert_ne!(zero_zero, near_zero_alpha_outside);

    // Test near zero for both components
    let near_zero_both_within = GreyAlpha::<f64>::new(tolerance * 0.99, tolerance * 0.99);
    let near_zero_both_outside = GreyAlpha::<f64>::new(tolerance * 1.01, tolerance * 1.01);

    assert_eq!(zero_zero, near_zero_both_within);
    assert_ne!(zero_zero, near_zero_both_outside);

    // Near one values
    let one_one = GreyAlpha::<f64>::new(1.0, 1.0);

    // Test near one for grey component
    let near_one_grey_within = GreyAlpha::<f64>::new(1.0 - tolerance * 0.99, 1.0);
    let near_one_grey_outside = GreyAlpha::<f64>::new(1.0 - tolerance * 1.01, 1.0);

    assert_eq!(one_one, near_one_grey_within);
    assert_ne!(one_one, near_one_grey_outside);

    // Test near one for alpha component
    let near_one_alpha_within = GreyAlpha::<f64>::new(1.0, 1.0 - tolerance * 0.99);
    let near_one_alpha_outside = GreyAlpha::<f64>::new(1.0, 1.0 - tolerance * 1.01);

    assert_eq!(one_one, near_one_alpha_within);
    assert_ne!(one_one, near_one_alpha_outside);

    // Test near one for both components
    let near_one_both_within = GreyAlpha::<f64>::new(1.0 - tolerance * 0.99, 1.0 - tolerance * 0.99);
    let near_one_both_outside = GreyAlpha::<f64>::new(1.0 - tolerance * 1.01, 1.0 - tolerance * 1.01);

    assert_eq!(one_one, near_one_both_within);
    assert_ne!(one_one, near_one_both_outside);
}

// Test that tolerance application is consistent.
#[test]
fn test_grey_alpha_tolerance_consistency() {
    // Create a sequence of values, each separated by slightly less than the tolerance
    let tolerance = GreyAlpha::<f64>::tolerance();
    let small_step = tolerance * 0.8;

    let g1 = GreyAlpha::<f64>::new(0.1, 0.2);
    let g2 = GreyAlpha::<f64>::new(0.1 + small_step, 0.2 + small_step);
    let g3 = GreyAlpha::<f64>::new(0.1 + small_step * 2.0, 0.2 + small_step * 2.0);
    let g4 = GreyAlpha::<f64>::new(0.1 + small_step * 3.0, 0.2 + small_step * 3.0);

    // Adjacent points should be equal
    assert_eq!(g1, g2);
    assert_eq!(g2, g3);
    assert_eq!(g3, g4);

    // Points that are far apart should not be equal
    assert_ne!(g1, g4);
}

// Test that tolerance works appropriately for different precision types.
#[test]
fn test_grey_alpha_tolerance_precision() {
    // For f32
    let g1_f32 = GreyAlpha::<f32>::new(0.5, 0.5);
    let g2_f32 = GreyAlpha::<f32>::new(0.5 + 1.0 / 300.0, 0.5 + 1.0 / 300.0);
    assert_eq!(g1_f32, g2_f32);

    // For f64
    let g1_f64 = GreyAlpha::<f64>::new(0.5, 0.5);
    let g2_f64 = GreyAlpha::<f64>::new(0.5 + 1.0 / 300.0, 0.5 + 1.0 / 300.0);
    assert_eq!(g1_f64, g2_f64);

    // f64 should be able to distinguish smaller differences than f32
    let small_diff = 1.0e-7;
    let g3_f32 = GreyAlpha::<f32>::new(0.25, 0.25);
    let g4_f32 = GreyAlpha::<f32>::new(0.25 + small_diff, 0.25 + small_diff);

    let small_diff = 1.0e-7;
    let g3_f64 = GreyAlpha::<f64>::new(0.25, 0.25);
    let g4_f64 = GreyAlpha::<f64>::new(0.25 + small_diff, 0.25 + small_diff);

    // For f32, these small differences are below precision and should be equal
    assert_eq!(g3_f32, g4_f32);

    // For f64, the tolerance is still the determining factor
    assert_eq!(g3_f64, g4_f64);
}

// Tests for the internal clamping (implicit truncation) in `GreyAlpha::new`.
#[test]
fn test_grey_alpha_new_clamping() {
    // Test that values outside range are clamped properly

    // Slightly negative values should be clamped to 0
    let grey_alpha = GreyAlpha::<f64>::new(-0.001, -0.001);
    assert_eq!(grey_alpha.grey(), 0.0);
    assert_eq!(grey_alpha.alpha(), 0.0);

    // Slightly over 1 should be clamped to 1
    let grey_alpha = GreyAlpha::<f64>::new(1.001, 1.001);
    assert_eq!(grey_alpha.grey(), 1.0);
    assert_eq!(grey_alpha.alpha(), 1.0);

    // Test mixed clamping
    let grey_alpha = GreyAlpha::<f64>::new(-0.001, 1.001);
    assert_eq!(grey_alpha.grey(), 0.0);
    assert_eq!(grey_alpha.alpha(), 1.0);

    let grey_alpha = GreyAlpha::<f64>::new(1.001, -0.001);
    assert_eq!(grey_alpha.grey(), 1.0);
    assert_eq!(grey_alpha.alpha(), 0.0);
}
