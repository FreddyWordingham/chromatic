use chromatic::{ColourMap, Grey};

#[test]
fn test_sample_grey() {
    let colours = vec![Grey::new(0.0), Grey::new(1.0)];
    let positions = vec![0.0, 1.0];
    let colour_map = ColourMap::new(&colours, &positions);

    // Sample at the exact positions
    assert_eq!(colour_map.sample(0.0), Grey::new(0.0));
    assert_eq!(colour_map.sample(1.0), Grey::new(1.0));

    // Sample in between
    assert_eq!(colour_map.sample(0.5), Grey::new(0.5));
    assert_eq!(colour_map.sample(0.25), Grey::new(0.25));
    assert_eq!(colour_map.sample(0.75), Grey::new(0.75));
}

#[test]
fn test_sample_multi_segment() {
    let colours = vec![Grey::new(0.0), Grey::new(0.5), Grey::new(1.0)];
    let positions = vec![0.0, 0.6, 1.0];
    let colour_map = ColourMap::new(&colours, &positions);

    // Sample at segment boundaries
    assert_eq!(colour_map.sample(0.0), Grey::new(0.0));
    assert_eq!(colour_map.sample(0.6), Grey::new(0.5));
    assert_eq!(colour_map.sample(1.0), Grey::new(1.0));

    // Sample within first segment (0.0 to 0.6)
    assert_eq!(colour_map.sample(0.3), Grey::new(0.25)); // 50% of the way through first segment

    // Sample within second segment (0.6 to 1.0)
    assert_eq!(colour_map.sample(0.8), Grey::new(0.75)); // 50% of the way through second segment
}

#[test]
#[should_panic(expected = "Sample position must be in range [0, 1]")]
fn test_sample_out_of_range_low() {
    let colours = vec![Grey::new(0.0), Grey::new(1.0)];
    let positions = vec![0.0, 1.0];
    let colour_map = ColourMap::new(&colours, &positions);
    let _sample = colour_map.sample(-0.1);
}

#[test]
#[should_panic(expected = "Sample position must be in range [0, 1]")]
fn test_sample_out_of_range_high() {
    let colours = vec![Grey::new(0.0), Grey::new(1.0)];
    let positions = vec![0.0, 1.0];
    let colour_map = ColourMap::new(&colours, &positions);
    let _sample = colour_map.sample(1.1);
}

#[test]
fn test_uneven_distribution() {
    let colours = vec![Grey::new(0.0), Grey::new(0.5), Grey::new(1.0)];
    let positions = vec![0.0, 0.25, 1.0];
    let colour_map = ColourMap::new(&colours, &positions);

    // First segment is from 0.0 to 0.25
    assert_eq!(colour_map.sample(0.125), Grey::new(0.25)); // Midpoint of first segment

    // Second segment is from 0.25 to 1.0
    assert_eq!(colour_map.sample(0.625), Grey::new(0.75)); // Midpoint of second segment
}

#[test]
fn test_is_empty() {
    let colours = vec![Grey::new(0.0), Grey::new(1.0)];
    let positions = vec![0.0, 1.0];
    let colour_map = ColourMap::new(&colours, &positions);

    assert!(!colour_map.is_empty());
}

#[test]
fn test_with_different_numeric_types() {
    // Test with f32
    let colours_f32 = vec![Grey::<f32>::new(0.0), Grey::<f32>::new(1.0)];
    let positions_f32 = vec![0.0, 1.0];
    let colour_map_f32 = ColourMap::new(&colours_f32, &positions_f32);
    assert_eq!(colour_map_f32.sample(0.5), Grey::<f32>::new(0.5));

    // Test with f64
    let colours_f64 = vec![Grey::<f64>::new(0.0), Grey::<f64>::new(1.0)];
    let positions_f64 = vec![0.0, 1.0];
    let colour_map_f64 = ColourMap::new(&colours_f64, &positions_f64);
    assert_eq!(colour_map_f64.sample(0.5), Grey::<f64>::new(0.5));
}
