use approx::assert_relative_eq;
use chromatic::{ColourParseError, Rgb};
use core::str::FromStr;

#[test]
fn test_rgb_from_str_valid() {
    let result = Rgb::<f32>::from_str("#FF0080");
    assert!(result.is_ok());
    let rgb = result.unwrap();
    assert_relative_eq!(rgb.r(), 255.0 / 255.0);
    assert_relative_eq!(rgb.g(), 0.0 / 255.0);
    assert_relative_eq!(rgb.b(), 128.0 / 255.0);
}

#[test]
fn test_rgb_from_str_empty() {
    let result = Rgb::<f32>::from_str("");

    match result {
        Err(ColourParseError::InvalidLength(len)) => {
            assert_eq!(len, 0, "Expected length to be 0");
        }
        _ => panic!("Expected InvalidLength error, got {:?}", result),
    }
}

#[test]
fn test_rgb_from_str_wrong_length() {
    let result = Rgb::<f32>::from_str("#00000");

    match result {
        Err(ColourParseError::InvalidLength(len)) => {
            assert_eq!(len, 5, "Expected length to be 5");
        }
        _ => panic!("Expected InvalidLength error, got {:?}", result),
    }
}

#[test]
fn test_rgb_from_str_invalid_chars() {
    let result = Rgb::<f32>::from_str("#XYZABC");

    match result {
        Err(ColourParseError::InvalidHex(_)) => {}
        _ => panic!("Expected InvalidHex error, got {:?}", result),
    }
}

#[test]
fn test_rgb_from_str_no_hash() {
    let result = Rgb::<f32>::from_str("FF0080");
    assert!(result.is_ok());
    let rgb = result.unwrap();
    assert_relative_eq!(rgb.r(), 255.0 / 255.0);
    assert_relative_eq!(rgb.g(), 0.0 / 255.0);
    assert_relative_eq!(rgb.b(), 128.0 / 255.0);
}

#[test]
fn test_rgb_from_str_with_whitespace() {
    let result = Rgb::<f32>::from_str("  #FF0080  ");
    assert!(result.is_ok());
    let rgb = result.unwrap();
    assert_relative_eq!(rgb.r(), 255.0 / 255.0);
    assert_relative_eq!(rgb.g(), 0.0 / 255.0);
    assert_relative_eq!(rgb.b(), 128.0 / 255.0);
}
