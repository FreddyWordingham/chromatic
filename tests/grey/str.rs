use chromatic::{Colour, Grey};
use core::str::FromStr;

// Tests for Display implementation.
#[test]
fn test_grey_display() {
    // Test formatting to hex strings
    let grey = Grey::<f32>::new(0.0);
    assert_eq!(format!("{}", grey), "#0");

    let grey = Grey::<f32>::new(1.0);
    assert_eq!(format!("{}", grey), "#F");

    let grey = Grey::<f32>::new(8.0 / 15.0);
    assert_eq!(format!("{}", grey), "#8");

    // Test rounding behavior
    let grey = Grey::<f32>::new(8.0 / 15.0 + 0.01);
    assert_eq!(format!("{}", grey), "#8");

    let grey = Grey::<f32>::new(7.0 / 15.0 - 0.01);
    assert_eq!(format!("{}", grey), "#7");
}

// Test consistency between FromStr and Display (round-trip tests).
#[test]
fn test_grey_round_trip() {
    // FromStr -> Display should result in the same value
    // Only test with the exact hex values that map cleanly to the output format
    for i in 0..16 {
        let value = i as f32 / 15.0;
        let grey = Grey::<f32>::new(value);
        let displayed = format!("{}", grey);
        let parsed = Grey::<f32>::from_str(&displayed).unwrap();

        assert_eq!(grey, parsed);
    }

    // Display -> FromStr should result in the same value
    for hex_char in ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'] {
        let hex_str = format!("#{}", hex_char);
        let grey = Grey::<f32>::from_str(&hex_str).unwrap();
        let formatted = format!("{}", grey);

        assert_eq!(formatted.to_uppercase(), hex_str.to_uppercase());
    }
}

// Tests for rounding behavior.
#[test]
fn test_grey_rounding_behavior() {
    // Test with values that correspond exactly to the 16 quantized levels
    let values = [
        0.0,
        1.0 / 15.0,
        2.0 / 15.0,
        3.0 / 15.0,
        4.0 / 15.0,
        5.0 / 15.0,
        6.0 / 15.0,
        7.0 / 15.0,
        8.0 / 15.0,
        9.0 / 15.0,
        10.0 / 15.0,
        11.0 / 15.0,
        12.0 / 15.0,
        13.0 / 15.0,
        14.0 / 15.0,
        1.0,
    ];

    for &value in &values {
        let grey = Grey::<f64>::new(value);
        let as_str = format!("{}", grey);
        let parsed_back = Grey::<f64>::from_str(&as_str).unwrap();

        // The displayed and parsed values should be considered equal
        assert_eq!(grey, parsed_back);
    }
}

// Tests for parsing with different floating point types.
#[test]
fn test_grey_from_str_different_types() {
    // Use an exact value that can be represented precisely
    let float_str = "0.2";

    let grey_f32 = Grey::<f32>::from_str(float_str).unwrap();
    let grey_f64 = Grey::<f64>::from_str(float_str).unwrap();

    // We can't directly compare f32 and f64 values for exact equality
    // Instead, verify they're within the tolerance
    assert!((grey_f32.grey() as f64 - grey_f64.grey()).abs() < Grey::<f64>::tolerance());

    // Test hex parsing with different types
    let hex_str = "#5";

    let grey_f32 = Grey::<f32>::from_str(hex_str).unwrap();
    let grey_f64 = Grey::<f64>::from_str(hex_str).unwrap();

    assert!((grey_f32.grey() as f64 - grey_f64.grey()).abs() < Grey::<f64>::tolerance());
}

// Tests for error handling and error types.
#[test]
fn test_parse_grey_error_types() {
    // Test ParseFloat error
    let result = Grey::<f32>::from_str("not_a_number");
    match result {
        Err(chromatic::ParseColourError::ParseFloat(_)) => { /* Expected */ }
        _ => panic!("Expected ParseFloat error"),
    }

    // Test ParseHex error
    let result = Grey::<f32>::from_str("#Z");
    match result {
        Err(chromatic::ParseColourError::ParseHex(_)) => { /* Expected */ }
        _ => panic!("Expected ParseHex error"),
    }

    // Make sure the error type implements Debug
    let result = Grey::<f32>::from_str("invalid");
    if let Err(e) = result {
        let _debug_str = format!("{:?}", e);
        // Just check that this doesn't panic
    }
}

// Tests for additional edge cases in string parsing.
#[test]
fn test_grey_from_str_edge_cases() {
    // Test parsing exact upper/lower bounds
    let grey = Grey::<f32>::from_str("0.0").unwrap();
    assert_eq!(grey.grey(), 0.0);

    let grey = Grey::<f32>::from_str("1.0").unwrap();
    assert_eq!(grey.grey(), 1.0);

    // Test that values are correctly mapped from hex
    let grey = Grey::<f32>::from_str("#7").unwrap();
    let expected = 7.0 / 15.0;
    assert!((grey.grey() - expected).abs() < 0.0001);

    let grey = Grey::<f32>::from_str("#a").unwrap(); // Test lowercase hex
    let expected = 10.0 / 15.0;
    assert!((grey.grey() - expected).abs() < 0.0001);
}
