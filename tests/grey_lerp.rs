use approx::assert_relative_eq;
use chromatic::{Colour, Grey};

#[test]
fn test_grey_lerp() {
    let g1 = Grey::<f32>::new(0.0);
    let g2 = Grey::<f32>::new(1.0);

    let g_mid = g1.lerp(&g2, 0.5);
    assert_relative_eq!(g_mid.g(), 0.5);

    let g_quarter = g1.lerp(&g2, 0.25);
    assert_relative_eq!(g_quarter.g(), 0.25);
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

#[test]
#[should_panic(expected = "Lerp factor must be between 0 and 1")]
fn test_grey_lerp_below_range() {
    let g1 = Grey::<f32>::new(0.0);
    let g2 = Grey::<f32>::new(1.0);
    let _ = g1.lerp(&g2, -0.5);
}

#[test]
#[should_panic(expected = "Lerp factor must be between 0 and 1")]
fn test_grey_lerp_above_range() {
    let g1 = Grey::<f32>::new(0.0);
    let g2 = Grey::<f32>::new(1.0);
    let _ = g1.lerp(&g2, 1.5);
}
