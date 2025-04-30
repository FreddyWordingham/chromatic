use chromatic::Grey;

#[test]
fn test_grey_to_string() {
    // Test basic grey values
    let black = Grey::new(0.0);
    let white = Grey::new(1.0);
    let mid_grey = Grey::new(0.5);

    assert_eq!(black.to_string(), "#00");
    assert_eq!(white.to_string(), "#FF");
    assert_eq!(mid_grey.to_string(), "#80");

    // Test with exact value that should produce "#80"
    let expected_mid_grey = 128.0 / 255.0;
    let mid_grey_exact = Grey::new(expected_mid_grey);
    assert_eq!(mid_grey_exact.to_string(), "#80");
}
