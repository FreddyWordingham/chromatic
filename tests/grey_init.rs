use approx::assert_relative_eq;
use chromatic::Grey;

#[test]
fn test_grey_creation() {
    let g1 = Grey::new(0.5f32);
    assert_relative_eq!(g1.g(), 0.5f32);
}
