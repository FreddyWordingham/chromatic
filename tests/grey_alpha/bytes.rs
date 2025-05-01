use chromatic::{Colour, GreyAlpha};

// Test basic functionality of from_bytes method.
#[test]
fn test_grey_alpha_from_bytes() {
    // Test black, fully transparent
    let grey_alpha = GreyAlpha::<f32>::from_bytes([0, 0]);
    assert_eq!(grey_alpha.grey(), 0.0);
    assert_eq!(grey_alpha.alpha(), 0.0);

    // Test white, fully opaque
    let grey_alpha = GreyAlpha::<f32>::from_bytes([255, 255]);
    assert_eq!(grey_alpha.grey(), 1.0);
    assert_eq!(grey_alpha.alpha(), 1.0);

    // Test middle values
    let grey_alpha = GreyAlpha::<f32>::from_bytes([127, 127]);
    // Expected value is approximately 0.498... which is 127/255
    let expected = 127.0 / 255.0;
    assert!((grey_alpha.grey() - expected).abs() < GreyAlpha::<f32>::tolerance());
    assert!((grey_alpha.alpha() - expected).abs() < GreyAlpha::<f32>::tolerance());

    // Test mixed values
    let grey_alpha = GreyAlpha::<f32>::from_bytes([50, 200]);
    let expected_grey = 50.0 / 255.0;
    let expected_alpha = 200.0 / 255.0;
    assert!((grey_alpha.grey() - expected_grey).abs() < GreyAlpha::<f32>::tolerance());
    assert!((grey_alpha.alpha() - expected_alpha).abs() < GreyAlpha::<f32>::tolerance());
}

// Test basic functionality of to_bytes method.
#[test]
fn test_grey_alpha_to_bytes() {
    // Test black, fully transparent
    let grey_alpha = GreyAlpha::<f32>::new(0.0, 0.0);
    assert_eq!(grey_alpha.to_bytes(), [0, 0]);

    // Test white, fully opaque
    let grey_alpha = GreyAlpha::<f32>::new(1.0, 1.0);
    assert_eq!(grey_alpha.to_bytes(), [255, 255]);

    // Test middle value
    let grey_alpha = GreyAlpha::<f32>::new(0.5, 0.5);
    assert_eq!(grey_alpha.to_bytes(), [128, 128]);

    // Test mixed values
    let grey_alpha = GreyAlpha::<f32>::new(0.2, 0.8);
    assert_eq!(grey_alpha.to_bytes(), [51, 204]); // 0.2*255=51, 0.8*255=204
}

// Test round-trip conversion between bytes and float.
#[test]
fn test_grey_alpha_bytes_round_trip() {
    // Test a selection of byte values
    let test_values = [
        [0, 0],     // Black, transparent
        [255, 255], // White, opaque
        [128, 128], // Mid-grey, mid-alpha
        [64, 192],  // Dark grey, mostly opaque
        [192, 64],  // Light grey, mostly transparent
    ];

    for original_bytes in test_values {
        let grey_alpha = GreyAlpha::<f32>::from_bytes(original_bytes);
        let round_trip_bytes = grey_alpha.to_bytes();

        assert_eq!(original_bytes, round_trip_bytes);
    }
}

// Test round-trip conversion between float and bytes.
#[test]
fn test_grey_alpha_float_round_trip() {
    // Test a range of float values that should map precisely to bytes
    for i in 0..=4 {
        for j in 0..=4 {
            let grey_value = (i * 63) as f32 / 255.0;
            let alpha_value = (j * 63) as f32 / 255.0;

            let grey_alpha = GreyAlpha::<f32>::new(grey_value, alpha_value);
            let bytes = grey_alpha.to_bytes();
            let round_trip_grey_alpha = GreyAlpha::<f32>::from_bytes(bytes);

            // The round-trip should preserve the value within tolerance
            assert_eq!(grey_alpha, round_trip_grey_alpha);
        }
    }
}

// Test rounding behavior in to_bytes.
#[test]
fn test_grey_alpha_to_bytes_rounding() {
    // Test values just below and above the threshold for rounding
    let tests = [
        // (grey, alpha, expected_grey_byte, expected_alpha_byte)
        (0.5 - 0.001, 0.5 - 0.001, 127, 127),
        (0.5 + 0.001, 0.5 + 0.001, 128, 128),
        (0.1 - 0.001, 0.9 + 0.001, 25, 230),
        (0.1 + 0.001, 0.9 - 0.001, 26, 229),
    ];

    for (grey, alpha, expected_grey_byte, expected_alpha_byte) in tests {
        let grey_alpha = GreyAlpha::<f32>::new(grey, alpha);
        let bytes = grey_alpha.to_bytes();
        assert_eq!(bytes, [expected_grey_byte, expected_alpha_byte]);
    }
}

// Test behavior with different float types.
#[test]
fn test_grey_alpha_bytes_different_float_types() {
    // Test with f32
    let grey_alpha_f32 = GreyAlpha::<f32>::new(0.333, 0.667);
    let bytes_f32 = grey_alpha_f32.to_bytes();
    let expected_f32 = [85, 170]; // 0.333 * 255 ≈ 85, 0.667 * 255 ≈ 170
    assert_eq!(bytes_f32, expected_f32);

    // Test with f64
    let grey_alpha_f64 = GreyAlpha::<f64>::new(0.333, 0.667);
    let bytes_f64 = grey_alpha_f64.to_bytes();
    let expected_f64 = [85, 170]; // Same expected result as f32
    assert_eq!(bytes_f64, expected_f64);

    // Ensure both types yield the same result for the same input
    assert_eq!(bytes_f32, bytes_f64);
}
