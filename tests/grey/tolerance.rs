use chromatic::Grey;

// This test uses the fact that we know the internal implementation
// uses a tolerance value of 1/256 for equality comparisons
#[test]
fn test_grey_tolerance_value() {
    let base_value = 0.5;
    let base = Grey::<f64>::new(base_value);

    // Expected tolerance is 1/256
    let tolerance = 1.0 / 256.0;

    // Test values just inside the tolerance range
    let just_under = Grey::<f64>::new(base_value - tolerance * 0.99);
    let just_over = Grey::<f64>::new(base_value + tolerance * 0.99);

    // These should be considered equal
    assert_eq!(base, just_under);
    assert_eq!(base, just_over);

    // Test values just outside the tolerance range
    let outside_under = Grey::<f64>::new(base_value - tolerance * 1.01);
    let outside_over = Grey::<f64>::new(base_value + tolerance * 1.01);

    // These should be considered not equal
    assert_ne!(base, outside_under);
    assert_ne!(base, outside_over);
}

#[test]
fn test_grey_tolerance_edge_cases() {
    // Test tolerance behavior near boundaries

    // Near zero
    let zero = Grey::<f64>::new(0.0);
    let tolerance = 1.0 / 256.0;

    let near_zero_within = Grey::<f64>::new(tolerance * 0.99);
    let near_zero_outside = Grey::<f64>::new(tolerance * 1.01);

    assert_eq!(zero, near_zero_within);
    assert_ne!(zero, near_zero_outside);

    // Near one
    let one = Grey::<f64>::new(1.0);

    let near_one_within = Grey::<f64>::new(1.0 - tolerance * 0.99);
    let near_one_outside = Grey::<f64>::new(1.0 - tolerance * 1.01);

    assert_eq!(one, near_one_within);
    assert_ne!(one, near_one_outside);
}

#[test]
fn test_grey_tolerance_consistency() {
    // Test that tolerance application is consistent

    // Create a sequence of values, each separated by slightly less than the tolerance
    let tolerance = 1.0 / 256.0;
    let small_step = tolerance * 0.8;

    let g1 = Grey::<f64>::new(0.1);
    let g2 = Grey::<f64>::new(0.1 + small_step);
    let g3 = Grey::<f64>::new(0.1 + small_step * 2.0);
    let g4 = Grey::<f64>::new(0.1 + small_step * 3.0);

    // Adjacent points should be equal
    assert_eq!(g1, g2);
    assert_eq!(g2, g3);
    assert_eq!(g3, g4);

    // Points that are far apart should not be equal
    assert_ne!(g1, g4);
}

#[test]
fn test_grey_tolerance_precision() {
    // Test that tolerance works appropriately for different precision types

    // For f32
    let g1_f32 = Grey::<f32>::new(0.5);
    let g2_f32 = Grey::<f32>::new(0.5 + 1.0 / 300.0);
    assert_eq!(g1_f32, g2_f32);

    // For f64
    let g1_f64 = Grey::<f64>::new(0.5);
    let g2_f64 = Grey::<f64>::new(0.5 + 1.0 / 300.0);
    assert_eq!(g1_f64, g2_f64);

    // f64 should be able to distinguish smaller differences
    let small_diff = 1.0e-7;
    let g3_f32 = Grey::<f32>::new(0.25);
    let g4_f32 = Grey::<f32>::new(0.25 + small_diff);

    let small_diff = 1.0e-7;
    let g3_f64 = Grey::<f64>::new(0.25);
    let g4_f64 = Grey::<f64>::new(0.25 + small_diff);

    // For f32, these small differences are below precision and should be equal
    assert_eq!(g3_f32, g4_f32);

    // For f64, the tolerance is still the determining factor
    assert_eq!(g3_f64, g4_f64);
}
