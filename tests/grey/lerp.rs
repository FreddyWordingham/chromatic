use chromatic::{Colour, Grey};

// Tests for the lerp (linear interpolation) method.
#[test]
fn test_grey_lerp_basic() {
    let grey1 = Grey::<f32>::new(0.0); // Black
    let grey2 = Grey::<f32>::new(1.0); // White

    // Should be middle grey (0.5)
    let middle = Grey::<f32>::lerp(&grey1, &grey2, 0.5);
    assert_eq!(middle.grey(), 0.5);

    // Should be 1/4 of the way from black to white
    let quarter = Grey::<f32>::lerp(&grey1, &grey2, 0.25);
    assert_eq!(quarter.grey(), 0.25);

    // Should be 3/4 of the way from black to white
    let three_quarters = Grey::<f32>::lerp(&grey1, &grey2, 0.75);
    assert_eq!(three_quarters.grey(), 0.75);
}

#[test]
fn test_grey_lerp_edge_cases() {
    let grey1 = Grey::<f32>::new(0.2);
    let grey2 = Grey::<f32>::new(0.8);

    // When t = 0, result should equal the first value
    let result = Grey::<f32>::lerp(&grey1, &grey2, 0.0);
    assert_eq!(result, grey1);

    // When t = 1, result should equal the second value
    let result = Grey::<f32>::lerp(&grey1, &grey2, 1.0);
    assert_eq!(result, grey2);
}

#[test]
fn test_grey_lerp_precision() {
    // Test with different precision types
    let grey1_f32 = Grey::<f32>::new(0.1);
    let grey2_f32 = Grey::<f32>::new(0.9);
    let result_f32 = Grey::<f32>::lerp(&grey1_f32, &grey2_f32, 0.4);

    let grey1_f64 = Grey::<f64>::new(0.1);
    let grey2_f64 = Grey::<f64>::new(0.9);
    let result_f64 = Grey::<f64>::lerp(&grey1_f64, &grey2_f64, 0.4);

    // Both should yield 0.1 + 0.4*(0.9-0.1) = 0.1 + 0.32 = 0.42
    assert!((result_f32.grey() - 0.42).abs() < Grey::<f32>::tolerance());
    assert!((result_f64.grey() - 0.42).abs() < Grey::<f64>::tolerance());
}

#[test]
fn test_grey_lerp_inverse() {
    // Test lerping in both directions
    let grey1 = Grey::<f64>::new(0.2);
    let grey2 = Grey::<f64>::new(0.8);

    // Lerp from grey1 to grey2 with t = 0.3
    let result1 = Grey::<f64>::lerp(&grey1, &grey2, 0.3);
    // Expected: 0.2 + 0.3*(0.8-0.2) = 0.2 + 0.18 = 0.38
    assert!((result1.grey() - 0.38).abs() < Grey::<f64>::tolerance());

    // Lerp from grey2 to grey1 with t = 0.7
    let result2 = Grey::<f64>::lerp(&grey2, &grey1, 0.7);
    // Expected: 0.8 + 0.7*(0.2-0.8) = 0.8 - 0.42 = 0.38
    assert!((result2.grey() - 0.38).abs() < Grey::<f64>::tolerance());

    // These should be equivalent
    assert_eq!(result1, result2);
}

#[test]
#[should_panic(expected = "Interpolation factor")]
fn test_grey_lerp_t_below_min() {
    let grey1 = Grey::<f32>::new(0.3);
    let grey2 = Grey::<f32>::new(0.7);

    // This should panic because t is negative
    let _ = Grey::<f32>::lerp(&grey1, &grey2, -0.1);
}

#[test]
#[should_panic(expected = "Interpolation factor")]
fn test_grey_lerp_t_above_max() {
    let grey1 = Grey::<f32>::new(0.3);
    let grey2 = Grey::<f32>::new(0.7);

    // This should panic because t is greater than 1
    let _ = Grey::<f32>::lerp(&grey1, &grey2, 1.1);
}
