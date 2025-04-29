use approx::assert_relative_eq;
use chromatic::{ColourParseError, Grey};
use core::str::FromStr;

// Test successful parsing
#[test]
fn test_grey_from_str_valid() {
    let result = Grey::<f32>::from_str("#80");
    assert!(result.is_ok());
    let grey = result.unwrap();
    assert_relative_eq!(grey.g(), 128.0 / 255.0);
}

// Test empty string
#[test]
fn test_grey_from_str_empty() {
    let result = Grey::<f32>::from_str("");

    match result {
        Err(ColourParseError::InvalidLength(len)) => {
            assert_eq!(len, 0, "Expected length to be 0");
        }
        _ => panic!("Expected InvalidLength error, got {:?}", result),
    }
}

// Test wrong length
#[test]
fn test_grey_from_str_wrong_length() {
    let result = Grey::<f32>::from_str("#000");

    match result {
        Err(ColourParseError::InvalidLength(len)) => {
            assert_eq!(len, 3, "Expected length to be 3");
        }
        _ => panic!("Expected InvalidLength error, got {:?}", result),
    }
}

// Test invalid characters
#[test]
fn test_grey_from_str_invalid_chars() {
    let result = Grey::<f32>::from_str("#XY");

    match result {
        Err(ColourParseError::InvalidHex(_)) => {}
        _ => panic!("Expected InvalidHex error, got {:?}", result),
    }
}

// Test with leading hashtag but wrong length
#[test]
fn test_grey_from_str_with_hash_wrong_length() {
    let result = Grey::<f32>::from_str("#1");

    match result {
        Err(ColourParseError::InvalidLength(len)) => {
            assert_eq!(len, 1, "Expected length to be 1");
        }
        _ => panic!("Expected InvalidLength error, got {:?}", result),
    }
}

// Test with no leading hashtag and valid length
#[test]
fn test_grey_from_str_no_hash() {
    let result = Grey::<f32>::from_str("80");
    assert!(result.is_ok());
    let grey = result.unwrap();
    assert_relative_eq!(grey.g(), 128.0 / 255.0);
}

// Test with both whitespace and hashtag prefix
#[test]
fn test_grey_from_str_with_whitespace() {
    let result = Grey::<f32>::from_str("  #80  ");
    assert!(result.is_ok());
    let grey = result.unwrap();
    assert_relative_eq!(grey.g(), 128.0 / 255.0);
}

// Test with whitespace and no hashtag prefix
#[test]
fn test_grey_from_str_with_whitespace_no_hashtag() {
    let result = Grey::<f32>::from_str("   80  ");
    assert!(result.is_ok());
    let grey = result.unwrap();
    assert_relative_eq!(grey.g(), 128.0 / 255.0);
}
