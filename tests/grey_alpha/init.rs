use chromatic::GreyAlpha;

// Test with single precision type.
#[test]
fn test_grey_alpha_new_valid_f32() {
    let grey = 0.25;
    let alpha = 0.75;
    let grey_alpha = GreyAlpha::<f32>::new(grey, alpha);
    assert_eq!(grey_alpha.grey(), grey);
    assert_eq!(grey_alpha.alpha(), alpha);

    let grey = 1.0 / 3.0;
    let alpha = 2.0 / 3.0;
    let grey_alpha = GreyAlpha::<f32>::new(grey, alpha);
    assert_eq!(grey_alpha.grey(), grey);
    assert_eq!(grey_alpha.alpha(), alpha);

    let grey = 0.5;
    let alpha = 0.5;
    let grey_alpha = GreyAlpha::<f32>::new(grey, alpha);
    assert_eq!(grey_alpha.grey(), grey);
    assert_eq!(grey_alpha.alpha(), alpha);
}

// Test with different double precision type.
#[test]
fn test_grey_alpha_new_valid_f64() {
    let grey = 0.25;
    let alpha = 0.75;
    let grey_alpha = GreyAlpha::<f64>::new(grey, alpha);
    assert_eq!(grey_alpha.grey(), grey);
    assert_eq!(grey_alpha.alpha(), alpha);

    let grey = 1.0 / 3.0;
    let alpha = 2.0 / 3.0;
    let grey_alpha = GreyAlpha::<f64>::new(grey, alpha);
    assert_eq!(grey_alpha.grey(), grey);
    assert_eq!(grey_alpha.alpha(), alpha);

    let grey = 0.5;
    let alpha = 0.5;
    let grey_alpha = GreyAlpha::<f64>::new(grey, alpha);
    assert_eq!(grey_alpha.grey(), grey);
    assert_eq!(grey_alpha.alpha(), alpha);
}

// Test with a small values.
#[test]
fn test_grey_alpha_new_small_values() {
    let small_value = core::f32::EPSILON;
    let grey_alpha = GreyAlpha::<f32>::new(small_value, small_value);
    assert_eq!(grey_alpha.grey(), small_value);
    assert_eq!(grey_alpha.alpha(), small_value);

    let small_value = core::f64::EPSILON;
    let grey_alpha = GreyAlpha::<f64>::new(small_value, small_value);
    assert_eq!(grey_alpha.grey(), small_value);
    assert_eq!(grey_alpha.alpha(), small_value);
}

// Test with large values.
#[test]
fn test_grey_alpha_new_large_values() {
    let large_value = 1.0 - core::f32::EPSILON;
    let grey_alpha = GreyAlpha::<f32>::new(large_value, large_value);
    assert_eq!(grey_alpha.grey(), large_value);
    assert_eq!(grey_alpha.alpha(), large_value);

    let large_value = 1.0 - core::f64::EPSILON;
    let grey_alpha = GreyAlpha::<f64>::new(large_value, large_value);
    assert_eq!(grey_alpha.grey(), large_value);
    assert_eq!(grey_alpha.alpha(), large_value);
}

// Test with boundary values.
#[test]
fn test_grey_alpha_new_edge_cases() {
    let min_value = 0.0;
    let max_value = 1.0;

    // Test all combinations of min and max
    let grey_alpha1 = GreyAlpha::<f32>::new(min_value, min_value);
    let grey_alpha2 = GreyAlpha::<f32>::new(min_value, max_value);
    let grey_alpha3 = GreyAlpha::<f32>::new(max_value, min_value);
    let grey_alpha4 = GreyAlpha::<f32>::new(max_value, max_value);

    assert_eq!(grey_alpha1.grey(), min_value);
    assert_eq!(grey_alpha1.alpha(), min_value);

    assert_eq!(grey_alpha2.grey(), min_value);
    assert_eq!(grey_alpha2.alpha(), max_value);

    assert_eq!(grey_alpha3.grey(), max_value);
    assert_eq!(grey_alpha3.alpha(), min_value);

    assert_eq!(grey_alpha4.grey(), max_value);
    assert_eq!(grey_alpha4.alpha(), max_value);
}

// Test grey component out of bounds.
#[test]
#[should_panic(expected = "Grey component")]
fn test_grey_alpha_new_grey_below_min() {
    GreyAlpha::<f32>::new(-0.1, 0.5);
}

#[test]
#[should_panic(expected = "Grey component")]
fn test_grey_alpha_new_grey_above_max() {
    GreyAlpha::<f32>::new(1.1, 0.5);
}

// Test alpha component out of bounds.
#[test]
#[should_panic(expected = "Alpha component")]
fn test_grey_alpha_new_alpha_below_min() {
    GreyAlpha::<f32>::new(0.5, -0.1);
}

#[test]
#[should_panic(expected = "Alpha component")]
fn test_grey_alpha_new_alpha_above_max() {
    GreyAlpha::<f32>::new(0.5, 1.1);
}
