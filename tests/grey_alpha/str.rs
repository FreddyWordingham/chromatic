use chromatic::GreyAlpha;
use core::str::FromStr;

// Tests for Display implementation.
#[test]
fn test_grey_alpha_display() {
    // Test formatting to hex strings
    let grey_alpha = GreyAlpha::<f32>::new(0.0, 0.0);
    assert_eq!(format!("{}", grey_alpha), "#00");

    let grey_alpha = GreyAlpha::<f32>::new(1.0, 1.0);
    assert_eq!(format!("{}", grey_alpha), "#FF");

    let grey_alpha = GreyAlpha::<f32>::new(8.0 / 15.0, 12.0 / 15.0);
    assert_eq!(format!("{}", grey_alpha), "#8C");

    // Test mixed values
    let grey_alpha = GreyAlpha::<f32>::new(0.0, 1.0);
    assert_eq!(format!("{}", grey_alpha), "#0F");

    let grey_alpha = GreyAlpha::<f32>::new(1.0, 0.0);
    assert_eq!(format!("{}", grey_alpha), "#F0");
}

// Test parsing from string (FromStr implementation).
#[test]
fn test_grey_alpha_from_str() {
    // Test parsing hex strings
    let grey_alpha = GreyAlpha::<f32>::from_str("#00").unwrap();
    assert_eq!(grey_alpha.grey(), 0.0);
    assert_eq!(grey_alpha.alpha(), 0.0);

    let grey_alpha = GreyAlpha::<f32>::from_str("#FF").unwrap();
    assert_eq!(grey_alpha.grey(), 1.0);
    assert_eq!(grey_alpha.alpha(), 1.0);

    let grey_alpha = GreyAlpha::<f32>::from_str("#8C").unwrap();
    assert!((grey_alpha.grey() - 8.0 / 15.0).abs() < GreyAlpha::<f32>::tolerance());
    assert!((grey_alpha.alpha() - 12.0 / 15.0).abs() < GreyAlpha::<f32>::tolerance());

    // Test lowercase hex
    let grey_alpha = GreyAlpha::<f32>::from_str("#3a").unwrap();
    assert!((grey_alpha.grey() - 3.0 / 15.0).abs() < GreyAlpha::<f32>::tolerance());
    assert!((grey_alpha.alpha() - 10.0 / 15.0).abs() < GreyAlpha::<f32>::tolerance());

    // Test parsing with whitespace
    let grey_alpha = GreyAlpha::<f32>::from_str(" #5B ").unwrap();
    assert!((grey_alpha.grey() - 5.0 / 15.0).abs() < GreyAlpha::<f32>::tolerance());
    assert!((grey_alpha.alpha() - 11.0 / 15.0).abs() < GreyAlpha::<f32>::tolerance());

    // Test parsing float values
    let grey_alpha = GreyAlpha::<f32>::from_str("0.2, 0.8").unwrap();
    assert!((grey_alpha.grey() - 0.2).abs() < GreyAlpha::<f32>::tolerance());
    assert!((grey_alpha.alpha() - 0.8).abs() < GreyAlpha::<f32>::tolerance());

    // Test parsing float values with whitespace
    let grey_alpha = GreyAlpha::<f32>::from_str(" 0.4 , 0.6 ").unwrap();
    assert!((grey_alpha.grey() - 0.4).abs() < GreyAlpha::<f32>::tolerance());
    assert!((grey_alpha.alpha() - 0.6).abs() < GreyAlpha::<f32>::tolerance());
}

// Test consistency between FromStr and Display (round-trip tests).
#[test]
fn test_grey_alpha_round_trip() {
    // Display -> FromStr should result in the same value for hex values
    for g in 0..16 {
        for a in 0..16 {
            let hex_str = format!("#{:X}{:X}", g, a);
            let grey_alpha = GreyAlpha::<f32>::from_str(&hex_str).unwrap();
            let formatted = format!("{}", grey_alpha);

            assert_eq!(formatted.to_uppercase(), hex_str.to_uppercase());
        }
    }
}

// Tests for error handling and error types.
#[test]
fn test_parse_grey_alpha_error_types() {
    // Test ParseFloat error
    let result = GreyAlpha::<f32>::from_str("not_a_number, 0.5");
    match result {
        Err(chromatic::ParseGreyAlphaError::ParseFloat(_)) => { /* Expected */ }
        _ => panic!("Expected ParseFloat error"),
    }

    // Test ParseHex error
    let result = GreyAlpha::<f32>::from_str("#ZZ");
    match result {
        Err(chromatic::ParseGreyAlphaError::ParseHex(_)) => { /* Expected */ }
        _ => panic!("Expected ParseHex error"),
    }

    // Test InvalidFormat error (too few or too many characters in hex)
    let result = GreyAlpha::<f32>::from_str("#F");
    match result {
        Err(chromatic::ParseGreyAlphaError::InvalidFormat) => { /* Expected */ }
        _ => panic!("Expected InvalidFormat error"),
    }

    let result = GreyAlpha::<f32>::from_str("#FFF");
    match result {
        Err(chromatic::ParseGreyAlphaError::InvalidFormat) => { /* Expected */ }
        _ => panic!("Expected InvalidFormat error"),
    }

    // Test InvalidFormat error (not enough comma-separated values)
    let result = GreyAlpha::<f32>::from_str("0.5");
    match result {
        Err(chromatic::ParseGreyAlphaError::InvalidFormat) => { /* Expected */ }
        _ => panic!("Expected InvalidFormat error"),
    }

    let result = GreyAlpha::<f32>::from_str("0.1, 0.2, 0.3");
    match result {
        Err(chromatic::ParseGreyAlphaError::InvalidFormat) => { /* Expected */ }
        _ => panic!("Expected InvalidFormat error"),
    }

    // Make sure the error type implements Debug
    let result = GreyAlpha::<f32>::from_str("invalid");
    if let Err(e) = result {
        let _debug_str = format!("{:?}", e);
        // Just check that this doesn't panic
    }
}

// Tests for additional edge cases in string parsing.
#[test]
fn test_grey_alpha_from_str_edge_cases() {
    // Test parsing exact upper/lower bounds
    let grey_alpha = GreyAlpha::<f32>::from_str("0.0, 0.0").unwrap();
    assert_eq!(grey_alpha.grey(), 0.0);
    assert_eq!(grey_alpha.alpha(), 0.0);

    let grey_alpha = GreyAlpha::<f32>::from_str("1.0, 1.0").unwrap();
    assert_eq!(grey_alpha.grey(), 1.0);
    assert_eq!(grey_alpha.alpha(), 1.0);

    // Test mixed boundary values
    let grey_alpha = GreyAlpha::<f32>::from_str("0.0, 1.0").unwrap();
    assert_eq!(grey_alpha.grey(), 0.0);
    assert_eq!(grey_alpha.alpha(), 1.0);

    let grey_alpha = GreyAlpha::<f32>::from_str("1.0, 0.0").unwrap();
    assert_eq!(grey_alpha.grey(), 1.0);
    assert_eq!(grey_alpha.alpha(), 0.0);
}
