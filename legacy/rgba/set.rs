use chromatic::Rgba;

#[test]
fn test_set_rgba_valid_values() {
    let mut rgba = Rgba::<f32>::new(0.0, 0.0, 0.0, 0.0);

    // Test setting the red component
    rgba.set_red(0.25);
    assert_eq!(rgba.red(), 0.25);
    assert_eq!(rgba.green(), 0.0);
    assert_eq!(rgba.blue(), 0.0);
    assert_eq!(rgba.alpha(), 0.0);

    rgba.set_red(0.75);
    assert_eq!(rgba.red(), 0.75);
    assert_eq!(rgba.green(), 0.0);
    assert_eq!(rgba.blue(), 0.0);
    assert_eq!(rgba.alpha(), 0.0);

    // Test setting the green component
    rgba.set_green(0.5);
    assert_eq!(rgba.red(), 0.75);
    assert_eq!(rgba.green(), 0.5);
    assert_eq!(rgba.blue(), 0.0);
    assert_eq!(rgba.alpha(), 0.0);

    rgba.set_green(1.0);
    assert_eq!(rgba.red(), 0.75);
    assert_eq!(rgba.green(), 1.0);
    assert_eq!(rgba.blue(), 0.0);
    assert_eq!(rgba.alpha(), 0.0);

    // Test setting the blue component
    rgba.set_blue(0.33);
    assert_eq!(rgba.red(), 0.75);
    assert_eq!(rgba.green(), 1.0);
    assert_eq!(rgba.blue(), 0.33);
    assert_eq!(rgba.alpha(), 0.0);

    rgba.set_blue(0.67);
    assert_eq!(rgba.red(), 0.75);
    assert_eq!(rgba.green(), 1.0);
    assert_eq!(rgba.blue(), 0.67);
    assert_eq!(rgba.alpha(), 0.0);

    // Test setting the alpha component
    rgba.set_alpha(0.4);
    assert_eq!(rgba.red(), 0.75);
    assert_eq!(rgba.green(), 1.0);
    assert_eq!(rgba.blue(), 0.67);
    assert_eq!(rgba.alpha(), 0.4);

    rgba.set_alpha(0.8);
    assert_eq!(rgba.red(), 0.75);
    assert_eq!(rgba.green(), 1.0);
    assert_eq!(rgba.blue(), 0.67);
    assert_eq!(rgba.alpha(), 0.8);
}

#[test]
fn test_set_rgba_boundary_values() {
    let mut rgba = Rgba::<f64>::new(0.5, 0.5, 0.5, 0.5);

    // Test setting to minimum values
    rgba.set_red(0.0);
    assert_eq!(rgba.red(), 0.0);
    assert_eq!(rgba.green(), 0.5);
    assert_eq!(rgba.blue(), 0.5);
    assert_eq!(rgba.alpha(), 0.5);

    rgba.set_green(0.0);
    assert_eq!(rgba.red(), 0.0);
    assert_eq!(rgba.green(), 0.0);
    assert_eq!(rgba.blue(), 0.5);
    assert_eq!(rgba.alpha(), 0.5);

    rgba.set_blue(0.0);
    assert_eq!(rgba.red(), 0.0);
    assert_eq!(rgba.green(), 0.0);
    assert_eq!(rgba.blue(), 0.0);
    assert_eq!(rgba.alpha(), 0.5);

    rgba.set_alpha(0.0);
    assert_eq!(rgba.red(), 0.0);
    assert_eq!(rgba.green(), 0.0);
    assert_eq!(rgba.blue(), 0.0);
    assert_eq!(rgba.alpha(), 0.0);

    // Test setting to maximum values
    rgba.set_red(1.0);
    assert_eq!(rgba.red(), 1.0);
    assert_eq!(rgba.green(), 0.0);
    assert_eq!(rgba.blue(), 0.0);
    assert_eq!(rgba.alpha(), 0.0);

    rgba.set_green(1.0);
    assert_eq!(rgba.red(), 1.0);
    assert_eq!(rgba.green(), 1.0);
    assert_eq!(rgba.blue(), 0.0);
    assert_eq!(rgba.alpha(), 0.0);

    rgba.set_blue(1.0);
    assert_eq!(rgba.red(), 1.0);
    assert_eq!(rgba.green(), 1.0);
    assert_eq!(rgba.blue(), 1.0);
    assert_eq!(rgba.alpha(), 0.0);

    rgba.set_alpha(1.0);
    assert_eq!(rgba.red(), 1.0);
    assert_eq!(rgba.green(), 1.0);
    assert_eq!(rgba.blue(), 1.0);
    assert_eq!(rgba.alpha(), 1.0);

    // Test setting to very small positive values
    let small_value = core::f64::EPSILON;
    rgba.set_red(small_value);
    assert_eq!(rgba.red(), small_value);

    rgba.set_green(small_value);
    assert_eq!(rgba.green(), small_value);

    rgba.set_blue(small_value);
    assert_eq!(rgba.blue(), small_value);

    rgba.set_alpha(small_value);
    assert_eq!(rgba.alpha(), small_value);

    // Test setting to values very close to 1
    let near_one = 1.0 - core::f64::EPSILON;
    rgba.set_red(near_one);
    assert_eq!(rgba.red(), near_one);

    rgba.set_green(near_one);
    assert_eq!(rgba.green(), near_one);

    rgba.set_blue(near_one);
    assert_eq!(rgba.blue(), near_one);

    rgba.set_alpha(near_one);
    assert_eq!(rgba.alpha(), near_one);
}

#[test]
#[should_panic(expected = "Red component must be between 0 and 1")]
fn test_set_red_below_min() {
    let mut rgba = Rgba::<f32>::new(0.5, 0.5, 0.5, 0.5);
    rgba.set_red(-0.1);
}

#[test]
#[should_panic(expected = "Red component must be between 0 and 1")]
fn test_set_red_above_max() {
    let mut rgba = Rgba::<f32>::new(0.5, 0.5, 0.5, 0.5);
    rgba.set_red(1.1);
}

#[test]
#[should_panic(expected = "Green component must be between 0 and 1")]
fn test_set_green_below_min() {
    let mut rgba = Rgba::<f32>::new(0.5, 0.5, 0.5, 0.5);
    rgba.set_green(-0.1);
}

#[test]
#[should_panic(expected = "Green component must be between 0 and 1")]
fn test_set_green_above_max() {
    let mut rgba = Rgba::<f32>::new(0.5, 0.5, 0.5, 0.5);
    rgba.set_green(1.1);
}

#[test]
#[should_panic(expected = "Blue component must be between 0 and 1")]
fn test_set_blue_below_min() {
    let mut rgba = Rgba::<f32>::new(0.5, 0.5, 0.5, 0.5);
    rgba.set_blue(-0.1);
}

#[test]
#[should_panic(expected = "Blue component must be between 0 and 1")]
fn test_set_blue_above_max() {
    let mut rgba = Rgba::<f32>::new(0.5, 0.5, 0.5, 0.5);
    rgba.set_blue(1.1);
}

#[test]
#[should_panic(expected = "Alpha component must be between 0 and 1")]
fn test_set_alpha_below_min() {
    let mut rgba = Rgba::<f32>::new(0.5, 0.5, 0.5, 0.5);
    rgba.set_alpha(-0.1);
}

#[test]
#[should_panic(expected = "Alpha component must be between 0 and 1")]
fn test_set_alpha_above_max() {
    let mut rgba = Rgba::<f32>::new(0.5, 0.5, 0.5, 0.5);
    rgba.set_alpha(1.1);
}

#[test]
fn test_set_rgba_different_types() {
    // Test with f32
    let mut rgba_f32 = Rgba::<f32>::new(0.0, 0.0, 0.0, 0.0);
    rgba_f32.set_red(0.33);
    rgba_f32.set_green(0.66);
    rgba_f32.set_blue(0.99);
    rgba_f32.set_alpha(0.44);
    assert_eq!(rgba_f32.red(), 0.33);
    assert_eq!(rgba_f32.green(), 0.66);
    assert_eq!(rgba_f32.blue(), 0.99);
    assert_eq!(rgba_f32.alpha(), 0.44);

    // Test with f64
    let mut rgba_f64 = Rgba::<f64>::new(0.0, 0.0, 0.0, 0.0);
    rgba_f64.set_red(0.33);
    rgba_f64.set_green(0.66);
    rgba_f64.set_blue(0.99);
    rgba_f64.set_alpha(0.44);
    assert_eq!(rgba_f64.red(), 0.33);
    assert_eq!(rgba_f64.green(), 0.66);
    assert_eq!(rgba_f64.blue(), 0.99);
    assert_eq!(rgba_f64.alpha(), 0.44);
}
