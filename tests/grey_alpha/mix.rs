use chromatic::{Colour, GreyAlpha};

#[test]
fn test_grey_alpha_mix_basic() {
    let black_transparent = GreyAlpha::<f32>::new(0.0, 0.0);
    let mid_grey_mid_alpha = GreyAlpha::<f32>::new(0.5, 0.5);
    let white_opaque = GreyAlpha::<f32>::new(1.0, 1.0);

    let colours = [black_transparent, mid_grey_mid_alpha, white_opaque];
    let weights = [1.0, 1.0, 1.0]; // Equal weights

    // With equal weights, should be average of the values (0.5, 0.5)
    let result = GreyAlpha::<f32>::mix(&colours, &weights);
    assert_eq!(result.grey(), 0.5);
    assert_eq!(result.alpha(), 0.5);
}

#[test]
fn test_grey_alpha_mix_weighted() {
    let black_transparent = GreyAlpha::<f64>::new(0.0, 0.0);
    let white_opaque = GreyAlpha::<f64>::new(1.0, 1.0);

    let colours = [black_transparent, white_opaque];

    // With 1:3 weighting, should be 75% white, 75% opaque (0.75, 0.75)
    let weights = [1.0, 3.0];
    let result = GreyAlpha::<f64>::mix(&colours, &weights);
    assert!((result.grey() - 0.75).abs() < GreyAlpha::<f64>::tolerance());
    assert!((result.alpha() - 0.75).abs() < GreyAlpha::<f64>::tolerance());

    // With 3:1 weighting, should be 25% white, 25% opaque (0.25, 0.25)
    let weights = [3.0, 1.0];
    let result = GreyAlpha::<f64>::mix(&colours, &weights);
    assert!((result.grey() - 0.25).abs() < GreyAlpha::<f64>::tolerance());
    assert!((result.alpha() - 0.25).abs() < GreyAlpha::<f64>::tolerance());
}

#[test]
fn test_grey_alpha_mix_multiple_values() {
    let colours = [
        GreyAlpha::<f64>::new(0.1, 0.2),
        GreyAlpha::<f64>::new(0.3, 0.4),
        GreyAlpha::<f64>::new(0.5, 0.6),
        GreyAlpha::<f64>::new(0.9, 0.8),
    ];

    // Equal weights
    let weights = [1.0, 1.0, 1.0, 1.0];
    let result = GreyAlpha::<f64>::mix(&colours, &weights);

    let expected_grey = 0.45;
    let expected_alpha = 0.5;
    assert!((result.grey() - expected_grey).abs() < GreyAlpha::<f64>::tolerance());
    assert!((result.alpha() - expected_alpha).abs() < GreyAlpha::<f64>::tolerance());
}

#[test]
fn test_grey_alpha_mix_with_zero_weights() {
    let colours = [
        GreyAlpha::<f32>::new(0.2, 0.3),
        GreyAlpha::<f32>::new(0.4, 0.5),
        GreyAlpha::<f32>::new(0.8, 0.7),
    ];

    // Only the first color has non-zero weight
    let weights = [1.0, 0.0, 0.0];
    let result = GreyAlpha::<f32>::mix(&colours, &weights);

    // Result should be equal to the first color
    assert_eq!(result, colours[0]);
}

#[test]
#[should_panic(expected = "Cannot mix an empty list of colours")]
fn test_grey_alpha_mix_empty_list() {
    let empty_colours: [GreyAlpha<f32>; 0] = [];
    let empty_weights: [f32; 0] = [];

    // Should panic with empty lists
    let _ = GreyAlpha::<f32>::mix(&empty_colours, &empty_weights);
}

#[test]
#[should_panic(expected = "Colours and weights must have the same length")]
fn test_grey_alpha_mix_mismatched_lengths() {
    let colours = [GreyAlpha::<f32>::new(0.1, 0.2), GreyAlpha::<f32>::new(0.5, 0.6)];

    let weights = [1.0, 2.0, 3.0]; // Extra weight

    // Should panic due to mismatched lengths
    let _ = GreyAlpha::<f32>::mix(&colours, &weights);
}

#[test]
#[should_panic(expected = "Weights must be non-negative")]
fn test_grey_alpha_mix_negative_weights() {
    let colours = [GreyAlpha::<f32>::new(0.3, 0.4), GreyAlpha::<f32>::new(0.7, 0.6)];

    let weights = [1.0, -0.5]; // Negative weight

    // Should panic due to negative weight
    let _ = GreyAlpha::<f32>::mix(&colours, &weights);
}

#[test]
fn test_grey_alpha_mix_single_colour() {
    let colours = [GreyAlpha::<f64>::new(0.42, 0.84)];
    let weights = [3.14]; // Arbitrary positive weight

    let result = GreyAlpha::<f64>::mix(&colours, &weights);

    // With a single color, the result should equal that color
    assert_eq!(result, colours[0]);
}

#[test]
fn test_grey_alpha_mix_very_small_weights() {
    let colours = [GreyAlpha::<f64>::new(0.2, 0.3), GreyAlpha::<f64>::new(0.8, 0.7)];

    // Very small weights, but still valid
    let weights = [1e-10, 1e-10];

    // Should work with very small weights
    let result = GreyAlpha::<f64>::mix(&colours, &weights);

    // With equal weights, result should be halfway between
    assert!((result.grey() - 0.5).abs() < GreyAlpha::<f64>::tolerance());
    assert!((result.alpha() - 0.5).abs() < GreyAlpha::<f64>::tolerance());
}
