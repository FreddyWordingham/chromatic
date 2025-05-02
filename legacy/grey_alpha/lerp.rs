use chromatic::{Colour, GreyAlpha};

// Tests for the lerp (linear interpolation) method.
#[test]
fn test_grey_alpha_lerp_basic() {
    let black_transparent = GreyAlpha::<f32>::new(0.0, 0.0);
    let white_opaque = GreyAlpha::<f32>::new(1.0, 1.0);

    // Should be middle grey, middle alpha (0.5, 0.5)
    let middle = GreyAlpha::<f32>::lerp(&black_transparent, &white_opaque, 0.5);
    assert_eq!(middle.grey(), 0.5);
    assert_eq!(middle.alpha(), 0.5);

    // Should be 1/4 of the way from black_transparent to white_opaque
    let quarter = GreyAlpha::<f32>::lerp(&black_transparent, &white_opaque, 0.25);
    assert_eq!(quarter.grey(), 0.25);
    assert_eq!(quarter.alpha(), 0.25);

    // Should be 3/4 of the way from black_transparent to white_opaque
    let three_quarters = GreyAlpha::<f32>::lerp(&black_transparent, &white_opaque, 0.75);
    assert_eq!(three_quarters.grey(), 0.75);
    assert_eq!(three_quarters.alpha(), 0.75);
}

#[test]
fn test_grey_alpha_lerp_edge_cases() {
    let grey_alpha1 = GreyAlpha::<f32>::new(0.2, 0.3);
    let grey_alpha2 = GreyAlpha::<f32>::new(0.8, 0.7);

    // When t = 0, result should equal the first value
    let result = GreyAlpha::<f32>::lerp(&grey_alpha1, &grey_alpha2, 0.0);
    assert_eq!(result, grey_alpha1);

    // When t = 1, result should equal the second value
    let result = GreyAlpha::<f32>::lerp(&grey_alpha1, &grey_alpha2, 1.0);
    assert_eq!(result, grey_alpha2);
}

#[test]
fn test_grey_alpha_lerp_precision() {
    // Test with different precision types
    let grey_alpha1_f32 = GreyAlpha::<f32>::new(0.1, 0.2);
    let grey_alpha2_f32 = GreyAlpha::<f32>::new(0.9, 0.8);
    let result_f32 = GreyAlpha::<f32>::lerp(&grey_alpha1_f32, &grey_alpha2_f32, 0.4);

    let grey_alpha1_f64 = GreyAlpha::<f64>::new(0.1, 0.2);
    let grey_alpha2_f64 = GreyAlpha::<f64>::new(0.9, 0.8);
    let result_f64 = GreyAlpha::<f64>::lerp(&grey_alpha1_f64, &grey_alpha2_f64, 0.4);

    // Calculate expected values:
    // Grey: 0.1 + 0.4*(0.9-0.1) = 0.1 + 0.32 = 0.42
    // Alpha: 0.2 + 0.4*(0.8-0.2) = 0.2 + 0.24 = 0.44
    assert!((result_f32.grey() - 0.42).abs() < GreyAlpha::<f32>::tolerance());
    assert!((result_f32.alpha() - 0.44).abs() < GreyAlpha::<f32>::tolerance());
    assert!((result_f64.grey() - 0.42).abs() < GreyAlpha::<f64>::tolerance());
    assert!((result_f64.alpha() - 0.44).abs() < GreyAlpha::<f64>::tolerance());
}

#[test]
fn test_grey_alpha_lerp_inverse() {
    // Test lerping in both directions
    let grey_alpha1 = GreyAlpha::<f64>::new(0.2, 0.3);
    let grey_alpha2 = GreyAlpha::<f64>::new(0.8, 0.7);

    // Lerp from grey_alpha1 to grey_alpha2 with t = 0.3
    let result1 = GreyAlpha::<f64>::lerp(&grey_alpha1, &grey_alpha2, 0.3);
    // Expected grey: 0.2 + 0.3*(0.8-0.2) = 0.2 + 0.18 = 0.38
    // Expected alpha: 0.3 + 0.3*(0.7-0.3) = 0.3 + 0.12 = 0.42
    assert!((result1.grey() - 0.38).abs() < GreyAlpha::<f64>::tolerance());
    assert!((result1.alpha() - 0.42).abs() < GreyAlpha::<f64>::tolerance());

    // Lerp from grey_alpha2 to grey_alpha1 with t = 0.7
    let result2 = GreyAlpha::<f64>::lerp(&grey_alpha2, &grey_alpha1, 0.7);
    // Expected grey: 0.8 + 0.7*(0.2-0.8) = 0.8 - 0.42 = 0.38
    // Expected alpha: 0.7 + 0.7*(0.3-0.7) = 0.7 - 0.28 = 0.42
    assert!((result2.grey() - 0.38).abs() < GreyAlpha::<f64>::tolerance());
    assert!((result2.alpha() - 0.42).abs() < GreyAlpha::<f64>::tolerance());

    // These should be equivalent
    assert_eq!(result1, result2);
}

#[test]
#[should_panic(expected = "Interpolation factor")]
fn test_grey_alpha_lerp_t_below_min() {
    let grey_alpha1 = GreyAlpha::<f32>::new(0.3, 0.4);
    let grey_alpha2 = GreyAlpha::<f32>::new(0.7, 0.6);

    // This should panic because t is negative
    let _ = GreyAlpha::<f32>::lerp(&grey_alpha1, &grey_alpha2, -0.1);
}

#[test]
#[should_panic(expected = "Interpolation factor")]
fn test_grey_alpha_lerp_t_above_max() {
    let grey_alpha1 = GreyAlpha::<f32>::new(0.3, 0.4);
    let grey_alpha2 = GreyAlpha::<f32>::new(0.7, 0.6);

    // This should panic because t is greater than 1
    let _ = GreyAlpha::<f32>::lerp(&grey_alpha1, &grey_alpha2, 1.1);
}
