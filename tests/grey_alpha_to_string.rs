use chromatic::GreyAlpha;

// #[test]
// fn test_grey_alpha_to_string() {
//     // Test basic grey values
//     let black = GreyAlpha::new(0.0, 1.0);
//     let white = GreyAlpha::new(1.0, 1.0);
//     let mid_grey = GreyAlpha::new(0.5, 1.0);

//     assert_eq!(black.to_string(), "#00FF");
//     assert_eq!(white.to_string(), "#FFFF");
//     assert_eq!(mid_grey.to_string(), "#80FF");

//     // Test with exact value that should produce "#80"
//     let expected_mid_grey = 128.0 / 255.0;
//     let expected_mid_alpha = 128.0 / 255.0;
//     let mid_grey_exact = GreyAlpha::new(expected_mid_grey, expected_mid_alpha);
//     assert_eq!(mid_grey_exact.to_string(), "#8080");
// }
