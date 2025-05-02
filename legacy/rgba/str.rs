use chromatic::{Colour, Rgba};
use core::str::FromStr;

// Tests for Display implementation.
#[test]
fn test_rgba_display() {
    // Test formatting to hex strings
    let rgba = Rgba::<f32>::new(0.0, 0.0, 0.0, 0.0);
    assert_eq!(format!("{}", rgba), "#00000000");

    let rgba = Rgba::<f32>::new(1.0, 1.0, 1.0, 1.0);
    assert_eq!(format!("{}", rgba), "#FFFFFFFF");

    let rgba = Rgba::<f32>::new(1.0, 0.0, 0.0, 1.0);
    assert_eq!(format!("{}", rgba), "#FF0000FF");

    let rgba = Rgba::<f32>::new(0.0, 1.0, 0.0, 0.5);
    assert_eq!(format!("{}", rgba), "#00FF0080");

    let rgba = Rgba::<f32>::new(0.0, 0.0, 1.0, 0.0);
    assert_eq!(format!("{}", rgba), "#0000FF00");

    // Test mixed values - approximately 0.5 for each component
    let rgba = Rgba::<f32>::new(0.5, 0.5, 0.5, 0.5);
    assert_eq!(format!("{}", rgba), "#80808080");
}

// Test parsing from string (FromStr implementation).
#[test]
fn test_rgba_from_str() {
    // Test parsing hex strings
    let rgba = Rgba::<f32>::from_str("#00000000").unwrap();
    assert_eq!(rgba.red(), 0.0);
    assert_eq!(rgba.green(), 0.0);
    assert_eq!(rgba.blue(), 0.0);
    assert_eq!(rgba.alpha(), 0.0);

    let rgba = Rgba::<f32>::from_str("#FFFFFFFF").unwrap();
    assert_eq!(rgba.red(), 1.0);
    assert_eq!(rgba.green(), 1.0);
    assert_eq!(rgba.blue(), 1.0);
    assert_eq!(rgba.alpha(), 1.0);

    let rgba = Rgba::<f32>::from_str("#FF0000FF").unwrap();
    assert_eq!(rgba.red(), 1.0);
    assert_eq!(rgba.green(), 0.0);
    assert_eq!(rgba.blue(), 0.0);
    assert_eq!(rgba.alpha(), 1.0);

    let rgba = Rgba::<f32>::from_str("#00FF0080").unwrap();
    assert_eq!(rgba.red(), 0.0);
    assert_eq!(rgba.green(), 1.0);
    assert_eq!(rgba.blue(), 0.0);
    assert_eq!(rgba.alpha(), 0.5019608);

    let rgba = Rgba::<f32>::from_str("#0000FF00").unwrap();
    assert_eq!(rgba.red(), 0.0);
    assert_eq!(rgba.green(), 0.0);
    assert_eq!(rgba.blue(), 1.0);
    assert_eq!(rgba.alpha(), 0.0);

    // Test lowercase hex values
    let rgba = Rgba::<f32>::from_str("#aabbccdd").unwrap();
    assert!((rgba.red() - 0.667).abs() < Rgba::<f32>::tolerance());
    assert!((rgba.green() - 0.733).abs() < Rgba::<f32>::tolerance());
    assert!((rgba.blue() - 0.800).abs() < Rgba::<f32>::tolerance());
    assert!((rgba.alpha() - 0.867).abs() < Rgba::<f32>::tolerance());

    // Test parsing with whitespace
    let rgba = Rgba::<f32>::from_str(" #12345678 ").unwrap();
    assert!((rgba.red() - 0.071).abs() < Rgba::<f32>::tolerance());
    assert!((rgba.green() - 0.204).abs() < Rgba::<f32>::tolerance());
    assert!((rgba.blue() - 0.337).abs() < Rgba::<f32>::tolerance());
    assert!((rgba.alpha() - 0.471).abs() < Rgba::<f32>::tolerance());

    // Test short form hex (#RGBA)
    let rgba = Rgba::<f32>::from_str("#F00F").unwrap();
    assert_eq!(rgba.red(), 1.0);
    assert_eq!(rgba.green(), 0.0);
    assert_eq!(rgba.blue(), 0.0);
    assert_eq!(rgba.alpha(), 1.0);

    let rgba = Rgba::<f32>::from_str("#0F08").unwrap();
    assert_eq!(rgba.red(), 0.0);
    assert_eq!(rgba.green(), 1.0);
    assert_eq!(rgba.blue(), 0.0);
    assert_eq!(rgba.alpha(), 0.53333336);

    let rgba = Rgba::<f32>::from_str("#00F0").unwrap();
    assert_eq!(rgba.red(), 0.0);
    assert_eq!(rgba.green(), 0.0);
    assert_eq!(rgba.blue(), 1.0);
    assert_eq!(rgba.alpha(), 0.0);

    // Test short form mixed values
    let rgba = Rgba::<f32>::from_str("#7AD5").unwrap();
    assert!((rgba.red() - 0.467).abs() < Rgba::<f32>::tolerance());
    assert!((rgba.green() - 0.667).abs() < Rgba::<f32>::tolerance());
    assert!((rgba.blue() - 0.867).abs() < Rgba::<f32>::tolerance());
    assert!((rgba.alpha() - 0.333).abs() < Rgba::<f32>::tolerance());

    // Test parsing float values
    let rgba = Rgba::<f32>::from_str("0.2, 0.5, 0.8, 0.6").unwrap();
    assert!((rgba.red() - 0.2).abs() < Rgba::<f32>::tolerance());
    assert!((rgba.green() - 0.5).abs() < Rgba::<f32>::tolerance());
    assert!((rgba.blue() - 0.8).abs() < Rgba::<f32>::tolerance());
    assert!((rgba.alpha() - 0.6).abs() < Rgba::<f32>::tolerance());

    // Test parsing float values with whitespace
    let rgba = Rgba::<f32>::from_str(" 0.1 , 0.6 , 0.9 , 0.4 ").unwrap();
    assert!((rgba.red() - 0.1).abs() < Rgba::<f32>::tolerance());
    assert!((rgba.green() - 0.6).abs() < Rgba::<f32>::tolerance());
    assert!((rgba.blue() - 0.9).abs() < Rgba::<f32>::tolerance());
    assert!((rgba.alpha() - 0.4).abs() < Rgba::<f32>::tolerance());
}

// Test consistency between FromStr and Display (round-trip tests).
#[test]
fn test_rgba_round_trip() {
    // Display -> FromStr should result in the same value for standard hex values
    for r in (0..=255).step_by(51) {
        for g in (0..=255).step_by(51) {
            for b in (0..=255).step_by(51) {
                for a in (0..=255).step_by(51) {
                    let hex_str = format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a);
                    let rgba = Rgba::<f32>::from_str(&hex_str).unwrap();
                    let formatted = format!("{}", rgba);

                    assert_eq!(formatted.to_uppercase(), hex_str.to_uppercase());
                }
            }
        }
    }
}

// Tests for error handling and error types.
#[test]
fn test_parse_rgba_error_types() {
    // Test ParseFloat error
    let result = Rgba::<f32>::from_str("not_a_number, 0.5, 0.5, 0.5");
    match result {
        Err(chromatic::ParseColourError::ParseFloat(_)) => { /* Expected */ }
        _ => panic!("Expected ParseFloat error"),
    }

    // Test ParseHex error
    let result = Rgba::<f32>::from_str("#GHIJKLMN");
    match result {
        Err(chromatic::ParseColourError::ParseHex(_)) => { /* Expected */ }
        _ => panic!("Expected ParseHex error"),
    }

    // Test InvalidFormat error (too few or too many characters in hex)
    let result = Rgba::<f32>::from_str("#123");
    match result {
        Err(chromatic::ParseColourError::InvalidFormat) => { /* Expected */ }
        _ => panic!("Expected InvalidFormat error"),
    }

    let result = Rgba::<f32>::from_str("#1234567");
    match result {
        Err(chromatic::ParseColourError::InvalidFormat) => { /* Expected */ }
        _ => panic!("Expected InvalidFormat error"),
    }

    // Test InvalidFormat error (not enough comma-separated values)
    let result = Rgba::<f32>::from_str("0.5, 0.5, 0.5");
    match result {
        Err(chromatic::ParseColourError::InvalidFormat) => { /* Expected */ }
        _ => panic!("Expected InvalidFormat error"),
    }

    let result = Rgba::<f32>::from_str("0.1, 0.2, 0.3, 0.4, 0.5");
    match result {
        Err(chromatic::ParseColourError::InvalidFormat) => { /* Expected */ }
        _ => panic!("Expected InvalidFormat error"),
    }

    // Make sure the error type implements Debug
    let result = Rgba::<f32>::from_str("invalid");
    if let Err(e) = result {
        let _debug_str = format!("{:?}", e);
        // Just check that this doesn't panic
    }
}

// Tests for additional edge cases in string parsing.
#[test]
fn test_rgba_from_str_edge_cases() {
    // Test parsing exact upper/lower bounds
    let rgba = Rgba::<f32>::from_str("0.0, 0.0, 0.0, 0.0").unwrap();
    assert_eq!(rgba.red(), 0.0);
    assert_eq!(rgba.green(), 0.0);
    assert_eq!(rgba.blue(), 0.0);
    assert_eq!(rgba.alpha(), 0.0);

    let rgba = Rgba::<f32>::from_str("1.0, 1.0, 1.0, 1.0").unwrap();
    assert_eq!(rgba.red(), 1.0);
    assert_eq!(rgba.green(), 1.0);
    assert_eq!(rgba.blue(), 1.0);
    assert_eq!(rgba.alpha(), 1.0);

    // Test mixed boundary values
    let rgba = Rgba::<f32>::from_str("0.0, 0.0, 1.0, 0.5").unwrap();
    assert_eq!(rgba.red(), 0.0);
    assert_eq!(rgba.green(), 0.0);
    assert_eq!(rgba.blue(), 1.0);
    assert_eq!(rgba.alpha(), 0.5);

    let rgba = Rgba::<f32>::from_str("1.0, 0.0, 0.0, 0.0").unwrap();
    assert_eq!(rgba.red(), 1.0);
    assert_eq!(rgba.green(), 0.0);
    assert_eq!(rgba.blue(), 0.0);
    assert_eq!(rgba.alpha(), 0.0);

    // Test common colours with alpha
    let colours = [
        ("#FF0000FF", "red (opaque)", (1.0, 0.0, 0.0, 1.0)),
        ("#00FF00FF", "green (opaque)", (0.0, 1.0, 0.0, 1.0)),
        ("#0000FFFF", "blue (opaque)", (0.0, 0.0, 1.0, 1.0)),
        ("#FFFF0080", "yellow (semi-transparent)", (1.0, 1.0, 0.0, 0.5)),
        ("#00FFFF80", "cyan (semi-transparent)", (0.0, 1.0, 1.0, 0.5)),
        ("#FF00FF80", "magenta (semi-transparent)", (1.0, 0.0, 1.0, 0.5)),
        ("#C0C0C0C0", "silver (75% opaque)", (0.75, 0.75, 0.75, 0.75)),
        ("#80808080", "gray (50% opaque)", (0.5, 0.5, 0.5, 0.5)),
        ("#80000000", "maroon (transparent)", (0.5, 0.0, 0.0, 0.0)),
        ("#80800080", "olive (50% opaque)", (0.5, 0.5, 0.0, 0.5)),
    ];

    for (hex, name, (r, g, b, a)) in colours {
        let rgba = Rgba::<f32>::from_str(hex).unwrap();
        assert!(
            (rgba.red() - r).abs() < Rgba::<f32>::tolerance(),
            "Failed for colour {}: red",
            name
        );
        assert!(
            (rgba.green() - g).abs() < Rgba::<f32>::tolerance(),
            "Failed for colour {}: green",
            name
        );
        assert!(
            (rgba.blue() - b).abs() < Rgba::<f32>::tolerance(),
            "Failed for colour {}: blue",
            name
        );
        assert!(
            (rgba.alpha() - a).abs() < Rgba::<f32>::tolerance(),
            "Failed for colour {}: alpha",
            name
        );
    }
}
