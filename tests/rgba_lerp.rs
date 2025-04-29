use approx::assert_relative_eq;
use chromatic::{Colour, Rgba};

#[test]
fn test_rgba_lerp() {
    let rgba1 = Rgba::<f32>::new(0.0, 0.0, 0.0, 0.0);
    let rgba2 = Rgba::<f32>::new(1.0, 1.0, 1.0, 1.0);

    let rgba_mid = rgba1.lerp(&rgba2, 0.5);
    assert_relative_eq!(rgba_mid.r(), 0.5);
    assert_relative_eq!(rgba_mid.g(), 0.5);
    assert_relative_eq!(rgba_mid.b(), 0.5);
    assert_relative_eq!(rgba_mid.a(), 0.5);

    let rgba_quarter = rgba1.lerp(&rgba2, 0.25);
    assert_relative_eq!(rgba_quarter.r(), 0.25);
    assert_relative_eq!(rgba_quarter.g(), 0.25);
    assert_relative_eq!(rgba_quarter.b(), 0.25);
    assert_relative_eq!(rgba_quarter.a(), 0.25);
}

#[test]
#[should_panic(expected = "Lerp factor must be between 0 and 1")]
fn test_rgba_lerp_below_range() {
    let rgba1 = Rgba::<f32>::new(0.0, 0.0, 0.0, 0.0);
    let rgba2 = Rgba::<f32>::new(1.0, 1.0, 1.0, 1.0);
    let _ = rgba1.lerp(&rgba2, -0.5);
}

#[test]
#[should_panic(expected = "Lerp factor must be between 0 and 1")]
fn test_rgba_lerp_above_range() {
    let rgba1 = Rgba::<f32>::new(0.0, 0.0, 0.0, 0.0);
    let rgba2 = Rgba::<f32>::new(1.0, 1.0, 1.0, 1.0);
    let _ = rgba1.lerp(&rgba2, 1.5);
}
