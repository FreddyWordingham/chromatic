use chromatic::Grey;
use core::fmt::Debug;

// Verify Grey implements Debug trait.
#[test]
fn test_grey_debug_trait() {
    fn requires_debug<T: Debug>(_val: &T) {}

    let grey = Grey::<f32>::new(0.5);
    requires_debug(&grey);
}

#[test]
fn test_grey_debug_output() {
    let grey = Grey::<f32>::new(0.5);

    // Debug format should contain the value
    let debug_str = format!("{:?}", grey);
    assert!(debug_str.contains("Grey"));
}

#[test]
fn test_grey_debug_boundary_values() {
    let zero = Grey::<f32>::new(0.0);
    let one = Grey::<f32>::new(1.0);

    let zero_debug = format!("{:?}", zero);
    let one_debug = format!("{:?}", one);

    assert!(zero_debug.contains("Grey"));
    assert!(one_debug.contains("Grey"));
}

#[test]
fn test_grey_debug_different_types() {
    let grey_f32 = Grey::<f32>::new(0.5);
    let grey_f64 = Grey::<f64>::new(0.5);

    let debug_f32 = format!("{:?}", grey_f32);
    let debug_f64 = format!("{:?}", grey_f64);

    assert!(debug_f32.contains("Grey"));
    assert!(debug_f64.contains("Grey"));
}
