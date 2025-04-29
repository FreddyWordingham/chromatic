use approx::assert_relative_eq;
use chromatic::{Colour, GreyAlpha};

#[test]
fn test_grey_alpha_lerp() {
    let ga1 = GreyAlpha::<f32>::new(0.0, 0.0);
    let ga2 = GreyAlpha::<f32>::new(1.0, 1.0);

    let ga_mid = ga1.lerp(&ga2, 0.5);
    assert_relative_eq!(ga_mid.g(), 0.5);
    assert_relative_eq!(ga_mid.a(), 0.5);

    let ga_quarter = ga1.lerp(&ga2, 0.25);
    assert_relative_eq!(ga_quarter.g(), 0.25);
    assert_relative_eq!(ga_quarter.a(), 0.25);
}

#[test]
#[should_panic(expected = "Lerp factor must be between 0 and 1")]
fn test_grey_alpha_lerp_below_range() {
    let ga1 = GreyAlpha::<f32>::new(0.0, 0.0);
    let ga2 = GreyAlpha::<f32>::new(1.0, 1.0);
    let _ = ga1.lerp(&ga2, -0.5);
}

#[test]
#[should_panic(expected = "Lerp factor must be between 0 and 1")]
fn test_grey_alpha_lerp_above_range() {
    let ga1 = GreyAlpha::<f32>::new(0.0, 0.0);
    let ga2 = GreyAlpha::<f32>::new(1.0, 1.0);
    let _ = ga1.lerp(&ga2, 1.5);
}
