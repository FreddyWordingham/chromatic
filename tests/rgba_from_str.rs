use approx::assert_relative_eq;
use chromatic::{ColourParseError, Rgba};
use core::str::FromStr;

#[test]
fn test_rgba_from_str_valid() {
    let result = Rgba::<f32>::from_str("#FF0080FF");
    assert!(result.is_ok());
    let rgba = result.unwrap();
    assert_relative_eq!(rgba.r(), 255.0 / 255.0);
    assert_relative_eq!(rgba.g(), 0.0 / 255.0);
    assert_relative_eq!(rgba.b(), 128.0 / 255.0);
    assert_relative_eq!(rgba.a(), 255.0 / 255.0);
}

#[test]
fn test_rgba_from_str_empty() {
    let result = Rgba::<f32>::from_str("");

    match result {
        Err(ColourParseError::InvalidLength(len)) => {
            assert_eq!(len, 0, "Expected length to be 0");
        }
        _ => panic!("Expected InvalidLength error, got {:?}", result),
    }
}

#[test]
fn test_rgba_from_str_wrong_length() {
    let result = Rgba::<f32>::from_str("#FF0080");

    match result {
        Err(ColourParseError::InvalidLength(len)) => {
            assert_eq!(len, 6, "Expected length to be 6");
        }
        _ => panic!("Expected InvalidLength error, got {:?}", result),
    }
}

#[test]
fn test_rgba_from_str_invalid_chars() {
    let result = Rgba::<f32>::from_str("#XYZABCDE");

    match result {
        Err(ColourParseError::InvalidHex(_)) => {}
        _ => panic!("Expected InvalidHex error, got {:?}", result),
    }
}

#[test]
fn test_rgba_from_str_no_hash() {
    let result = Rgba::<f32>::from_str("FF0080FF");
    assert!(result.is_ok());
    let rgba = result.unwrap();
    assert_relative_eq!(rgba.r(), 255.0 / 255.0);
    assert_relative_eq!(rgba.g(), 0.0 / 255.0);
    assert_relative_eq!(rgba.b(), 128.0 / 255.0);
    assert_relative_eq!(rgba.a(), 255.0 / 255.0);
}

#[test]
fn test_rgba_from_str_with_whitespace() {
    let result = Rgba::<f32>::from_str("  #FF0080FF  ");
    assert!(result.is_ok());
    let rgba = result.unwrap();
    assert_relative_eq!(rgba.r(), 255.0 / 255.0);
    assert_relative_eq!(rgba.g(), 0.0 / 255.0);
    assert_relative_eq!(rgba.b(), 128.0 / 255.0);
    assert_relative_eq!(rgba.a(), 255.0 / 255.0);
}
