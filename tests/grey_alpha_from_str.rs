use approx::assert_relative_eq;
use chromatic::{ColourParseError, GreyAlpha};
use core::str::FromStr;

#[test]
fn test_grey_alpha_from_str_valid() {
    let result = GreyAlpha::<f32>::from_str("#80FF");
    assert!(result.is_ok());
    let ga = result.unwrap();
    assert_relative_eq!(ga.g(), 128.0 / 255.0);
    assert_relative_eq!(ga.a(), 255.0 / 255.0);
}

#[test]
fn test_grey_alpha_from_str_empty() {
    let result = GreyAlpha::<f32>::from_str("");

    match result {
        Err(ColourParseError::InvalidLength(len)) => {
            assert_eq!(len, 0, "Expected length to be 0");
        }
        _ => panic!("Expected InvalidLength error, got {:?}", result),
    }
}

#[test]
fn test_grey_alpha_from_str_wrong_length() {
    let result = GreyAlpha::<f32>::from_str("#000");

    match result {
        Err(ColourParseError::InvalidLength(len)) => {
            assert_eq!(len, 3, "Expected length to be 3");
        }
        _ => panic!("Expected InvalidLength error, got {:?}", result),
    }
}

#[test]
fn test_grey_alpha_from_str_invalid_chars() {
    let result = GreyAlpha::<f32>::from_str("#XYWZ");

    match result {
        Err(ColourParseError::InvalidHex(_)) => {}
        _ => panic!("Expected InvalidHex error, got {:?}", result),
    }
}

#[test]
fn test_grey_alpha_from_str_no_hash() {
    let result = GreyAlpha::<f32>::from_str("80FF");
    assert!(result.is_ok());
    let ga = result.unwrap();
    assert_relative_eq!(ga.g(), 128.0 / 255.0);
    assert_relative_eq!(ga.a(), 255.0 / 255.0);
}

#[test]
fn test_grey_alpha_from_str_with_whitespace() {
    let result = GreyAlpha::<f32>::from_str("  #80FF  ");
    assert!(result.is_ok());
    let ga = result.unwrap();
    assert_relative_eq!(ga.g(), 128.0 / 255.0);
    assert_relative_eq!(ga.a(), 255.0 / 255.0);
}
