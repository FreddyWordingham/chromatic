use chromatic::{ColourMap, GreyAlpha};

#[test]
fn test_sample_grey_alpha() {
    let colours = vec![GreyAlpha::new(0.0, 1.0), GreyAlpha::new(1.0, 0.0)];
    let positions = vec![0.0, 1.0];
    let colour_map = ColourMap::new(&colours, &positions);

    // Sample at the exact positions
    assert_eq!(colour_map.sample(0.0), GreyAlpha::new(0.0, 1.0));
    assert_eq!(colour_map.sample(1.0), GreyAlpha::new(1.0, 0.0));

    // Sample in between
    assert_eq!(colour_map.sample(0.5), GreyAlpha::new(0.5, 0.5));
    assert_eq!(colour_map.sample(0.25), GreyAlpha::new(0.25, 0.75));
    assert_eq!(colour_map.sample(0.75), GreyAlpha::new(0.75, 0.25));
}
