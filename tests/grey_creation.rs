use approx::assert_relative_eq;
use chromatic::Grey;

#[test]
fn test_grey_creation() {
    let g1 = Grey::new(0.5f32);
    assert_relative_eq!(g1.g(), 0.5f32);
}

#[test]
#[should_panic(expected = "Grey component must be between 0 and 1")]
fn test_grey_below_range() {
    Grey::<f32>::new(-0.1);
}

#[test]
#[should_panic(expected = "Grey component must be between 0 and 1")]
fn test_grey_above_range() {
    Grey::<f32>::new(1.1);
}
