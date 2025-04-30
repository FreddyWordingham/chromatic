use chromatic::Grey;

#[test]
fn test_set_grey_valid_values() {
    let mut grey = Grey::<f32>::new(0.0);

    grey.set_grey(0.25);
    assert_eq!(grey.grey(), 0.25);

    grey.set_grey(0.5);
    assert_eq!(grey.grey(), 0.5);

    grey.set_grey(0.75);
    assert_eq!(grey.grey(), 0.75);

    grey.set_grey(1.0);
    assert_eq!(grey.grey(), 1.0);
}

#[test]
fn test_set_grey_boundary_values() {
    let mut grey = Grey::<f64>::new(0.5);

    // Test setting to minimum value
    grey.set_grey(0.0);
    assert_eq!(grey.grey(), 0.0);

    // Test setting to maximum value
    grey.set_grey(1.0);
    assert_eq!(grey.grey(), 1.0);

    // Test setting to a very small positive value
    let small_value = core::f64::EPSILON;
    grey.set_grey(small_value);
    assert_eq!(grey.grey(), small_value);

    // Test setting to a value very close to 1
    let near_one = 1.0 - core::f64::EPSILON;
    grey.set_grey(near_one);
    assert_eq!(grey.grey(), near_one);
}

#[test]
#[should_panic(expected = "Grey component must be between 0 and 1")]
fn test_set_grey_below_min() {
    let mut grey = Grey::<f32>::new(0.5);
    grey.set_grey(-0.1);
}

#[test]
#[should_panic(expected = "Grey component must be between 0 and 1")]
fn test_set_grey_above_max() {
    let mut grey = Grey::<f32>::new(0.5);
    grey.set_grey(1.1);
}

#[test]
fn test_set_grey_chaining() {
    let mut grey = Grey::<f32>::new(0.0);

    grey.set_grey(0.1);
    grey.set_grey(0.2);
    grey.set_grey(0.3);

    assert_eq!(grey.grey(), 0.3);
}

#[test]
fn test_set_grey_different_types() {
    // Test with f32
    let mut grey_f32 = Grey::<f32>::new(0.0);
    grey_f32.set_grey(0.33);
    assert_eq!(grey_f32.grey(), 0.33);

    // Test with f64
    let mut grey_f64 = Grey::<f64>::new(0.0);
    grey_f64.set_grey(0.33);
    assert_eq!(grey_f64.grey(), 0.33);
}
