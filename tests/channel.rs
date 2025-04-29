use approx::assert_relative_eq;
use chromatic::Channel;

#[test]
fn test_u8_channel_conversion() {
    // For u8, the value should remain unchanged
    let original: u8 = 128;
    let converted = u8::from_u8(original);
    assert_eq!(converted, 128);

    // Full range test
    assert_eq!(u8::from_u8(0), 0);
    assert_eq!(u8::from_u8(255), 255);
}

#[test]
fn test_integer_channel_conversion() {
    // Test conversion to larger integer types

    // u16
    let converted_u16 = u16::from_u8(128);
    assert_eq!(converted_u16, 128u16);

    // u32
    let converted_u32 = u32::from_u8(128);
    assert_eq!(converted_u32, 128u32);

    // u64
    let converted_u64 = u64::from_u8(128);
    assert_eq!(converted_u64, 128u64);

    // usize
    let converted_usize = usize::from_u8(128);
    assert_eq!(converted_usize, 128usize);

    // Edge cases
    assert_eq!(u16::from_u8(0), 0u16);
    assert_eq!(u32::from_u8(0), 0u32);
    assert_eq!(u64::from_u8(0), 0u64);
    assert_eq!(usize::from_u8(0), 0usize);

    assert_eq!(u16::from_u8(255), 255u16);
    assert_eq!(u32::from_u8(255), 255u32);
    assert_eq!(u64::from_u8(255), 255u64);
    assert_eq!(usize::from_u8(255), 255usize);
}

#[test]
fn test_float_channel_conversion() {
    // For floating point types, the value should be normalized to [0.0, 1.0]

    // f32
    let converted_f32 = f32::from_u8(128);
    assert_relative_eq!(converted_f32, 128.0 / 255.0);

    // f64
    let converted_f64 = f64::from_u8(128);
    assert_relative_eq!(converted_f64, 128.0 / 255.0);

    // Edge cases

    // 0 should map to 0.0
    assert_relative_eq!(f32::from_u8(0), 0.0);
    assert_relative_eq!(f64::from_u8(0), 0.0);

    // 255 should map to 1.0
    assert_relative_eq!(f32::from_u8(255), 1.0);
    assert_relative_eq!(f64::from_u8(255), 1.0);

    // Middle value tests
    assert_relative_eq!(f32::from_u8(64), 64.0 / 255.0);
    assert_relative_eq!(f64::from_u8(64), 64.0 / 255.0);

    assert_relative_eq!(f32::from_u8(127), 127.0 / 255.0);
    assert_relative_eq!(f64::from_u8(127), 127.0 / 255.0);

    assert_relative_eq!(f32::from_u8(192), 192.0 / 255.0);
    assert_relative_eq!(f64::from_u8(192), 192.0 / 255.0);
}

#[test]
fn test_channel_precision() {
    // Test that the conversion preserves precision when converting to higher bit-depth types

    // f32 - should be precise enough for most colour calculations
    let step = 1.0 / 255.0;
    for i in 0..=255 {
        let expected = i as f32 * step;
        let converted = f32::from_u8(i);
        assert_relative_eq!(converted, expected, epsilon = 1e-7);
    }

    // f64 - should offer even better precision
    for i in 0..=255 {
        let expected = i as f64 / 255.0;
        let converted = f64::from_u8(i);
        assert_relative_eq!(converted, expected, epsilon = 1e-15);
    }
}

#[test]
fn test_channel_consistency() {
    // Test that all implementations produce consistent results
    // For example, u8(128) should be equivalent to f32(0.5019...) in representing the same colour intensity

    let original: u8 = 128;

    let as_u8 = u8::from_u8(original);
    let as_u16 = u16::from_u8(original);
    let as_f32 = f32::from_u8(original);

    // Integer types should retain the exact value
    assert_eq!(as_u8, 128);
    assert_eq!(as_u16, 128);

    // Floating point should normalize to [0, 1]
    assert_relative_eq!(as_f32, 128.0 / 255.0);

    // Conversion to original value should be consistent
    // This is crucial for colour transformations to be predictable
    let back_to_u8 = (as_f32 * 255.0).round() as u8;
    assert_eq!(back_to_u8, original);
}
