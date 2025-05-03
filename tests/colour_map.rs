use assert_approx_eq::assert_approx_eq;
use chromatic::{ColourMap, Grey, GreyAlpha, LabRgb, LabRgba, Rgb, Rgba};

// Helper function to create an RGB test colour map
fn create_test_rgb_map() -> ColourMap<Rgb<f32>, f32, 3> {
    let red = Rgb::new(1.0, 0.0, 0.0);
    let green = Rgb::new(0.0, 1.0, 0.0);
    let blue = Rgb::new(0.0, 0.0, 1.0);

    let colours = vec![red, green, blue];
    let positions = vec![0.0, 0.5, 1.0];

    ColourMap::new(&colours, &positions)
}

#[test]
fn test_new_colour_map() {
    let cmap = create_test_rgb_map();

    assert_eq!(cmap.len(), 3);
    assert_eq!(cmap.colours().len(), 3);
    assert_eq!(cmap.positions().len(), 3);

    assert_eq!(cmap.positions()[0], 0.0);
    assert_eq!(cmap.positions()[1], 0.5);
    assert_eq!(cmap.positions()[2], 1.0);
}

#[test]
fn test_new_uniform_colour_map() {
    let red = Rgb::new(1.0, 0.0, 0.0);
    let green = Rgb::new(0.0, 1.0, 0.0);
    let blue = Rgb::new(0.0, 0.0, 1.0);

    let colours = vec![red, green, blue];
    let cmap = ColourMap::new_uniform(&colours);

    assert_eq!(cmap.len(), 3);
    assert_eq!(cmap.positions()[0], 0.0);
    assert_eq!(cmap.positions()[1], 0.5);
    assert_eq!(cmap.positions()[2], 1.0);
}

#[test]
fn test_single_colour_map() {
    let red = Rgb::new(1.0, 0.0, 0.0);
    let colours = vec![red];

    let cmap = ColourMap::new_uniform(&colours);

    assert_eq!(cmap.len(), 1);
    assert_eq!(cmap.positions().len(), 1);
    assert_eq!(cmap.positions()[0], 0.0);

    // Sampling should always return the same colour
    let sample = cmap.sample(0.5);
    assert_approx_eq!(sample.red(), 1.0_f64);
    assert_approx_eq!(sample.green(), 0.0);
    assert_approx_eq!(sample.blue(), 0.0);
}

#[test]
fn test_sample_at_control_points() {
    let cmap = create_test_rgb_map();

    // Sample at exact control points
    let sample1 = cmap.sample(0.0);
    assert_approx_eq!(sample1.red(), 1.0);
    assert_approx_eq!(sample1.green(), 0.0);
    assert_approx_eq!(sample1.blue(), 0.0);

    let sample2 = cmap.sample(0.5);
    assert_approx_eq!(sample2.red(), 0.0);
    assert_approx_eq!(sample2.green(), 1.0);
    assert_approx_eq!(sample2.blue(), 0.0);

    let sample3 = cmap.sample(1.0);
    assert_approx_eq!(sample3.red(), 0.0);
    assert_approx_eq!(sample3.green(), 0.0);
    assert_approx_eq!(sample3.blue(), 1.0);
}

#[test]
fn test_sample_between_control_points() {
    let cmap = create_test_rgb_map();

    // Sample between red and green (25% of the way)
    let sample1 = cmap.sample(0.25);
    assert_approx_eq!(sample1.red(), 0.5);
    assert_approx_eq!(sample1.green(), 0.5);
    assert_approx_eq!(sample1.blue(), 0.0);

    // Sample between green and blue (75% of the way)
    let sample2 = cmap.sample(0.75);
    assert_approx_eq!(sample2.red(), 0.0);
    assert_approx_eq!(sample2.green(), 0.5);
    assert_approx_eq!(sample2.blue(), 0.5);
}

#[test]
fn test_sample_edge_cases() {
    let cmap = create_test_rgb_map();

    // Sample at exact boundaries
    let sample1 = cmap.sample(0.0);
    assert_eq!(sample1, cmap.colours()[0]);

    let sample2 = cmap.sample(1.0);
    assert_eq!(sample2, cmap.colours()[2]);
}

#[test]
#[should_panic(expected = "Sample position must be in range [0, 1].")]
fn test_sample_out_of_range_below() {
    let cmap = create_test_rgb_map();
    let _ = cmap.sample(-0.1);
}

#[test]
#[should_panic(expected = "Sample position must be in range [0, 1].")]
fn test_sample_out_of_range_above() {
    let cmap = create_test_rgb_map();
    let _ = cmap.sample(1.1);
}

#[test]
#[should_panic(expected = "Positions must be in range [0, 1].")]
fn test_invalid_positions_below_range() {
    let red = Rgb::new(1.0, 0.0, 0.0);
    let green = Rgb::new(0.0, 1.0, 0.0);

    let colours = vec![red, green];
    let positions = vec![-0.1, 0.5]; // Invalid: below 0.0

    let _ = ColourMap::new(&colours, &positions);
}

#[test]
#[should_panic(expected = "Positions must be in range [0, 1].")]
fn test_invalid_positions_above_range() {
    let red = Rgb::new(1.0, 0.0, 0.0);
    let green = Rgb::new(0.0, 1.0, 0.0);

    let colours = vec![red, green];
    let positions = vec![0.0, 1.1]; // Invalid: above 1.0

    let _ = ColourMap::new(&colours, &positions);
}

#[test]
#[should_panic(expected = "Positions must be in ascending order.")]
fn test_invalid_positions_not_ascending() {
    let red = Rgb::new(1.0, 0.0, 0.0);
    let green = Rgb::new(0.0, 1.0, 0.0);
    let blue = Rgb::new(0.0, 0.0, 1.0);

    let colours = vec![red, green, blue];
    let positions = vec![0.0, 0.8, 0.5]; // Invalid: not ascending

    let _ = ColourMap::new(&colours, &positions);
}

#[test]
#[should_panic(expected = "Colour map must have the same number of colours and positions.")]
fn test_mismatched_lengths() {
    let red = Rgb::new(1.0, 0.0, 0.0);
    let green = Rgb::new(0.0, 1.0, 0.0);
    let blue = Rgb::new(0.0, 0.0, 1.0);

    let colours = vec![red, green, blue];
    let positions = vec![0.0, 1.0]; // Too few positions

    let _ = ColourMap::new(&colours, &positions);
}

#[test]
#[should_panic(expected = "Colour map must have at least one colour.")]
fn test_empty_colour_map() {
    let colours: Vec<Rgb<f32>> = Vec::new();
    let positions: Vec<f32> = Vec::new();

    let _ = ColourMap::new(&colours, &positions);
}

#[test]
#[should_panic(expected = "Colour map must have at least one colour.")]
fn test_empty_uniform_colour_map() {
    let colours: Vec<Rgb<f32>> = Vec::new();
    let _ = ColourMap::new_uniform(&colours);
}

#[test]
fn test_different_colour_types() {
    // Test with Grey
    let black = Grey::new(0.0);
    let white = Grey::new(1.0);
    let colours = vec![black, white];
    let positions = vec![0.0, 1.0];

    let cmap = ColourMap::new(&colours, &positions);
    let grey50 = cmap.sample(0.5);
    assert_approx_eq!(grey50.grey(), 0.5_f64);

    // Test with GreyAlpha
    let transparent_black = GreyAlpha::new(0.0, 0.0);
    let opaque_white = GreyAlpha::new(1.0, 1.0);
    let colours = vec![transparent_black, opaque_white];
    let positions = vec![0.0, 1.0];

    let cmap = ColourMap::new(&colours, &positions);
    let grey_alpha50 = cmap.sample(0.5);
    assert_approx_eq!(grey_alpha50.grey(), 0.5_f64);
    assert_approx_eq!(grey_alpha50.alpha(), 0.5);

    // Test with Rgba
    let transparent_red = Rgba::new(1.0, 0.0, 0.0, 0.0);
    let opaque_blue = Rgba::new(0.0, 0.0, 1.0, 1.0);
    let colours = vec![transparent_red, opaque_blue];
    let positions = vec![0.0, 1.0];

    let cmap = ColourMap::new(&colours, &positions);
    let purple50 = cmap.sample(0.5);
    assert_approx_eq!(purple50.red(), 0.5_f64);
    assert_approx_eq!(purple50.green(), 0.0);
    assert_approx_eq!(purple50.blue(), 0.5);
    assert_approx_eq!(purple50.alpha(), 0.5);
}

#[test]
fn test_with_lab_colours() {
    // Create a map with LabRgb colours for perceptually uniform interpolation
    let yellow = LabRgb::new(1.0, 1.0, 0.0);
    let blue = LabRgb::new(0.0, 0.0, 1.0);

    let colours = vec![yellow, blue];
    let positions = vec![0.0, 1.0];

    let cmap = ColourMap::new(&colours, &positions);

    // Sample in the middle - this should use Lab interpolation internally
    let middle = cmap.sample(0.5);

    // We're not testing exact values since Lab interpolation is complex,
    // but we can verify that we get a valid colour back
    assert!(middle.red() >= 0.0 && middle.red() <= 1.0);
    assert!(middle.green() >= 0.0 && middle.green() <= 1.0);
    assert!(middle.blue() >= 0.0 && middle.blue() <= 1.0);

    // And similarly with LabRgba
    let transparent_yellow = LabRgba::new(1.0, 1.0, 0.0, 0.25);
    let opaque_blue = LabRgba::new(0.0, 0.0, 1.0, 1.0);

    let colours = vec![transparent_yellow, opaque_blue];
    let positions = vec![0.0, 1.0];

    let cmap = ColourMap::new(&colours, &positions);
    let middle = cmap.sample(0.5);

    assert!(middle.alpha() >= 0.25 && middle.alpha() <= 1.0);
}

#[test]
fn test_float_generic_with_f64() {
    // Test with f64 instead of f32
    let red = Rgb::<f64>::new(1.0, 0.0, 0.0);
    let blue = Rgb::<f64>::new(0.0, 0.0, 1.0);

    let colours = vec![red, blue];
    let positions = vec![0.0, 1.0];

    let cmap = ColourMap::new(&colours, &positions);
    let purple = cmap.sample(0.5);

    assert_eq!(purple.red(), 0.5);
    assert_eq!(purple.green(), 0.0);
    assert_eq!(purple.blue(), 0.5);
}
