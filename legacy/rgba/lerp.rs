use chromatic::{Colour, Rgba};

#[test]
fn test_rgba_lerp_interpolation_points() {
    let c1 = Rgba::<f32>::new(0.2, 0.3, 0.4, 0.1);
    let c2 = Rgba::<f32>::new(0.8, 0.7, 0.6, 0.9);

    // At t=0, should be equal to c1
    let result = Rgba::lerp(&c1, &c2, 0.0);
    assert_eq!(result, c1);

    // At t=1, should be equal to c2
    let result = Rgba::lerp(&c1, &c2, 1.0);
    assert_eq!(result, c2);

    // Test multiple interpolation points
    for t in 0..=10 {
        let t_value = t as f32 / 10.0;
        let result = Rgba::lerp(&c1, &c2, t_value);

        // For RGB, using Lab space makes this not a direct linear interpolation
        // But alpha should be directly linear
        assert!((result.alpha() - (c1.alpha() * (1.0 - t_value) + c2.alpha() * t_value)).abs() < 1e-6);
    }
}

#[test]
#[should_panic(expected = "Interpolation factor")]
fn test_rgba_lerp_t_below_zero() {
    let c1 = Rgba::<f32>::new(0.0, 0.0, 0.0, 0.0);
    let c2 = Rgba::<f32>::new(1.0, 1.0, 1.0, 1.0);
    let _ = Rgba::lerp(&c1, &c2, -0.1);
}

#[test]
#[should_panic(expected = "Interpolation factor")]
fn test_rgba_lerp_t_above_one() {
    let c1 = Rgba::<f32>::new(0.0, 0.0, 0.0, 0.0);
    let c2 = Rgba::<f32>::new(1.0, 1.0, 1.0, 1.0);
    let _ = Rgba::lerp(&c1, &c2, 1.1);
}

#[test]
fn test_rgba_lerp_symmetry() {
    // Test with two different colours
    let c1 = Rgba::<f32>::new(0.1, 0.2, 0.3, 0.4);
    let c2 = Rgba::<f32>::new(0.9, 0.8, 0.7, 0.6);

    // Interpolating from c1 to c2 at t=0.25 should be the same as
    // interpolating from c2 to c1 at t=0.75
    let forward = Rgba::lerp(&c1, &c2, 0.25);
    let backward = Rgba::lerp(&c2, &c1, 0.75);

    assert!((forward.red() - backward.red()).abs() < Rgba::<f32>::tolerance());
    assert!((forward.green() - backward.green()).abs() < Rgba::<f32>::tolerance());
    assert!((forward.blue() - backward.blue()).abs() < Rgba::<f32>::tolerance());
    assert!((forward.alpha() - backward.alpha()).abs() < Rgba::<f32>::tolerance());
}
