use chromatic::{Colour, Rgb};
use core::str::FromStr;

// Tests for Display implementation.
#[test]
fn test_rgb_display() {
    // Test formatting to hex strings
    let rgb = Rgb::<f32>::new(0.0, 0.0, 0.0);
    assert_eq!(format!("{}", rgb), "#000000");

    let rgb = Rgb::<f32>::new(1.0, 1.0, 1.0);
    assert_eq!(format!("{}", rgb), "#FFFFFF");

    let rgb = Rgb::<f32>::new(1.0, 0.0, 0.0);
    assert_eq!(format!("{}", rgb), "#FF0000");

    let rgb = Rgb::<f32>::new(0.0, 1.0, 0.0);
    assert_eq!(format!("{}", rgb), "#00FF00");

    let rgb = Rgb::<f32>::new(0.0, 0.0, 1.0);
    assert_eq!(format!("{}", rgb), "#0000FF");

    // Test mixed values - approximately 0.5 for each component
    let rgb = Rgb::<f32>::new(0.5, 0.5, 0.5);
    assert_eq!(format!("{}", rgb), "#808080");
}

// Test parsing from string (FromStr implementation).
#[test]
fn test_rgb_from_str() {
    // Test parsing hex strings
    let rgb = Rgb::<f32>::from_str("#000000").unwrap();
    assert_eq!(rgb.red(), 0.0);
    assert_eq!(rgb.green(), 0.0);
    assert_eq!(rgb.blue(), 0.0);

    let rgb = Rgb::<f32>::from_str("#FFFFFF").unwrap();
    assert_eq!(rgb.red(), 1.0);
    assert_eq!(rgb.green(), 1.0);
    assert_eq!(rgb.blue(), 1.0);

    let rgb = Rgb::<f32>::from_str("#FF0000").unwrap();
    assert_eq!(rgb.red(), 1.0);
    assert_eq!(rgb.green(), 0.0);
    assert_eq!(rgb.blue(), 0.0);

    let rgb = Rgb::<f32>::from_str("#00FF00").unwrap();
    assert_eq!(rgb.red(), 0.0);
    assert_eq!(rgb.green(), 1.0);
    assert_eq!(rgb.blue(), 0.0);

    let rgb = Rgb::<f32>::from_str("#0000FF").unwrap();
    assert_eq!(rgb.red(), 0.0);
    assert_eq!(rgb.green(), 0.0);
    assert_eq!(rgb.blue(), 1.0);

    // Test lowercase hex values
    let rgb = Rgb::<f32>::from_str("#aabbcc").unwrap();
    assert!((rgb.red() - 0.667).abs() < Rgb::<f32>::tolerance());
    assert!((rgb.green() - 0.733).abs() < Rgb::<f32>::tolerance());
    assert!((rgb.blue() - 0.800).abs() < Rgb::<f32>::tolerance());

    // Test parsing with whitespace
    let rgb = Rgb::<f32>::from_str(" #123456 ").unwrap();
    assert!((rgb.red() - 0.071).abs() < Rgb::<f32>::tolerance());
    assert!((rgb.green() - 0.204).abs() < Rgb::<f32>::tolerance());
    assert!((rgb.blue() - 0.337).abs() < Rgb::<f32>::tolerance());

    // Test short form hex (#RGB)
    let rgb = Rgb::<f32>::from_str("#F00").unwrap();
    assert_eq!(rgb.red(), 1.0);
    assert_eq!(rgb.green(), 0.0);
    assert_eq!(rgb.blue(), 0.0);

    let rgb = Rgb::<f32>::from_str("#0F0").unwrap();
    assert_eq!(rgb.red(), 0.0);
    assert_eq!(rgb.green(), 1.0);
    assert_eq!(rgb.blue(), 0.0);

    let rgb = Rgb::<f32>::from_str("#00F").unwrap();
    assert_eq!(rgb.red(), 0.0);
    assert_eq!(rgb.green(), 0.0);
    assert_eq!(rgb.blue(), 1.0);

    // Test short form mixed values
    let rgb = Rgb::<f32>::from_str("#7AD").unwrap();
    assert!((rgb.red() - 0.467).abs() < Rgb::<f32>::tolerance());
    assert!((rgb.green() - 0.667).abs() < Rgb::<f32>::tolerance());
    assert!((rgb.blue() - 0.867).abs() < Rgb::<f32>::tolerance());

    // Test parsing float values
    let rgb = Rgb::<f32>::from_str("0.2, 0.5, 0.8").unwrap();
    assert!((rgb.red() - 0.2).abs() < Rgb::<f32>::tolerance());
    assert!((rgb.green() - 0.5).abs() < Rgb::<f32>::tolerance());
    assert!((rgb.blue() - 0.8).abs() < Rgb::<f32>::tolerance());

    // Test parsing float values with whitespace
    let rgb = Rgb::<f32>::from_str(" 0.1 , 0.6 , 0.9 ").unwrap();
    assert!((rgb.red() - 0.1).abs() < Rgb::<f32>::tolerance());
    assert!((rgb.green() - 0.6).abs() < Rgb::<f32>::tolerance());
    assert!((rgb.blue() - 0.9).abs() < Rgb::<f32>::tolerance());
}

// Test consistency between FromStr and Display (round-trip tests).
#[test]
fn test_rgb_round_trip() {
    // Display -> FromStr should result in the same value for standard hex values
    for r in (0..=255).step_by(51) {
        for g in (0..=255).step_by(51) {
            for b in (0..=255).step_by(51) {
                let hex_str = format!("#{:02X}{:02X}{:02X}", r, g, b);
                let rgb = Rgb::<f32>::from_str(&hex_str).unwrap();
                let formatted = format!("{}", rgb);

                assert_eq!(formatted.to_uppercase(), hex_str.to_uppercase());
            }
        }
    }
}

// Tests for error handling and error types.
#[test]
fn test_parse_rgb_error_types() {
    // Test ParseFloat error
    let result = Rgb::<f32>::from_str("not_a_number, 0.5, 0.5");
    match result {
        Err(chromatic::ParseRgbError::ParseFloat(_)) => { /* Expected */ }
        _ => panic!("Expected ParseFloat error"),
    }

    // Test ParseHex error
    let result = Rgb::<f32>::from_str("#GHIJKL");
    match result {
        Err(chromatic::ParseRgbError::ParseHex(_)) => { /* Expected */ }
        _ => panic!("Expected ParseHex error"),
    }

    // Test InvalidFormat error (too few or too many characters in hex)
    let result = Rgb::<f32>::from_str("#12");
    match result {
        Err(chromatic::ParseRgbError::InvalidFormat) => { /* Expected */ }
        _ => panic!("Expected InvalidFormat error"),
    }

    let result = Rgb::<f32>::from_str("#1234567");
    match result {
        Err(chromatic::ParseRgbError::InvalidFormat) => { /* Expected */ }
        _ => panic!("Expected InvalidFormat error"),
    }

    // Test InvalidFormat error (not enough comma-separated values)
    let result = Rgb::<f32>::from_str("0.5, 0.5");
    match result {
        Err(chromatic::ParseRgbError::InvalidFormat) => { /* Expected */ }
        _ => panic!("Expected InvalidFormat error"),
    }

    let result = Rgb::<f32>::from_str("0.1, 0.2, 0.3, 0.4");
    match result {
        Err(chromatic::ParseRgbError::InvalidFormat) => { /* Expected */ }
        _ => panic!("Expected InvalidFormat error"),
    }

    // Make sure the error type implements Debug
    let result = Rgb::<f32>::from_str("invalid");
    if let Err(e) = result {
        let _debug_str = format!("{:?}", e);
        // Just check that this doesn't panic
    }
}

// Tests for additional edge cases in string parsing.
#[test]
fn test_rgb_from_str_edge_cases() {
    // Test parsing exact upper/lower bounds
    let rgb = Rgb::<f32>::from_str("0.0, 0.0, 0.0").unwrap();
    assert_eq!(rgb.red(), 0.0);
    assert_eq!(rgb.green(), 0.0);
    assert_eq!(rgb.blue(), 0.0);

    let rgb = Rgb::<f32>::from_str("1.0, 1.0, 1.0").unwrap();
    assert_eq!(rgb.red(), 1.0);
    assert_eq!(rgb.green(), 1.0);
    assert_eq!(rgb.blue(), 1.0);

    // Test mixed boundary values
    let rgb = Rgb::<f32>::from_str("0.0, 0.0, 1.0").unwrap();
    assert_eq!(rgb.red(), 0.0);
    assert_eq!(rgb.green(), 0.0);
    assert_eq!(rgb.blue(), 1.0);

    let rgb = Rgb::<f32>::from_str("1.0, 0.0, 0.0").unwrap();
    assert_eq!(rgb.red(), 1.0);
    assert_eq!(rgb.green(), 0.0);
    assert_eq!(rgb.blue(), 0.0);

    // Test common HTML colour names via hex codes
    let colours = [
        ("#FF0000", "red", (1.0, 0.0, 0.0)),
        ("#00FF00", "green", (0.0, 1.0, 0.0)),
        ("#0000FF", "blue", (0.0, 0.0, 1.0)),
        ("#FFFF00", "yellow", (1.0, 1.0, 0.0)),
        ("#00FFFF", "cyan", (0.0, 1.0, 1.0)),
        ("#FF00FF", "magenta", (1.0, 0.0, 1.0)),
        ("#C0C0C0", "silver", (0.75, 0.75, 0.75)),
        ("#808080", "gray", (0.5, 0.5, 0.5)),
        ("#800000", "maroon", (0.5, 0.0, 0.0)),
        ("#808000", "olive", (0.5, 0.5, 0.0)),
    ];

    for (hex, name, (r, g, b)) in colours {
        let rgb = Rgb::<f32>::from_str(hex).unwrap();
        assert!(
            (rgb.red() - r).abs() < Rgb::<f32>::tolerance(),
            "Failed for colour {}: red",
            name
        );
        assert!(
            (rgb.green() - g).abs() < Rgb::<f32>::tolerance(),
            "Failed for colour {}: green",
            name
        );
        assert!(
            (rgb.blue() - b).abs() < Rgb::<f32>::tolerance(),
            "Failed for colour {}: blue",
            name
        );
    }
}
