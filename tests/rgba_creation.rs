use approx::assert_relative_eq;
use chromatic::Rgba;

#[test]
fn test_rgba_creation() {
    let rgba = Rgba::new(0.1f32, 0.5f32, 0.9f32, 0.7f32);
    assert_relative_eq!(rgba.r(), 0.1f32);
    assert_relative_eq!(rgba.g(), 0.5f32);
    assert_relative_eq!(rgba.b(), 0.9f32);
    assert_relative_eq!(rgba.a(), 0.7f32);
}

#[test]
#[should_panic(expected = "Red component must be between 0 and 1")]
fn test_rgba_red_below_range() {
    Rgba::<f32>::new(-0.1, 0.5, 0.5, 0.5);
}

#[test]
#[should_panic(expected = "Red component must be between 0 and 1")]
fn test_rgba_red_above_range() {
    Rgba::<f32>::new(1.1, 0.5, 0.5, 0.5);
}

#[test]
#[should_panic(expected = "Green component must be between 0 and 1")]
fn test_rgba_green_below_range() {
    Rgba::<f32>::new(0.5, -0.1, 0.5, 0.5);
}

#[test]
#[should_panic(expected = "Green component must be between 0 and 1")]
fn test_rgba_green_above_range() {
    Rgba::<f32>::new(0.5, 1.1, 0.5, 0.5);
}

#[test]
#[should_panic(expected = "Blue component must be between 0 and 1")]
fn test_rgba_blue_below_range() {
    Rgba::<f32>::new(0.5, 0.5, -0.1, 0.5);
}

#[test]
#[should_panic(expected = "Blue component must be between 0 and 1")]
fn test_rgba_blue_above_range() {
    Rgba::<f32>::new(0.5, 0.5, 1.1, 0.5);
}

#[test]
#[should_panic(expected = "Alpha component must be between 0 and 1")]
fn test_rgba_alpha_below_range() {
    Rgba::<f32>::new(0.5, 0.5, 0.5, -0.1);
}

#[test]
#[should_panic(expected = "Alpha component must be between 0 and 1")]
fn test_rgba_alpha_above_range() {
    Rgba::<f32>::new(0.5, 0.5, 0.5, 1.1);
}
