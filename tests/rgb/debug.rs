use chromatic::Rgb;
use core::fmt::Debug;

// Verify Rgb implements Debug trait.
#[test]
fn test_rgb_debug_trait() {
    fn requires_debug<T: Debug>(_val: &T) {}

    let rgb = Rgb::<f32>::new(0.5, 0.5, 0.5);
    requires_debug(&rgb);
}

#[test]
fn test_rgb_debug_output() {
    let rgb = Rgb::<f32>::new(0.5, 0.7, 0.3);

    // Debug format should contain the value
    let debug_str = format!("{:?}", rgb);
    assert!(debug_str.contains("Rgb"));
}

#[test]
fn test_rgb_debug_boundary_values() {
    let black = Rgb::<f32>::new(0.0, 0.0, 0.0);
    let white = Rgb::<f32>::new(1.0, 1.0, 1.0);
    let red = Rgb::<f32>::new(1.0, 0.0, 0.0);
    let green = Rgb::<f32>::new(0.0, 1.0, 0.0);
    let blue = Rgb::<f32>::new(0.0, 0.0, 1.0);
    let yellow = Rgb::<f32>::new(1.0, 1.0, 0.0);
    let cyan = Rgb::<f32>::new(0.0, 1.0, 1.0);
    let magenta = Rgb::<f32>::new(1.0, 0.0, 1.0);

    let black_debug = format!("{:?}", black);
    let white_debug = format!("{:?}", white);
    let red_debug = format!("{:?}", red);
    let green_debug = format!("{:?}", green);
    let blue_debug = format!("{:?}", blue);
    let yellow_debug = format!("{:?}", yellow);
    let cyan_debug = format!("{:?}", cyan);
    let magenta_debug = format!("{:?}", magenta);

    assert!(black_debug.contains("Rgb"));
    assert!(white_debug.contains("Rgb"));
    assert!(red_debug.contains("Rgb"));
    assert!(green_debug.contains("Rgb"));
    assert!(blue_debug.contains("Rgb"));
    assert!(yellow_debug.contains("Rgb"));
    assert!(cyan_debug.contains("Rgb"));
    assert!(magenta_debug.contains("Rgb"));
}

#[test]
fn test_rgb_debug_different_types() {
    let rgb_f32 = Rgb::<f32>::new(0.5, 0.5, 0.5);
    let rgb_f64 = Rgb::<f64>::new(0.5, 0.5, 0.5);

    let debug_f32 = format!("{:?}", rgb_f32);
    let debug_f64 = format!("{:?}", rgb_f64);

    assert!(debug_f32.contains("Rgb"));
    assert!(debug_f64.contains("Rgb"));
}
