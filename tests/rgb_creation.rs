use approx::assert_relative_eq;
use chromatic::Rgb;

#[test]
fn test_rgb_creation() {
    let rgb = Rgb::new(0.1f32, 0.5f32, 0.9f32);
    assert_relative_eq!(rgb.r(), 0.1f32);
    assert_relative_eq!(rgb.g(), 0.5f32);
    assert_relative_eq!(rgb.b(), 0.9f32);
}

#[test]
#[should_panic(expected = "Red component must be between 0 and 1")]
fn test_rgb_red_below_range() {
    Rgb::<f32>::new(-0.1, 0.5, 0.5);
}

#[test]
#[should_panic(expected = "Red component must be between 0 and 1")]
fn test_rgb_red_above_range() {
    Rgb::<f32>::new(1.1, 0.5, 0.5);
}

#[test]
#[should_panic(expected = "Green component must be between 0 and 1")]
fn test_rgb_green_below_range() {
    Rgb::<f32>::new(0.5, -0.1, 0.5);
}

#[test]
#[should_panic(expected = "Green component must be between 0 and 1")]
fn test_rgb_green_above_range() {
    Rgb::<f32>::new(0.5, 1.1, 0.5);
}

#[test]
#[should_panic(expected = "Blue component must be between 0 and 1")]
fn test_rgb_blue_below_range() {
    Rgb::<f32>::new(0.5, 0.5, -0.1);
}

#[test]
#[should_panic(expected = "Blue component must be between 0 and 1")]
fn test_rgb_blue_above_range() {
    Rgb::<f32>::new(0.5, 0.5, 1.1);
}
