// use approx::assert_approx_eq;
// use chromatic::{ColourMap, Rgb};

// // Helper function to create a test gradient
// fn create_test_gradient() -> ColourMap<Rgb<f32>, f32> {
//     let red = Rgb::new(1.0, 0.0, 0.0);
//     let green = Rgb::new(0.0, 1.0, 0.0);
//     let blue = Rgb::new(0.0, 0.0, 1.0);

//     let colours = [red, green, blue];
//     let positions = [0.0, 0.5, 1.0];

//     ColourMap::new(&colours, &positions)
// }

// #[test]
// fn test_new() {
//     let map = create_test_gradient();

//     // Test that we can at least create the map
//     assert!(matches!(map.inner, ColourMapInner::Gradient(_)));
// }

// #[test]
// fn test_new_uniform() {
//     let red = Rgb::new(1.0, 0.0, 0.0);
//     let green = Rgb::new(0.0, 1.0, 0.0);
//     let blue = Rgb::new(0.0, 0.0, 1.0);

//     let colours = [red, green, blue];

//     let map = ColourMap::new_uniform(&colours);

//     // Test that we can create a uniform map
//     assert!(matches!(map.inner, ColourMapInner::Gradient(_)));

//     // Test that uniform positions are correctly distributed
//     let sample_start = map.sample(0.0);
//     let sample_mid = map.sample(0.5);
//     let sample_end = map.sample(1.0);

//     assert_approx_eq!(sample_start.r, 1.0);
//     assert_approx_eq!(sample_start.g, 0.0);
//     assert_approx_eq!(sample_start.b, 0.0);

//     // Middle should be close to green
//     assert_approx_eq!(sample_mid.r, 0.0);
//     assert_approx_eq!(sample_mid.g, 1.0);
//     assert_approx_eq!(sample_mid.b, 0.0);

//     assert_approx_eq!(sample_end.r, 0.0);
//     assert_approx_eq!(sample_end.g, 0.0);
//     assert_approx_eq!(sample_end.b, 1.0);
// }

// #[test]
// fn test_single_color() {
//     let white = Rgb::new(1.0, 1.0, 1.0);
//     let map = ColourMap::new(&[white], &[0.5]);

//     // Single color map should return the same color regardless of position
//     assert_eq!(map.sample(0.0), white);
//     assert_eq!(map.sample(0.3), white);
//     assert_eq!(map.sample(0.5), white);
//     assert_eq!(map.sample(0.7), white);
//     assert_eq!(map.sample(1.0), white);
// }

// #[test]
// fn test_gradient_sampling() {
//     let map = create_test_gradient();

//     // Test sampling at the defined positions
//     let sample_start = map.sample(0.0);
//     let sample_mid = map.sample(0.5);
//     let sample_end = map.sample(1.0);

//     // Start should be red
//     assert_approx_eq!(sample_start.r, 1.0);
//     assert_approx_eq!(sample_start.g, 0.0);
//     assert_approx_eq!(sample_start.b, 0.0);

//     // Middle should be green
//     assert_approx_eq!(sample_mid.r, 0.0);
//     assert_approx_eq!(sample_mid.g, 1.0);
//     assert_approx_eq!(sample_mid.b, 0.0);

//     // End should be blue
//     assert_approx_eq!(sample_end.r, 0.0);
//     assert_approx_eq!(sample_end.g, 0.0);
//     assert_approx_eq!(sample_end.b, 1.0);

//     // Test interpolated values
//     let sample_quarter = map.sample(0.25);

//     // Quarter should be between red and green
//     assert!(sample_quarter.r > 0.0 && sample_quarter.r < 1.0);
//     assert!(sample_quarter.g > 0.0 && sample_quarter.g < 1.0);
//     assert_approx_eq!(sample_quarter.b, 0.0);

//     // Test value clamping
//     let sample_below = map.sample(-0.5);
//     let sample_above = map.sample(1.5);

//     // Below range should clamp to start color
//     assert_eq!(sample_below, sample_start);

//     // Above range should clamp to end color
//     assert_eq!(sample_above, sample_end);
// }

// #[test]
// fn test_insert_colour() {
//     let mut map = create_test_gradient();
//     let yellow = Rgb::new(1.0, 1.0, 0.0);

//     // Insert a new color
//     map.insert_colour(yellow, 0.75);

//     // Sample at the new position
//     let sample = map.sample(0.75);

//     // Should be yellow
//     assert_approx_eq!(sample.r, 1.0);
//     assert_approx_eq!(sample.g, 1.0);
//     assert_approx_eq!(sample.b, 0.0);

//     // Test that colors before and after are still correct
//     let sample_mid = map.sample(0.5);
//     let sample_end = map.sample(1.0);

//     // Middle should still be green
//     assert_approx_eq!(sample_mid.r, 0.0);
//     assert_approx_eq!(sample_mid.g, 1.0);
//     assert_approx_eq!(sample_mid.b, 0.0);

//     // End should still be blue
//     assert_approx_eq!(sample_end.r, 0.0);
//     assert_approx_eq!(sample_end.g, 0.0);
//     assert_approx_eq!(sample_end.b, 1.0);
// }

// #[test]
// #[should_panic(expected = "Cannot insert a colour at an existing position")]
// fn test_insert_duplicate_position() {
//     let mut map = create_test_gradient();
//     let yellow = Rgb::new(1.0, 1.0, 0.0);

//     // Try to insert at an existing position
//     map.insert_colour(yellow, 0.5);
// }

// #[test]
// #[should_panic(expected = "Position must be in range [0, 1]")]
// fn test_insert_out_of_range() {
//     let mut map = create_test_gradient();
//     let yellow = Rgb::new(1.0, 1.0, 0.0);

//     // Try to insert out of range
//     map.insert_colour(yellow, 1.5);
// }

// #[test]
// fn test_empty() {
//     let empty_map = ColourMap::<Rgb<f32>, f32>::empty();

//     // Should be a Raw map
//     assert!(matches!(empty_map.inner, ColourMapInner::Raw { .. }));
// }

// #[test]
// #[should_panic(expected = "Cannot sample an empty colour map")]
// fn test_sample_empty() {
//     let empty_map = ColourMap::<Rgb<f32>, f32>::empty();

//     // Try to sample an empty map
//     let _ = empty_map.sample(0.5);
// }
