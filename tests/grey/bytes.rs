use chromatic::{Colour, Grey};

// Test basic functionality of from_bytes method.
#[test]
fn test_grey_from_bytes() {
    // Test black
    let grey = Grey::<f32>::from_bytes([0]);
    assert_eq!(grey.grey(), 0.0);

    // Test white
    let grey = Grey::<f32>::from_bytes([255]);
    assert_eq!(grey.grey(), 1.0);

    // Test middle value
    let grey = Grey::<f32>::from_bytes([127]);
    // Expected value is approximately 0.498... which is 127/255
    let expected = 127.0 / 255.0;
    assert!((grey.grey() - expected).abs() < Grey::<f32>::tolerance());
}

// Test basic functionality of to_bytes method.
#[test]
fn test_grey_to_bytes() {
    // Test black
    let grey = Grey::<f32>::new(0.0);
    assert_eq!(grey.to_bytes(), [0]);

    // Test white
    let grey = Grey::<f32>::new(1.0);
    assert_eq!(grey.to_bytes(), [255]);

    // Test middle value
    let grey = Grey::<f32>::new(0.5);
    assert_eq!(grey.to_bytes(), [128]);
}

// Test round-trip conversion between bytes and float.
#[test]
fn test_grey_bytes_round_trip() {
    // Test all possible byte values
    for byte in 0..=255 {
        let original_bytes = [byte];
        let grey = Grey::<f32>::from_bytes(original_bytes);
        let round_trip_bytes = grey.to_bytes();

        assert_eq!(original_bytes, round_trip_bytes);
    }
}

// Test round-trip conversion between float and bytes.
#[test]
fn test_grey_float_round_trip() {
    // Test a range of float values that should map precisely to bytes
    for i in 0..=255 {
        let float_value = i as f32 / 255.0;
        let grey = Grey::<f32>::new(float_value);
        let bytes = grey.to_bytes();
        let round_trip_grey = Grey::<f32>::from_bytes(bytes);

        // The round-trip should preserve the value within tolerance
        assert_eq!(grey, round_trip_grey);
    }
}

// Test rounding behavior in to_bytes.
#[test]
fn test_grey_to_bytes_rounding() {
    // Test values just below the threshold for rounding up
    let grey = Grey::<f32>::new(0.5 - 0.001);
    assert_eq!(grey.to_bytes(), [127]);

    // Test values just above the threshold for rounding up
    let grey = Grey::<f32>::new(0.5 + 0.001);
    assert_eq!(grey.to_bytes(), [128]);

    // Test more critical rounding cases
    let test_cases = [
        (1.0 / 255.0 * 0.4, [0]),       // Should round down to 0
        (1.0 / 255.0 * 0.6, [1]),       // Should round up to 1
        (254.0 / 255.0 - 0.001, [254]), // Should be within tolerance of 254
        (254.0 / 255.0 + 0.001, [254]), // Should be within tolerance of 254
    ];

    for (float_value, expected_byte) in test_cases {
        let grey = Grey::<f32>::new(float_value);
        assert_eq!(grey.to_bytes(), expected_byte);
    }
}

// Test behavior with different float types.
#[test]
fn test_grey_bytes_different_float_types() {
    // Test with f32
    let grey_f32 = Grey::<f32>::new(0.333);
    let bytes_f32 = grey_f32.to_bytes();
    let expected_f32 = [85]; // 0.333 * 255 ≈ 84.915, rounds to 85
    assert_eq!(bytes_f32, expected_f32);

    // Test with f64
    let grey_f64 = Grey::<f64>::new(0.333);
    let bytes_f64 = grey_f64.to_bytes();
    let expected_f64 = [85]; // Same expected result as f32
    assert_eq!(bytes_f64, expected_f64);

    // Ensure both types yield the same result for the same input
    assert_eq!(bytes_f32, bytes_f64);
}

// Test edge case handling.
#[test]
fn test_grey_bytes_edge_cases() {
    // Test with very small positive value
    let grey = Grey::<f64>::new(1.0e-10);
    assert_eq!(grey.to_bytes(), [0]); // Should round to 0

    // Test with value very close to 1
    let grey = Grey::<f64>::new(1.0 - 1.0e-10);
    assert_eq!(grey.to_bytes(), [255]); // Should round to 255
}
