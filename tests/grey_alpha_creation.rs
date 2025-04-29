use approx::assert_relative_eq;
use chromatic::GreyAlpha;

#[test]
fn test_grey_alpha_creation() {
    let ga = GreyAlpha::new(0.5f32, 0.7f32);
    assert_relative_eq!(ga.g(), 0.5f32);
    assert_relative_eq!(ga.a(), 0.7f32);
}

#[test]
#[should_panic(expected = "Grey component must be between 0 and 1")]
fn test_grey_alpha_grey_below_range() {
    GreyAlpha::<f32>::new(-0.1, 0.5);
}

#[test]
#[should_panic(expected = "Grey component must be between 0 and 1")]
fn test_grey_alpha_grey_above_range() {
    GreyAlpha::<f32>::new(1.1, 0.5);
}

#[test]
#[should_panic(expected = "Alpha component must be between 0 and 1")]
fn test_grey_alpha_alpha_below_range() {
    GreyAlpha::<f32>::new(0.5, -0.1);
}

#[test]
#[should_panic(expected = "Alpha component must be between 0 and 1")]
fn test_grey_alpha_alpha_above_range() {
    GreyAlpha::<f32>::new(0.5, 1.1);
}
