use chromatic::Rgba;
use core::fmt::Debug;

// Verify Rgba implements Debug trait.
#[test]
fn test_rgba_debug_trait() {
    fn requires_debug<T: Debug>(_val: &T) {}

    let rgba = Rgba::<f32>::new(0.5, 0.5, 0.5, 0.5);
    requires_debug(&rgba);
}

#[test]
fn test_rgba_debug_output() {
    let rgba = Rgba::<f32>::new(0.5, 0.7, 0.3, 0.6);

    // Debug format should contain the value
    let debug_str = format!("{:?}", rgba);
    assert!(debug_str.contains("Rgba"));
}

#[test]
fn test_rgba_debug_boundary_values() {
    let black_transparent = Rgba::<f32>::new(0.0, 0.0, 0.0, 0.0);
    let white_opaque = Rgba::<f32>::new(1.0, 1.0, 1.0, 1.0);
    let red_opaque = Rgba::<f32>::new(1.0, 0.0, 0.0, 1.0);
    let green_opaque = Rgba::<f32>::new(0.0, 1.0, 0.0, 1.0);
    let blue_opaque = Rgba::<f32>::new(0.0, 0.0, 1.0, 1.0);
    let yellow_semi = Rgba::<f32>::new(1.0, 1.0, 0.0, 0.5);
    let cyan_semi = Rgba::<f32>::new(0.0, 1.0, 1.0, 0.5);
    let magenta_semi = Rgba::<f32>::new(1.0, 0.0, 1.0, 0.5);

    let black_debug = format!("{:?}", black_transparent);
    let white_debug = format!("{:?}", white_opaque);
    let red_debug = format!("{:?}", red_opaque);
    let green_debug = format!("{:?}", green_opaque);
    let blue_debug = format!("{:?}", blue_opaque);
    let yellow_debug = format!("{:?}", yellow_semi);
    let cyan_debug = format!("{:?}", cyan_semi);
    let magenta_debug = format!("{:?}", magenta_semi);

    assert!(black_debug.contains("Rgba"));
    assert!(white_debug.contains("Rgba"));
    assert!(red_debug.contains("Rgba"));
    assert!(green_debug.contains("Rgba"));
    assert!(blue_debug.contains("Rgba"));
    assert!(yellow_debug.contains("Rgba"));
    assert!(cyan_debug.contains("Rgba"));
    assert!(magenta_debug.contains("Rgba"));
}

#[test]
fn test_rgba_debug_different_types() {
    let rgba_f32 = Rgba::<f32>::new(0.5, 0.5, 0.5, 0.5);
    let rgba_f64 = Rgba::<f64>::new(0.5, 0.5, 0.5, 0.5);

    let debug_f32 = format!("{:?}", rgba_f32);
    let debug_f64 = format!("{:?}", rgba_f64);

    assert!(debug_f32.contains("Rgba"));
    assert!(debug_f64.contains("Rgba"));
}
