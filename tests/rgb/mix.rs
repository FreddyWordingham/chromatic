use chromatic::{Colour, Rgb};

#[test]
fn test_rgb_mix_basic() {
    let black = Rgb::<f32>::new(0.0, 0.0, 0.0);
    let mid_gray = Rgb::<f32>::new(0.5, 0.5, 0.5);
    let white = Rgb::<f32>::new(1.0, 1.0, 1.0);

    let colours = [black, mid_gray, white];
    let weights = [1.0, 1.0, 1.0]; // Equal weights

    // With equal weights, should be average of the values (0.5, 0.5, 0.5)
    let result = Rgb::<f32>::mix(&colours, &weights);
    assert_eq!(result.red(), 0.5);
    assert_eq!(result.green(), 0.5);
    assert_eq!(result.blue(), 0.5);
}

#[test]
fn test_rgb_mix_weighted() {
    let black = Rgb::<f64>::new(0.0, 0.0, 0.0);
    let white = Rgb::<f64>::new(1.0, 1.0, 1.0);

    let colours = [black, white];

    // With 1:3 weighting, should be 75% white (0.75, 0.75, 0.75)
    let weights = [1.0, 3.0];
    let result = Rgb::<f64>::mix(&colours, &weights);
    assert!((result.red() - 0.75).abs() < Rgb::<f64>::tolerance());
    assert!((result.green() - 0.75).abs() < Rgb::<f64>::tolerance());
    assert!((result.blue() - 0.75).abs() < Rgb::<f64>::tolerance());

    // With 3:1 weighting, should be 25% white (0.25, 0.25, 0.25)
    let weights = [3.0, 1.0];
    let result = Rgb::<f64>::mix(&colours, &weights);
    assert!((result.red() - 0.25).abs() < Rgb::<f64>::tolerance());
    assert!((result.green() - 0.25).abs() < Rgb::<f64>::tolerance());
    assert!((result.blue() - 0.25).abs() < Rgb::<f64>::tolerance());
}

#[test]
fn test_rgb_mix_multiple_values() {
    let colours = [
        Rgb::<f64>::new(0.1, 0.2, 0.3),
        Rgb::<f64>::new(0.3, 0.4, 0.5),
        Rgb::<f64>::new(0.5, 0.6, 0.7),
        Rgb::<f64>::new(0.9, 0.8, 0.7),
    ];

    // Equal weights
    let weights = [1.0, 1.0, 1.0, 1.0];
    let result = Rgb::<f64>::mix(&colours, &weights);

    let expected_red = 0.45;
    let expected_green = 0.5;
    let expected_blue = 0.55;
    assert!((result.red() - expected_red).abs() < Rgb::<f64>::tolerance());
    assert!((result.green() - expected_green).abs() < Rgb::<f64>::tolerance());
    assert!((result.blue() - expected_blue).abs() < Rgb::<f64>::tolerance());
}

#[test]
fn test_rgb_mix_with_zero_weights() {
    let colours = [
        Rgb::<f32>::new(0.2, 0.3, 0.4),
        Rgb::<f32>::new(0.4, 0.5, 0.6),
        Rgb::<f32>::new(0.8, 0.7, 0.6),
    ];

    // Only the first colour has non-zero weight
    let weights = [1.0, 0.0, 0.0];
    let result = Rgb::<f32>::mix(&colours, &weights);

    // Result should be equal to the first colour
    assert_eq!(result, colours[0]);
}

#[test]
#[should_panic(expected = "Cannot mix an empty list of colours")]
fn test_rgb_mix_empty_list() {
    let empty_colours: [Rgb<f32>; 0] = [];
    let empty_weights: [f32; 0] = [];

    // Should panic with empty lists
    let _ = Rgb::<f32>::mix(&empty_colours, &empty_weights);
}

#[test]
#[should_panic(expected = "Colours and weights must have the same length")]
fn test_rgb_mix_mismatched_lengths() {
    let colours = [Rgb::<f32>::new(0.1, 0.2, 0.3), Rgb::<f32>::new(0.5, 0.6, 0.7)];

    let weights = [1.0, 2.0, 3.0]; // Extra weight

    // Should panic due to mismatched lengths
    let _ = Rgb::<f32>::mix(&colours, &weights);
}

#[test]
#[should_panic(expected = "Weights must be non-negative")]
fn test_rgb_mix_negative_weights() {
    let colours = [Rgb::<f32>::new(0.3, 0.4, 0.5), Rgb::<f32>::new(0.7, 0.6, 0.5)];

    let weights = [1.0, -0.5]; // Negative weight

    // Should panic due to negative weight
    let _ = Rgb::<f32>::mix(&colours, &weights);
}

#[test]
fn test_rgb_mix_single_colour() {
    let colours = [Rgb::<f64>::new(0.42, 0.84, 0.33)];
    let weights = [3.14]; // Arbitrary positive weight

    let result = Rgb::<f64>::mix(&colours, &weights);

    // With a single colour, the result should equal that colour
    assert_eq!(result, colours[0]);
}

#[test]
fn test_rgb_mix_very_small_weights() {
    let colours = [Rgb::<f64>::new(0.2, 0.3, 0.4), Rgb::<f64>::new(0.8, 0.7, 0.6)];

    // Very small weights, but still valid
    let weights = [1e-10, 1e-10];

    // Should work with very small weights
    let result = Rgb::<f64>::mix(&colours, &weights);

    // With equal weights, result should be halfway between
    assert!((result.red() - 0.5).abs() < Rgb::<f64>::tolerance());
    assert!((result.green() - 0.5).abs() < Rgb::<f64>::tolerance());
    assert!((result.blue() - 0.5).abs() < Rgb::<f64>::tolerance());
}

#[test]
fn test_rgb_mix_primary_colours() {
    // Test mixing primary colours
    let red = Rgb::<f32>::new(1.0, 0.0, 0.0);
    let green = Rgb::<f32>::new(0.0, 1.0, 0.0);
    let blue = Rgb::<f32>::new(0.0, 0.0, 1.0);

    let colours = [red, green, blue];
    let weights = [1.0, 1.0, 1.0];

    let result = Rgb::<f32>::mix(&colours, &weights);

    // Equal mix of RGB primaries should produce grey
    assert!((result.red() - 1.0 / 3.0).abs() < Rgb::<f32>::tolerance());
    assert!((result.green() - 1.0 / 3.0).abs() < Rgb::<f32>::tolerance());
    assert!((result.blue() - 1.0 / 3.0).abs() < Rgb::<f32>::tolerance());

    // Test mixing to create secondary colours
    let colours = [red, green];
    let weights = [1.0, 1.0];

    let result = Rgb::<f32>::mix(&colours, &weights);

    // Equal mix of red and green should produce yellow
    assert!((result.red() - 0.5).abs() < Rgb::<f32>::tolerance());
    assert!((result.green() - 0.5).abs() < Rgb::<f32>::tolerance());
    assert!((result.blue() - 0.0).abs() < Rgb::<f32>::tolerance());
}
