use chromatic::Grey;

#[test]
fn test_grey_equality() {
    let grey1 = Grey::<f32>::new(0.5);
    let grey2 = Grey::<f32>::new(0.5);
    let grey3 = Grey::<f32>::new(0.501);
    let grey4 = Grey::<f32>::new(0.499);

    // Exact equality
    assert_eq!(grey1, grey2);

    // Using the tolerance defined in the Grey impl (1/256)
    assert_eq!(grey1, grey3);
    assert_eq!(grey1, grey4);

    // Beyond tolerance range
    let out_of_range = Grey::<f32>::new(0.5 + 1.0 / 128.0);
    assert_ne!(grey1, out_of_range);
}

// Test edge cases around the tolerance boundary.
#[test]
fn test_grey_difference_boundary() {
    let tolerance = Grey::<f64>::tolerance();

    let base = Grey::<f64>::new(0.5);
    let just_within = Grey::<f64>::new(0.5 + tolerance * 0.99);
    let just_outside = Grey::<f64>::new(0.5 + tolerance * 1.01);

    assert_eq!(base, just_within);
    assert_ne!(base, just_outside);
}

// Test equality at the boundaries.
#[test]
fn test_grey_boundary_values() {
    let zero1 = Grey::<f32>::new(0.0);
    let zero2 = Grey::<f32>::new(0.0);
    let almost_zero = Grey::<f32>::new(1.0 / 512.0);

    let one1 = Grey::<f32>::new(1.0);
    let one2 = Grey::<f32>::new(1.0);
    let almost_one = Grey::<f32>::new(1.0 - 1.0 / 512.0);

    assert_eq!(zero1, zero2);
    assert_eq!(one1, one2);

    assert_eq!(zero1, almost_zero);
    assert_eq!(one1, almost_one);
}

// Test transitivity of equality.
#[test]
fn test_grey_transitivity() {
    let tolerance = Grey::<f64>::tolerance();

    let g1 = Grey::<f64>::new(0.5);
    let g2 = Grey::<f64>::new(0.5 + tolerance * 0.5);
    let g3 = Grey::<f64>::new(0.5 + tolerance * 0.9);

    assert_eq!(g1, g2);
    assert_eq!(g2, g3);
    assert_eq!(g1, g3);
}
