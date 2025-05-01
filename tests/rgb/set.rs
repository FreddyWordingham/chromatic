use chromatic::Rgb;

#[test]
fn test_set_rgb_valid_values() {
    let mut rgb = Rgb::<f32>::new(0.0, 0.0, 0.0);

    // Test setting the red component
    rgb.set_red(0.25);
    assert_eq!(rgb.red(), 0.25);
    assert_eq!(rgb.green(), 0.0);
    assert_eq!(rgb.blue(), 0.0);

    rgb.set_red(0.75);
    assert_eq!(rgb.red(), 0.75);
    assert_eq!(rgb.green(), 0.0);
    assert_eq!(rgb.blue(), 0.0);

    // Test setting the green component
    rgb.set_green(0.5);
    assert_eq!(rgb.red(), 0.75);
    assert_eq!(rgb.green(), 0.5);
    assert_eq!(rgb.blue(), 0.0);

    rgb.set_green(1.0);
    assert_eq!(rgb.red(), 0.75);
    assert_eq!(rgb.green(), 1.0);
    assert_eq!(rgb.blue(), 0.0);

    // Test setting the blue component
    rgb.set_blue(0.33);
    assert_eq!(rgb.red(), 0.75);
    assert_eq!(rgb.green(), 1.0);
    assert_eq!(rgb.blue(), 0.33);

    rgb.set_blue(0.67);
    assert_eq!(rgb.red(), 0.75);
    assert_eq!(rgb.green(), 1.0);
    assert_eq!(rgb.blue(), 0.67);
}

#[test]
fn test_set_rgb_boundary_values() {
    let mut rgb = Rgb::<f64>::new(0.5, 0.5, 0.5);

    // Test setting to minimum values
    rgb.set_red(0.0);
    assert_eq!(rgb.red(), 0.0);
    assert_eq!(rgb.green(), 0.5);
    assert_eq!(rgb.blue(), 0.5);

    rgb.set_green(0.0);
    assert_eq!(rgb.red(), 0.0);
    assert_eq!(rgb.green(), 0.0);
    assert_eq!(rgb.blue(), 0.5);

    rgb.set_blue(0.0);
    assert_eq!(rgb.red(), 0.0);
    assert_eq!(rgb.green(), 0.0);
    assert_eq!(rgb.blue(), 0.0);

    // Test setting to maximum values
    rgb.set_red(1.0);
    assert_eq!(rgb.red(), 1.0);
    assert_eq!(rgb.green(), 0.0);
    assert_eq!(rgb.blue(), 0.0);

    rgb.set_green(1.0);
    assert_eq!(rgb.red(), 1.0);
    assert_eq!(rgb.green(), 1.0);
    assert_eq!(rgb.blue(), 0.0);

    rgb.set_blue(1.0);
    assert_eq!(rgb.red(), 1.0);
    assert_eq!(rgb.green(), 1.0);
    assert_eq!(rgb.blue(), 1.0);

    // Test setting to very small positive values
    let small_value = core::f64::EPSILON;
    rgb.set_red(small_value);
    assert_eq!(rgb.red(), small_value);

    rgb.set_green(small_value);
    assert_eq!(rgb.green(), small_value);

    rgb.set_blue(small_value);
    assert_eq!(rgb.blue(), small_value);

    // Test setting to values very close to 1
    let near_one = 1.0 - core::f64::EPSILON;
    rgb.set_red(near_one);
    assert_eq!(rgb.red(), near_one);

    rgb.set_green(near_one);
    assert_eq!(rgb.green(), near_one);

    rgb.set_blue(near_one);
    assert_eq!(rgb.blue(), near_one);
}

#[test]
#[should_panic(expected = "Red component must be between 0 and 1")]
fn test_set_red_below_min() {
    let mut rgb = Rgb::<f32>::new(0.5, 0.5, 0.5);
    rgb.set_red(-0.1);
}

#[test]
#[should_panic(expected = "Red component must be between 0 and 1")]
fn test_set_red_above_max() {
    let mut rgb = Rgb::<f32>::new(0.5, 0.5, 0.5);
    rgb.set_red(1.1);
}

#[test]
#[should_panic(expected = "Green component must be between 0 and 1")]
fn test_set_green_below_min() {
    let mut rgb = Rgb::<f32>::new(0.5, 0.5, 0.5);
    rgb.set_green(-0.1);
}

#[test]
#[should_panic(expected = "Green component must be between 0 and 1")]
fn test_set_green_above_max() {
    let mut rgb = Rgb::<f32>::new(0.5, 0.5, 0.5);
    rgb.set_green(1.1);
}

#[test]
#[should_panic(expected = "Blue component must be between 0 and 1")]
fn test_set_blue_below_min() {
    let mut rgb = Rgb::<f32>::new(0.5, 0.5, 0.5);
    rgb.set_blue(-0.1);
}

#[test]
#[should_panic(expected = "Blue component must be between 0 and 1")]
fn test_set_blue_above_max() {
    let mut rgb = Rgb::<f32>::new(0.5, 0.5, 0.5);
    rgb.set_blue(1.1);
}

#[test]
fn test_set_rgb_different_types() {
    // Test with f32
    let mut rgb_f32 = Rgb::<f32>::new(0.0, 0.0, 0.0);
    rgb_f32.set_red(0.33);
    rgb_f32.set_green(0.66);
    rgb_f32.set_blue(0.99);
    assert_eq!(rgb_f32.red(), 0.33);
    assert_eq!(rgb_f32.green(), 0.66);
    assert_eq!(rgb_f32.blue(), 0.99);

    // Test with f64
    let mut rgb_f64 = Rgb::<f64>::new(0.0, 0.0, 0.0);
    rgb_f64.set_red(0.33);
    rgb_f64.set_green(0.66);
    rgb_f64.set_blue(0.99);
    assert_eq!(rgb_f64.red(), 0.33);
    assert_eq!(rgb_f64.green(), 0.66);
    assert_eq!(rgb_f64.blue(), 0.99);
}
