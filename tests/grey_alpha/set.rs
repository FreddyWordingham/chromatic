use chromatic::GreyAlpha;

#[test]
fn test_set_grey_alpha_valid_values() {
    let mut grey_alpha = GreyAlpha::<f32>::new(0.0, 0.0);

    // Test setting the grey component
    grey_alpha.set_grey(0.25);
    assert_eq!(grey_alpha.grey(), 0.25);
    assert_eq!(grey_alpha.alpha(), 0.0);

    grey_alpha.set_grey(0.75);
    assert_eq!(grey_alpha.grey(), 0.75);
    assert_eq!(grey_alpha.alpha(), 0.0);

    // Test setting the alpha component
    grey_alpha.set_alpha(0.5);
    assert_eq!(grey_alpha.grey(), 0.75);
    assert_eq!(grey_alpha.alpha(), 0.5);

    grey_alpha.set_alpha(1.0);
    assert_eq!(grey_alpha.grey(), 0.75);
    assert_eq!(grey_alpha.alpha(), 1.0);
}

#[test]
fn test_set_grey_alpha_boundary_values() {
    let mut grey_alpha = GreyAlpha::<f64>::new(0.5, 0.5);

    // Test setting to minimum values
    grey_alpha.set_grey(0.0);
    assert_eq!(grey_alpha.grey(), 0.0);
    assert_eq!(grey_alpha.alpha(), 0.5);

    grey_alpha.set_alpha(0.0);
    assert_eq!(grey_alpha.grey(), 0.0);
    assert_eq!(grey_alpha.alpha(), 0.0);

    // Test setting to maximum values
    grey_alpha.set_grey(1.0);
    assert_eq!(grey_alpha.grey(), 1.0);
    assert_eq!(grey_alpha.alpha(), 0.0);

    grey_alpha.set_alpha(1.0);
    assert_eq!(grey_alpha.grey(), 1.0);
    assert_eq!(grey_alpha.alpha(), 1.0);

    // Test setting to very small positive values
    let small_value = core::f64::EPSILON;
    grey_alpha.set_grey(small_value);
    assert_eq!(grey_alpha.grey(), small_value);

    grey_alpha.set_alpha(small_value);
    assert_eq!(grey_alpha.alpha(), small_value);

    // Test setting to values very close to 1
    let near_one = 1.0 - core::f64::EPSILON;
    grey_alpha.set_grey(near_one);
    assert_eq!(grey_alpha.grey(), near_one);

    grey_alpha.set_alpha(near_one);
    assert_eq!(grey_alpha.alpha(), near_one);
}

#[test]
#[should_panic(expected = "Grey component must be between 0 and 1")]
fn test_set_grey_below_min() {
    let mut grey_alpha = GreyAlpha::<f32>::new(0.5, 0.5);
    grey_alpha.set_grey(-0.1);
}

#[test]
#[should_panic(expected = "Grey component must be between 0 and 1")]
fn test_set_grey_above_max() {
    let mut grey_alpha = GreyAlpha::<f32>::new(0.5, 0.5);
    grey_alpha.set_grey(1.1);
}

#[test]
#[should_panic(expected = "Alpha component must be between 0 and 1")]
fn test_set_alpha_below_min() {
    let mut grey_alpha = GreyAlpha::<f32>::new(0.5, 0.5);
    grey_alpha.set_alpha(-0.1);
}

#[test]
#[should_panic(expected = "Alpha component must be between 0 and 1")]
fn test_set_alpha_above_max() {
    let mut grey_alpha = GreyAlpha::<f32>::new(0.5, 0.5);
    grey_alpha.set_alpha(1.1);
}

#[test]
fn test_set_grey_alpha_different_types() {
    // Test with f32
    let mut grey_alpha_f32 = GreyAlpha::<f32>::new(0.0, 0.0);
    grey_alpha_f32.set_grey(0.33);
    grey_alpha_f32.set_alpha(0.66);
    assert_eq!(grey_alpha_f32.grey(), 0.33);
    assert_eq!(grey_alpha_f32.alpha(), 0.66);

    // Test with f64
    let mut grey_alpha_f64 = GreyAlpha::<f64>::new(0.0, 0.0);
    grey_alpha_f64.set_grey(0.33);
    grey_alpha_f64.set_alpha(0.66);
    assert_eq!(grey_alpha_f64.grey(), 0.33);
    assert_eq!(grey_alpha_f64.alpha(), 0.66);
}
