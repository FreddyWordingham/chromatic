// rgba/tolerance.rs
use chromatic::{Colour, Rgba};

// Test the tolerance value for `Rgba`.
#[test]
fn test_rgba_tolerance_value() {
    let base_red = 0.5;
    let base_green = 0.6;
    let base_blue = 0.7;
    let base_alpha = 0.8;
    let base = Rgba::<f64>::new(base_red, base_green, base_blue, base_alpha);

    // Expected tolerance is 1/256
    let tolerance = Rgba::<f64>::tolerance();

    // Test red values just inside the tolerance range
    let just_under_red = Rgba::<f64>::new(base_red - tolerance * 0.99, base_green, base_blue, base_alpha);
    let just_over_red = Rgba::<f64>::new(base_red + tolerance * 0.99, base_green, base_blue, base_alpha);

    // These should be considered equal
    assert_eq!(base, just_under_red);
    assert_eq!(base, just_over_red);

    // Test green values just inside the tolerance range
    let just_under_green = Rgba::<f64>::new(base_red, base_green - tolerance * 0.99, base_blue, base_alpha);
    let just_over_green = Rgba::<f64>::new(base_red, base_green + tolerance * 0.99, base_blue, base_alpha);

    // These should be considered equal
    assert_eq!(base, just_under_green);
    assert_eq!(base, just_over_green);

    // Test blue values just inside the tolerance range
    let just_under_blue = Rgba::<f64>::new(base_red, base_green, base_blue - tolerance * 0.99, base_alpha);
    let just_over_blue = Rgba::<f64>::new(base_red, base_green, base_blue + tolerance * 0.99, base_alpha);

    // These should be considered equal
    assert_eq!(base, just_under_blue);
    assert_eq!(base, just_over_blue);

    // Test alpha values just inside the tolerance range
    let just_under_alpha = Rgba::<f64>::new(base_red, base_green, base_blue, base_alpha - tolerance * 0.99);
    let just_over_alpha = Rgba::<f64>::new(base_red, base_green, base_blue, base_alpha + tolerance * 0.99);

    // These should be considered equal
    assert_eq!(base, just_under_alpha);
    assert_eq!(base, just_over_alpha);

    // Test all components just inside the tolerance range
    let just_within_all = Rgba::<f64>::new(
        base_red - tolerance * 0.99,
        base_green - tolerance * 0.99,
        base_blue - tolerance * 0.99,
        base_alpha - tolerance * 0.99,
    );
    assert_eq!(base, just_within_all);

    // Test values just outside the tolerance range for red
    let outside_under_red = Rgba::<f64>::new(base_red - tolerance * 1.01, base_green, base_blue, base_alpha);
    let outside_over_red = Rgba::<f64>::new(base_red + tolerance * 1.01, base_green, base_blue, base_alpha);

    // These should be considered not equal
    assert_ne!(base, outside_under_red);
    assert_ne!(base, outside_over_red);

    // Test values just outside the tolerance range for green
    let outside_under_green = Rgba::<f64>::new(base_red, base_green - tolerance * 1.01, base_blue, base_alpha);
    let outside_over_green = Rgba::<f64>::new(base_red, base_green + tolerance * 1.01, base_blue, base_alpha);

    // These should be considered not equal
    assert_ne!(base, outside_under_green);
    assert_ne!(base, outside_over_green);

    // Test values just outside the tolerance range for blue
    let outside_under_blue = Rgba::<f64>::new(base_red, base_green, base_blue - tolerance * 1.01, base_alpha);
    let outside_over_blue = Rgba::<f64>::new(base_red, base_green, base_blue + tolerance * 1.01, base_alpha);

    // These should be considered not equal
    assert_ne!(base, outside_under_blue);
    assert_ne!(base, outside_over_blue);

    // Test values just outside the tolerance range for alpha
    let outside_under_alpha = Rgba::<f64>::new(base_red, base_green, base_blue, base_alpha - tolerance * 1.01);
    let outside_over_alpha = Rgba::<f64>::new(base_red, base_green, base_blue, base_alpha + tolerance * 1.01);

    // These should be considered not equal
    assert_ne!(base, outside_under_alpha);
    assert_ne!(base, outside_over_alpha);
}

// Test tolerance behavior near boundaries.
#[test]
fn test_rgba_tolerance_edge_cases() {
    // Near zero values
    let zero_rgba = Rgba::<f64>::new(0.0, 0.0, 0.0, 0.0);
    let tolerance = Rgba::<f64>::tolerance();

    // Test near zero for red component
    let near_zero_red_within = Rgba::<f64>::new(tolerance * 0.99, 0.0, 0.0, 0.0);
    let near_zero_red_outside = Rgba::<f64>::new(tolerance * 1.01, 0.0, 0.0, 0.0);

    assert_eq!(zero_rgba, near_zero_red_within);
    assert_ne!(zero_rgba, near_zero_red_outside);

    // Test near zero for green component
    let near_zero_green_within = Rgba::<f64>::new(0.0, tolerance * 0.99, 0.0, 0.0);
    let near_zero_green_outside = Rgba::<f64>::new(0.0, tolerance * 1.01, 0.0, 0.0);

    assert_eq!(zero_rgba, near_zero_green_within);
    assert_ne!(zero_rgba, near_zero_green_outside);

    // Test near zero for blue component
    let near_zero_blue_within = Rgba::<f64>::new(0.0, 0.0, tolerance * 0.99, 0.0);
    let near_zero_blue_outside = Rgba::<f64>::new(0.0, 0.0, tolerance * 1.01, 0.0);

    assert_eq!(zero_rgba, near_zero_blue_within);
    assert_ne!(zero_rgba, near_zero_blue_outside);

    // Test near zero for alpha component
    let near_zero_alpha_within = Rgba::<f64>::new(0.0, 0.0, 0.0, tolerance * 0.99);
    let near_zero_alpha_outside = Rgba::<f64>::new(0.0, 0.0, 0.0, tolerance * 1.01);

    assert_eq!(zero_rgba, near_zero_alpha_within);
    assert_ne!(zero_rgba, near_zero_alpha_outside);

    // Test near zero for all components
    let near_zero_all_within = Rgba::<f64>::new(tolerance * 0.99, tolerance * 0.99, tolerance * 0.99, tolerance * 0.99);
    let near_zero_all_outside = Rgba::<f64>::new(tolerance * 1.01, tolerance * 1.01, tolerance * 1.01, tolerance * 1.01);

    assert_eq!(zero_rgba, near_zero_all_within);
    assert_ne!(zero_rgba, near_zero_all_outside);

    // Near one values
    let one_rgba = Rgba::<f64>::new(1.0, 1.0, 1.0, 1.0);

    // Test near one for red component
    let near_one_red_within = Rgba::<f64>::new(1.0 - tolerance * 0.99, 1.0, 1.0, 1.0);
    let near_one_red_outside = Rgba::<f64>::new(1.0 - tolerance * 1.01, 1.0, 1.0, 1.0);

    assert_eq!(one_rgba, near_one_red_within);
    assert_ne!(one_rgba, near_one_red_outside);

    // Test near one for green component
    let near_one_green_within = Rgba::<f64>::new(1.0, 1.0 - tolerance * 0.99, 1.0, 1.0);
    let near_one_green_outside = Rgba::<f64>::new(1.0, 1.0 - tolerance * 1.01, 1.0, 1.0);

    assert_eq!(one_rgba, near_one_green_within);
    assert_ne!(one_rgba, near_one_green_outside);

    // Test near one for blue component
    let near_one_blue_within = Rgba::<f64>::new(1.0, 1.0, 1.0 - tolerance * 0.99, 1.0);
    let near_one_blue_outside = Rgba::<f64>::new(1.0, 1.0, 1.0 - tolerance * 1.01, 1.0);

    assert_eq!(one_rgba, near_one_blue_within);
    assert_ne!(one_rgba, near_one_blue_outside);

    // Test near one for alpha component
    let near_one_alpha_within = Rgba::<f64>::new(1.0, 1.0, 1.0, 1.0 - tolerance * 0.99);
    let near_one_alpha_outside = Rgba::<f64>::new(1.0, 1.0, 1.0, 1.0 - tolerance * 1.01);

    assert_eq!(one_rgba, near_one_alpha_within);
    assert_ne!(one_rgba, near_one_alpha_outside);

    // Test near one for all components
    let near_one_all_within = Rgba::<f64>::new(
        1.0 - tolerance * 0.99,
        1.0 - tolerance * 0.99,
        1.0 - tolerance * 0.99,
        1.0 - tolerance * 0.99,
    );
    let near_one_all_outside = Rgba::<f64>::new(
        1.0 - tolerance * 1.01,
        1.0 - tolerance * 1.01,
        1.0 - tolerance * 1.01,
        1.0 - tolerance * 1.01,
    );

    assert_eq!(one_rgba, near_one_all_within);
    assert_ne!(one_rgba, near_one_all_outside);
}

// Test that tolerance application is consistent.
#[test]
fn test_rgba_tolerance_consistency() {
    // Create a sequence of values, each separated by slightly less than the tolerance
    let tolerance = Rgba::<f64>::tolerance();
    let small_step = tolerance * 0.8;

    let r1 = Rgba::<f64>::new(0.1, 0.2, 0.3, 0.4);
    let r2 = Rgba::<f64>::new(0.1 + small_step, 0.2 + small_step, 0.3 + small_step, 0.4 + small_step);
    let r3 = Rgba::<f64>::new(
        0.1 + small_step * 2.0,
        0.2 + small_step * 2.0,
        0.3 + small_step * 2.0,
        0.4 + small_step * 2.0,
    );
    let r4 = Rgba::<f64>::new(
        0.1 + small_step * 3.0,
        0.2 + small_step * 3.0,
        0.3 + small_step * 3.0,
        0.4 + small_step * 3.0,
    );

    // Adjacent points should be equal
    assert_eq!(r1, r2);
    assert_eq!(r2, r3);
    assert_eq!(r3, r4);

    // Points that are far apart should not be equal
    assert_ne!(r1, r4);
}

// Test that tolerance works appropriately for different precision types.
#[test]
fn test_rgba_tolerance_precision() {
    // For f32
    let r1_f32 = Rgba::<f32>::new(0.5, 0.5, 0.5, 0.5);
    let r2_f32 = Rgba::<f32>::new(0.5 + 1.0 / 300.0, 0.5 + 1.0 / 300.0, 0.5 + 1.0 / 300.0, 0.5 + 1.0 / 300.0);
    assert_eq!(r1_f32, r2_f32);

    // For f64
    let r1_f64 = Rgba::<f64>::new(0.5, 0.5, 0.5, 0.5);
    let r2_f64 = Rgba::<f64>::new(0.5 + 1.0 / 300.0, 0.5 + 1.0 / 300.0, 0.5 + 1.0 / 300.0, 0.5 + 1.0 / 300.0);
    assert_eq!(r1_f64, r2_f64);

    // f64 should be able to distinguish smaller differences than f32
    let small_diff = 1.0e-7;
    let r3_f32 = Rgba::<f32>::new(0.25, 0.25, 0.25, 0.25);
    let r4_f32 = Rgba::<f32>::new(0.25 + small_diff, 0.25 + small_diff, 0.25 + small_diff, 0.25 + small_diff);

    let small_diff = 1.0e-7;
    let r3_f64 = Rgba::<f64>::new(0.25, 0.25, 0.25, 0.25);
    let r4_f64 = Rgba::<f64>::new(0.25 + small_diff, 0.25 + small_diff, 0.25 + small_diff, 0.25 + small_diff);

    // For f32, these small differences are below precision and should be equal
    assert_eq!(r3_f32, r4_f32);

    // For f64, the tolerance is still the determining factor
    assert_eq!(r3_f64, r4_f64);
}

// Tests for the internal clamping (implicit truncation) in `Rgba::new`.
#[test]
fn test_rgba_new_clamping() {
    // Test that values outside range are clamped properly

    // Slightly negative values should be clamped to 0
    let rgba = Rgba::<f64>::new(-0.001, -0.001, -0.001, -0.001);
    assert_eq!(rgba.red(), 0.0);
    assert_eq!(rgba.green(), 0.0);
    assert_eq!(rgba.blue(), 0.0);
    assert_eq!(rgba.alpha(), 0.0);

    // Slightly over 1 should be clamped to 1
    let rgba = Rgba::<f64>::new(1.001, 1.001, 1.001, 1.001);
    assert_eq!(rgba.red(), 1.0);
    assert_eq!(rgba.green(), 1.0);
    assert_eq!(rgba.blue(), 1.0);
    assert_eq!(rgba.alpha(), 1.0);

    // Test mixed clamping
    let rgba = Rgba::<f64>::new(-0.001, 0.5, 1.001, 0.3);
    assert_eq!(rgba.red(), 0.0);
    assert_eq!(rgba.green(), 0.5);
    assert_eq!(rgba.blue(), 1.0);
    assert_eq!(rgba.alpha(), 0.3);
}
