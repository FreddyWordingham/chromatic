use chromatic::{ColourMap, Grey, GreyAlpha};

#[test]
fn test_new_colour_map() {
    // Test with Grey
    let colours = vec![Grey::new(0.0), Grey::new(0.5), Grey::new(1.0)];
    let positions = vec![0.0, 0.5, 1.0];
    let colour_map = ColourMap::new(&colours, &positions);

    assert_eq!(colour_map.len(), 3);
    assert_eq!(colour_map.colours(), &colours);
    assert_eq!(colour_map.positions(), &positions);

    // Test with GreyAlpha
    let colours = vec![GreyAlpha::new(0.0, 1.0), GreyAlpha::new(0.5, 0.5), GreyAlpha::new(1.0, 0.0)];
    let positions = vec![0.0, 0.5, 1.0];
    let colour_map = ColourMap::new(&colours, &positions);

    assert_eq!(colour_map.len(), 3);
    assert_eq!(colour_map.colours(), &colours);
    assert_eq!(colour_map.positions(), &positions);
}

#[test]
#[should_panic(expected = "Colour map must have at least one colour")]
fn test_empty_colour_map() {
    let colours: Vec<Grey<f32>> = vec![];
    let positions: Vec<f32> = vec![];
    let _ = ColourMap::new(&colours, &positions);
}

#[test]
#[should_panic(expected = "Colour map must have the same number of colours and positions")]
fn test_mismatched_length() {
    let colours = vec![Grey::new(0.0), Grey::new(1.0)];
    let positions = vec![0.0, 0.5, 1.0];
    let _ = ColourMap::new(&colours, &positions);
}

#[test]
#[should_panic(expected = "Positions must be in range [0, 1]")]
fn test_position_out_of_range_low() {
    let colours = vec![Grey::new(0.0), Grey::new(1.0)];
    let positions = vec![-0.1, 1.0];
    let _ = ColourMap::new(&colours, &positions);
}

#[test]
#[should_panic(expected = "Positions must be in range [0, 1]")]
fn test_position_out_of_range_high() {
    let colours = vec![Grey::new(0.0), Grey::new(1.0)];
    let positions = vec![0.0, 1.1];
    let _ = ColourMap::new(&colours, &positions);
}

#[test]
#[should_panic(expected = "Positions must be in ascending order")]
fn test_positions_not_ascending() {
    let colours = vec![Grey::new(0.0), Grey::new(0.5), Grey::new(1.0)];
    let positions = vec![0.0, 0.7, 0.5];
    let _ = ColourMap::new(&colours, &positions);
}
