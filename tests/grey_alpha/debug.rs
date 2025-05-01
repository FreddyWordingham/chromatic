use chromatic::GreyAlpha;
use core::fmt::Debug;

// Verify GreyAlpha implements Debug trait.
#[test]
fn test_grey_alpha_debug_trait() {
    fn requires_debug<T: Debug>(_val: &T) {}

    let grey_alpha = GreyAlpha::<f32>::new(0.5, 0.5);
    requires_debug(&grey_alpha);
}

#[test]
fn test_grey_alpha_debug_output() {
    let grey_alpha = GreyAlpha::<f32>::new(0.5, 0.7);

    // Debug format should contain the value
    let debug_str = format!("{:?}", grey_alpha);
    assert!(debug_str.contains("GreyAlpha"));

    // Don't test exact format as it might change,
    // just ensure it contains necessary information
}

#[test]
fn test_grey_alpha_debug_boundary_values() {
    let zero_zero = GreyAlpha::<f32>::new(0.0, 0.0);
    let one_one = GreyAlpha::<f32>::new(1.0, 1.0);
    let zero_one = GreyAlpha::<f32>::new(0.0, 1.0);
    let one_zero = GreyAlpha::<f32>::new(1.0, 0.0);

    let zero_zero_debug = format!("{:?}", zero_zero);
    let one_one_debug = format!("{:?}", one_one);
    let zero_one_debug = format!("{:?}", zero_one);
    let one_zero_debug = format!("{:?}", one_zero);

    assert!(zero_zero_debug.contains("GreyAlpha"));
    assert!(one_one_debug.contains("GreyAlpha"));
    assert!(zero_one_debug.contains("GreyAlpha"));
    assert!(one_zero_debug.contains("GreyAlpha"));
}

#[test]
fn test_grey_alpha_debug_different_types() {
    let grey_alpha_f32 = GreyAlpha::<f32>::new(0.5, 0.5);
    let grey_alpha_f64 = GreyAlpha::<f64>::new(0.5, 0.5);

    let debug_f32 = format!("{:?}", grey_alpha_f32);
    let debug_f64 = format!("{:?}", grey_alpha_f64);

    assert!(debug_f32.contains("GreyAlpha"));
    assert!(debug_f64.contains("GreyAlpha"));
}
