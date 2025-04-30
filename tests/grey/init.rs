use chromatic::Grey;

// Test with single precision type.
#[test]
fn test_grey_new_valid_f32() {
    let one_quarter = 0.25;
    let grey_f32 = Grey::<f32>::new(one_quarter);
    assert_eq!(grey_f32.grey(), one_quarter);

    let one_third = 1.0 / 3.0;
    let grey_f32 = Grey::<f32>::new(one_third);
    assert_eq!(grey_f32.grey(), one_third);

    let one_half = 0.5;
    let grey_f32 = Grey::<f32>::new(one_half);
    assert_eq!(grey_f32.grey(), one_half);

    let three_quarters = 0.75;
    let grey_f32 = Grey::<f32>::new(three_quarters);
    assert_eq!(grey_f32.grey(), three_quarters);
}

// Test with different double precision type.
#[test]
fn test_grey_new_valid_f64() {
    let one_quarter = 0.25;
    let grey_f64 = Grey::<f64>::new(one_quarter);
    assert_eq!(grey_f64.grey(), one_quarter);

    let one_third = 1.0 / 3.0;
    let grey_f64 = Grey::<f64>::new(one_third);
    assert_eq!(grey_f64.grey(), one_third);

    let one_half = 0.5;
    let grey_f64 = Grey::<f64>::new(one_half);
    assert_eq!(grey_f64.grey(), one_half);

    let three_quarters = 0.75;
    let grey_f64 = Grey::<f64>::new(three_quarters);
    assert_eq!(grey_f64.grey(), three_quarters);
}

// Test with a small single precision value.
#[test]
fn test_grey_new_small_f32_value() {
    let small_value = core::f32::EPSILON;
    let grey_small = Grey::<f32>::new(small_value);
    assert_eq!(grey_small.grey(), small_value);
}

// Test with a small double precision value.
#[test]
fn test_grey_new_small_f64_value() {
    let small_value = core::f64::EPSILON;
    let grey_small = Grey::<f64>::new(small_value);
    assert_eq!(grey_small.grey(), small_value);
}

// Test with a large single precision value.
#[test]
fn test_grey_new_large_f32_value() {
    let large_value = 1.0 - core::f32::EPSILON;
    let grey_large = Grey::<f32>::new(large_value);
    assert_eq!(grey_large.grey(), large_value);
}

// Test with a large double precision value.
#[test]
fn test_grey_new_large_f64_value() {
    let large_value = 1.0 - core::f64::EPSILON;
    let grey_large = Grey::<f64>::new(large_value);
    assert_eq!(grey_large.grey(), large_value);
}

// Test with boundary values.
#[test]
fn test_grey_new_edge_cases() {
    let grey_min = Grey::<f32>::new(0.0);
    let grey_max = Grey::<f32>::new(1.0);

    assert_eq!(grey_min.grey(), 0.0);
    assert_eq!(grey_max.grey(), 1.0);
}

// This should panic since value is less than 0.
#[test]
#[should_panic(expected = "Grey component must be between 0 and 1")]
fn test_grey_new_below_min() {
    Grey::<f32>::new(-0.1);
}

// This should panic since value is greater than 1.
#[test]
#[should_panic(expected = "Grey component must be between 0 and 1")]
fn test_grey_new_above_max() {
    Grey::<f32>::new(1.1);
}
