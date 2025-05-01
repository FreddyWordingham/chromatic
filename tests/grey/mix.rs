use chromatic::Grey;

// Tests for the mix_fold method.
#[test]
fn test_grey_mix_fold_basic() {
    let grey1 = Grey::<f32>::new(0.0); // Black
    let grey2 = Grey::<f32>::new(0.5); // Mid-grey
    let grey3 = Grey::<f32>::new(1.0); // White

    let colours = [grey1, grey2, grey3];
    let weights = [1.0, 1.0, 1.0]; // Equal weights

    // With equal weights, should be average of the values (0.5)
    let result = Grey::<f32>::mix_fold(&colours, &weights);
    assert_eq!(result.grey(), 0.5);
}

#[test]
fn test_grey_mix_fold_weighted() {
    let grey1 = Grey::<f64>::new(0.0); // Black
    let grey2 = Grey::<f64>::new(1.0); // White

    let colours = [grey1, grey2];

    // With 1:3 weighting, should be 75% white (0.75)
    let weights = [1.0, 3.0];
    let result = Grey::<f64>::mix_fold(&colours, &weights);
    assert!((result.grey() - 0.75).abs() < Grey::<f64>::tolerance());

    // With 3:1 weighting, should be 25% white (0.25)
    let weights = [3.0, 1.0];
    let result = Grey::<f64>::mix_fold(&colours, &weights);
    assert!((result.grey() - 0.25).abs() < Grey::<f64>::tolerance());
}

#[test]
fn test_grey_mix_fold_multiple_values() {
    let colours = [
        Grey::<f64>::new(0.1),
        Grey::<f64>::new(0.3),
        Grey::<f64>::new(0.5),
        Grey::<f64>::new(0.9),
    ];

    // Equal weights
    let weights = [1.0, 1.0, 1.0, 1.0];
    let result = Grey::<f64>::mix_fold(&colours, &weights);

    let expected = 0.45;
    assert!((result.grey() - expected).abs() < Grey::<f64>::tolerance());
}

#[test]
fn test_grey_mix_fold_with_zero_weights() {
    let colours = [Grey::<f32>::new(0.2), Grey::<f32>::new(0.4), Grey::<f32>::new(0.8)];

    // Only the first color has non-zero weight
    let weights = [1.0, 0.0, 0.0];
    let result = Grey::<f32>::mix_fold(&colours, &weights);

    // Result should be equal to the first color
    assert_eq!(result, colours[0]);
}

#[test]
#[should_panic(expected = "Cannot mix an empty list of colours")]
fn test_grey_mix_fold_empty_list() {
    let empty_colours: [Grey<f32>; 0] = [];
    let empty_weights: [f32; 0] = [];

    // Should panic with empty lists
    Grey::<f32>::mix_fold(&empty_colours, &empty_weights);
}

#[test]
#[should_panic(expected = "Colours and weights must have the same length")]
fn test_grey_mix_fold_mismatched_lengths() {
    let colours = [Grey::<f32>::new(0.1), Grey::<f32>::new(0.5)];

    let weights = [1.0, 2.0, 3.0]; // Extra weight

    // Should panic due to mismatched lengths
    Grey::<f32>::mix_fold(&colours, &weights);
}

#[test]
#[should_panic(expected = "Weights must be non-negative")]
fn test_grey_mix_fold_negative_weights() {
    let colours = [Grey::<f32>::new(0.3), Grey::<f32>::new(0.7)];

    let weights = [1.0, -0.5]; // Negative weight

    // Should panic due to negative weight
    Grey::<f32>::mix_fold(&colours, &weights);
}

#[test]
fn test_grey_mix_fold_single_colour() {
    let colours = [Grey::<f64>::new(0.42)];
    let weights = [3.14]; // Arbitrary positive weight

    let result = Grey::<f64>::mix_fold(&colours, &weights);

    // With a single color, the result should equal that color
    assert_eq!(result, colours[0]);
}

#[test]
fn test_grey_mix_fold_very_small_weights() {
    let colours = [Grey::<f64>::new(0.2), Grey::<f64>::new(0.8)];

    // Very small weights, but still valid
    let weights = [1e-10, 1e-10];

    // Should work with very small weights
    let result = Grey::<f64>::mix_fold(&colours, &weights);

    // With equal weights, result should be halfway between
    assert!((result.grey() - 0.5).abs() < Grey::<f64>::tolerance());
}
