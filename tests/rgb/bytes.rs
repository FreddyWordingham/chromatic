use chromatic::{Colour, Rgb};

// Test basic functionality of from_bytes method.
#[test]
fn test_rgb_from_bytes() {
    // Test black
    let rgb = Rgb::<f32>::from_bytes([0, 0, 0]);
    assert_eq!(rgb.red(), 0.0);
    assert_eq!(rgb.green(), 0.0);
    assert_eq!(rgb.blue(), 0.0);

    // Test white
    let rgb = Rgb::<f32>::from_bytes([255, 255, 255]);
    assert_eq!(rgb.red(), 1.0);
    assert_eq!(rgb.green(), 1.0);
    assert_eq!(rgb.blue(), 1.0);

    // Test primary colors
    let rgb = Rgb::<f32>::from_bytes([255, 0, 0]);
    assert_eq!(rgb.red(), 1.0);
    assert_eq!(rgb.green(), 0.0);
    assert_eq!(rgb.blue(), 0.0);

    let rgb = Rgb::<f32>::from_bytes([0, 255, 0]);
    assert_eq!(rgb.red(), 0.0);
    assert_eq!(rgb.green(), 1.0);
    assert_eq!(rgb.blue(), 0.0);

    let rgb = Rgb::<f32>::from_bytes([0, 0, 255]);
    assert_eq!(rgb.red(), 0.0);
    assert_eq!(rgb.green(), 0.0);
    assert_eq!(rgb.blue(), 1.0);

    // Test middle values
    let rgb = Rgb::<f32>::from_bytes([127, 127, 127]);
    // Expected value is approximately 0.498... which is 127/255
    let expected = 127.0 / 255.0;
    assert!((rgb.red() - expected).abs() < Rgb::<f32>::tolerance());
    assert!((rgb.green() - expected).abs() < Rgb::<f32>::tolerance());
    assert!((rgb.blue() - expected).abs() < Rgb::<f32>::tolerance());

    // Test mixed values
    let rgb = Rgb::<f32>::from_bytes([50, 100, 200]);
    let expected_red = 50.0 / 255.0;
    let expected_green = 100.0 / 255.0;
    let expected_blue = 200.0 / 255.0;
    assert!((rgb.red() - expected_red).abs() < Rgb::<f32>::tolerance());
    assert!((rgb.green() - expected_green).abs() < Rgb::<f32>::tolerance());
    assert!((rgb.blue() - expected_blue).abs() < Rgb::<f32>::tolerance());
}

// Test basic functionality of to_bytes method.
#[test]
fn test_rgb_to_bytes() {
    // Test black
    let rgb = Rgb::<f32>::new(0.0, 0.0, 0.0);
    assert_eq!(rgb.to_bytes(), [0, 0, 0]);

    // Test white
    let rgb = Rgb::<f32>::new(1.0, 1.0, 1.0);
    assert_eq!(rgb.to_bytes(), [255, 255, 255]);

    // Test primary colors
    let rgb = Rgb::<f32>::new(1.0, 0.0, 0.0);
    assert_eq!(rgb.to_bytes(), [255, 0, 0]);

    let rgb = Rgb::<f32>::new(0.0, 1.0, 0.0);
    assert_eq!(rgb.to_bytes(), [0, 255, 0]);

    let rgb = Rgb::<f32>::new(0.0, 0.0, 1.0);
    assert_eq!(rgb.to_bytes(), [0, 0, 255]);

    // Test middle value
    let rgb = Rgb::<f32>::new(0.5, 0.5, 0.5);
    assert_eq!(rgb.to_bytes(), [128, 128, 128]);

    // Test mixed values
    let rgb = Rgb::<f32>::new(0.2, 0.4, 0.8);
    assert_eq!(rgb.to_bytes(), [51, 102, 204]); // 0.2*255=51, 0.4*255=102, 0.8*255=204
}

// Test round-trip conversion between bytes and float.
#[test]
fn test_rgb_bytes_round_trip() {
    // Test a selection of byte values
    let test_values = [
        [0, 0, 0],       // Black
        [255, 255, 255], // White
        [255, 0, 0],     // Red
        [0, 255, 0],     // Green
        [0, 0, 255],     // Blue
        [255, 255, 0],   // Yellow
        [0, 255, 255],   // Cyan
        [255, 0, 255],   // Magenta
        [128, 128, 128], // Mid-gray
        [64, 128, 192],  // Some mixed values
    ];

    for original_bytes in test_values {
        let rgb = Rgb::<f32>::from_bytes(original_bytes);
        let round_trip_bytes = rgb.to_bytes();

        assert_eq!(original_bytes, round_trip_bytes);
    }
}

// Test round-trip conversion between float and bytes.
#[test]
fn test_rgb_float_round_trip() {
    // Test a range of float values that should map precisely to bytes
    for r in 0..=4 {
        for g in 0..=4 {
            for b in 0..=4 {
                let red_value = (r * 63) as f32 / 255.0;
                let green_value = (g * 63) as f32 / 255.0;
                let blue_value = (b * 63) as f32 / 255.0;

                let rgb = Rgb::<f32>::new(red_value, green_value, blue_value);
                let bytes = rgb.to_bytes();
                let round_trip_rgb = Rgb::<f32>::from_bytes(bytes);

                // The round-trip should preserve the value within tolerance
                assert_eq!(rgb, round_trip_rgb);
            }
        }
    }
}

// Test rounding behavior in to_bytes.
#[test]
fn test_rgb_to_bytes_rounding() {
    // Test values just below and above the threshold for rounding
    let tests = [
        // (red, green, blue, expected_red_byte, expected_green_byte, expected_blue_byte)
        (0.5 - 0.001, 0.5 - 0.001, 0.5 - 0.001, 127, 127, 127),
        (0.5 + 0.001, 0.5 + 0.001, 0.5 + 0.001, 128, 128, 128),
        (0.1 - 0.001, 0.5, 0.9 + 0.001, 25, 128, 230),
        (0.1 + 0.001, 0.5, 0.9 - 0.001, 26, 128, 229),
    ];

    for (red, green, blue, expected_red_byte, expected_green_byte, expected_blue_byte) in tests {
        let rgb = Rgb::<f32>::new(red, green, blue);
        let bytes = rgb.to_bytes();
        assert_eq!(bytes, [expected_red_byte, expected_green_byte, expected_blue_byte]);
    }
}

// Test behavior with different float types.
#[test]
fn test_rgb_bytes_different_float_types() {
    // Test with f32
    let rgb_f32 = Rgb::<f32>::new(0.333, 0.667, 0.5);
    let bytes_f32 = rgb_f32.to_bytes();
    let expected_f32 = [85, 170, 128]; // 0.333 * 255 ≈ 85, 0.667 * 255 ≈ 170, 0.5 * 255 = 128
    assert_eq!(bytes_f32, expected_f32);

    // Test with f64
    let rgb_f64 = Rgb::<f64>::new(0.333, 0.667, 0.5);
    let bytes_f64 = rgb_f64.to_bytes();
    let expected_f64 = [85, 170, 128]; // Same expected result as f32
    assert_eq!(bytes_f64, expected_f64);

    // Ensure both types yield the same result for the same input
    assert_eq!(bytes_f32, bytes_f64);
}

// Test conversion of common named colors
#[test]
fn test_rgb_bytes_named_colors() {
    // Test common color names and their RGB values
    let colors = [
        // (R, G, B, name)
        ([255, 0, 0], "red"),
        ([0, 255, 0], "green"),
        ([0, 0, 255], "blue"),
        ([255, 255, 0], "yellow"),
        ([255, 0, 255], "magenta"),
        ([0, 255, 255], "cyan"),
        ([0, 0, 0], "black"),
        ([255, 255, 255], "white"),
        ([128, 128, 128], "gray"),
        ([192, 192, 192], "silver"),
        ([128, 0, 0], "maroon"),
        ([128, 128, 0], "olive"),
        ([0, 128, 0], "dark green"),
        ([0, 0, 128], "navy"),
        ([128, 0, 128], "purple"),
    ];

    for (bytes, name) in colors {
        let rgb = Rgb::<f32>::from_bytes(bytes);
        let round_trip = rgb.to_bytes();

        assert_eq!(round_trip, bytes, "Failed for color: {}", name);

        // Also verify the expected float values
        let expected_red = bytes[0] as f32 / 255.0;
        let expected_green = bytes[1] as f32 / 255.0;
        let expected_blue = bytes[2] as f32 / 255.0;

        assert!(
            (rgb.red() - expected_red).abs() < Rgb::<f32>::tolerance(),
            "Failed red component for color: {}",
            name
        );
        assert!(
            (rgb.green() - expected_green).abs() < Rgb::<f32>::tolerance(),
            "Failed green component for color: {}",
            name
        );
        assert!(
            (rgb.blue() - expected_blue).abs() < Rgb::<f32>::tolerance(),
            "Failed blue component for color: {}",
            name
        );
    }
}

// Test edge cases with extreme values
#[test]
fn test_rgb_bytes_extreme_values() {
    // Test all combinations of min (0) and max (255) values
    let extremes = [
        ([0, 0, 0], [0.0, 0.0, 0.0]),       // black
        ([255, 0, 0], [1.0, 0.0, 0.0]),     // red
        ([0, 255, 0], [0.0, 1.0, 0.0]),     // green
        ([0, 0, 255], [0.0, 0.0, 1.0]),     // blue
        ([255, 255, 0], [1.0, 1.0, 0.0]),   // yellow
        ([255, 0, 255], [1.0, 0.0, 1.0]),   // magenta
        ([0, 255, 255], [0.0, 1.0, 1.0]),   // cyan
        ([255, 255, 255], [1.0, 1.0, 1.0]), // white
    ];

    for (bytes, floats) in extremes {
        let rgb = Rgb::<f32>::from_bytes(bytes);

        assert_eq!(rgb.red(), floats[0]);
        assert_eq!(rgb.green(), floats[1]);
        assert_eq!(rgb.blue(), floats[2]);

        let bytes_result = rgb.to_bytes();
        assert_eq!(bytes_result, bytes);
    }
}
