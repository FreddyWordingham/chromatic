use chromatic::{Colour, Rgba};

// Test basic functionality of from_bytes method.
#[test]
fn test_rgba_from_bytes() {
    // Test black transparent
    let rgba = Rgba::<f32>::from_bytes([0, 0, 0, 0]);
    assert_eq!(rgba.red(), 0.0);
    assert_eq!(rgba.green(), 0.0);
    assert_eq!(rgba.blue(), 0.0);
    assert_eq!(rgba.alpha(), 0.0);

    // Test white opaque
    let rgba = Rgba::<f32>::from_bytes([255, 255, 255, 255]);
    assert_eq!(rgba.red(), 1.0);
    assert_eq!(rgba.green(), 1.0);
    assert_eq!(rgba.blue(), 1.0);
    assert_eq!(rgba.alpha(), 1.0);

    // Test primary colours with full opacity
    let rgba = Rgba::<f32>::from_bytes([255, 0, 0, 255]);
    assert_eq!(rgba.red(), 1.0);
    assert_eq!(rgba.green(), 0.0);
    assert_eq!(rgba.blue(), 0.0);
    assert_eq!(rgba.alpha(), 1.0);

    let rgba = Rgba::<f32>::from_bytes([0, 255, 0, 255]);
    assert_eq!(rgba.red(), 0.0);
    assert_eq!(rgba.green(), 1.0);
    assert_eq!(rgba.blue(), 0.0);
    assert_eq!(rgba.alpha(), 1.0);

    let rgba = Rgba::<f32>::from_bytes([0, 0, 255, 255]);
    assert_eq!(rgba.red(), 0.0);
    assert_eq!(rgba.green(), 0.0);
    assert_eq!(rgba.blue(), 1.0);
    assert_eq!(rgba.alpha(), 1.0);

    // Test various alpha values
    let rgba = Rgba::<f32>::from_bytes([128, 128, 128, 128]);
    let expected = 128.0 / 255.0;
    assert!((rgba.red() - expected).abs() < Rgba::<f32>::tolerance());
    assert!((rgba.green() - expected).abs() < Rgba::<f32>::tolerance());
    assert!((rgba.blue() - expected).abs() < Rgba::<f32>::tolerance());
    assert!((rgba.alpha() - expected).abs() < Rgba::<f32>::tolerance());

    // Test mixed values with transparency
    let rgba = Rgba::<f32>::from_bytes([50, 100, 200, 150]);
    let expected_red = 50.0 / 255.0;
    let expected_green = 100.0 / 255.0;
    let expected_blue = 200.0 / 255.0;
    let expected_alpha = 150.0 / 255.0;
    assert!((rgba.red() - expected_red).abs() < Rgba::<f32>::tolerance());
    assert!((rgba.green() - expected_green).abs() < Rgba::<f32>::tolerance());
    assert!((rgba.blue() - expected_blue).abs() < Rgba::<f32>::tolerance());
    assert!((rgba.alpha() - expected_alpha).abs() < Rgba::<f32>::tolerance());
}

// Test basic functionality of to_bytes method.
#[test]
fn test_rgba_to_bytes() {
    // Test black transparent
    let rgba = Rgba::<f32>::new(0.0, 0.0, 0.0, 0.0);
    assert_eq!(rgba.to_bytes(), [0, 0, 0, 0]);

    // Test white opaque
    let rgba = Rgba::<f32>::new(1.0, 1.0, 1.0, 1.0);
    assert_eq!(rgba.to_bytes(), [255, 255, 255, 255]);

    // Test primary colours with opacity
    let rgba = Rgba::<f32>::new(1.0, 0.0, 0.0, 1.0);
    assert_eq!(rgba.to_bytes(), [255, 0, 0, 255]);

    let rgba = Rgba::<f32>::new(0.0, 1.0, 0.0, 0.5);
    assert_eq!(rgba.to_bytes(), [0, 255, 0, 128]);

    let rgba = Rgba::<f32>::new(0.0, 0.0, 1.0, 0.75);
    assert_eq!(rgba.to_bytes(), [0, 0, 255, 191]); // 0.75 * 255 ~= 191

    // Test middle value
    let rgba = Rgba::<f32>::new(0.5, 0.5, 0.5, 0.5);
    assert_eq!(rgba.to_bytes(), [128, 128, 128, 128]);

    // Test mixed values
    let rgba = Rgba::<f32>::new(0.2, 0.4, 0.8, 0.6);
    assert_eq!(rgba.to_bytes(), [51, 102, 204, 153]); // 0.2*255=51, 0.4*255=102, 0.8*255=204, 0.6*255=153
}

// Test round-trip conversion between bytes and float.
#[test]
fn test_rgba_bytes_round_trip() {
    // Test a selection of byte values
    let test_values = [
        [0, 0, 0, 0],         // Black transparent
        [255, 255, 255, 255], // White opaque
        [255, 0, 0, 255],     // Red opaque
        [0, 255, 0, 255],     // Green opaque
        [0, 0, 255, 255],     // Blue opaque
        [255, 255, 0, 128],   // Yellow semi-transparent
        [0, 255, 255, 64],    // Cyan mostly transparent
        [255, 0, 255, 192],   // Magenta mostly opaque
        [128, 128, 128, 128], // Mid-gray mid-transparent
        [64, 128, 192, 240],  // Some mixed values
    ];

    for original_bytes in test_values {
        let rgba = Rgba::<f32>::from_bytes(original_bytes);
        let round_trip_bytes = rgba.to_bytes();

        assert_eq!(original_bytes, round_trip_bytes);
    }
}

// Test round-trip conversion between float and bytes.
#[test]
fn test_rgba_float_round_trip() {
    // Test a range of float values that should map precisely to bytes
    for r in 0..=4 {
        for g in 0..=4 {
            for b in 0..=4 {
                for a in 0..=4 {
                    let red_value = (r * 63) as f32 / 255.0;
                    let green_value = (g * 63) as f32 / 255.0;
                    let blue_value = (b * 63) as f32 / 255.0;
                    let alpha_value = (a * 63) as f32 / 255.0;

                    let rgba = Rgba::<f32>::new(red_value, green_value, blue_value, alpha_value);
                    let bytes = rgba.to_bytes();
                    let round_trip_rgba = Rgba::<f32>::from_bytes(bytes);

                    // The round-trip should preserve the value within tolerance
                    assert_eq!(rgba, round_trip_rgba);
                }
            }
        }
    }
}

// Test rounding behavior in to_bytes.
#[test]
fn test_rgba_to_bytes_rounding() {
    // Test values just below and above the threshold for rounding
    let tests = [
        // (red, green, blue, alpha, expected_red_byte, expected_green_byte, expected_blue_byte, expected_alpha_byte)
        (0.5 - 0.001, 0.5 - 0.001, 0.5 - 0.001, 0.5 - 0.001, 127, 127, 127, 127),
        (0.5 + 0.001, 0.5 + 0.001, 0.5 + 0.001, 0.5 + 0.001, 128, 128, 128, 128),
        (0.1 - 0.001, 0.5, 0.9 + 0.001, 0.3, 25, 128, 230, 77),
        (0.1 + 0.001, 0.5, 0.9 - 0.001, 0.7, 26, 128, 229, 179),
    ];

    for (red, green, blue, alpha, expected_red_byte, expected_green_byte, expected_blue_byte, expected_alpha_byte) in tests {
        let rgba = Rgba::<f32>::new(red, green, blue, alpha);
        let bytes = rgba.to_bytes();
        assert_eq!(
            bytes,
            [
                expected_red_byte,
                expected_green_byte,
                expected_blue_byte,
                expected_alpha_byte
            ]
        );
    }
}

// Test behavior with different float types.
#[test]
fn test_rgba_bytes_different_float_types() {
    // Test with f32
    let rgba_f32 = Rgba::<f32>::new(0.333, 0.667, 0.5, 0.25);
    let bytes_f32 = rgba_f32.to_bytes();
    let expected_f32 = [85, 170, 128, 64]; // 0.333 * 255 ≈ 85, 0.667 * 255 ≈ 170, 0.5 * 255 = 128, 0.25 * 255 = 64
    assert_eq!(bytes_f32, expected_f32);

    // Test with f64
    let rgba_f64 = Rgba::<f64>::new(0.333, 0.667, 0.5, 0.25);
    let bytes_f64 = rgba_f64.to_bytes();
    let expected_f64 = [85, 170, 128, 64]; // Same expected result as f32
    assert_eq!(bytes_f64, expected_f64);

    // Ensure both types yield the same result for the same input
    assert_eq!(bytes_f32, bytes_f64);
}

// Test conversion of common named colours with alpha
#[test]
fn test_rgba_bytes_named_colours() {
    // Test common colour names and their RGBA values
    let colours = [
        // (R, G, B, A, name)
        ([255, 0, 0, 255], "red (opaque)"),
        ([0, 255, 0, 255], "green (opaque)"),
        ([0, 0, 255, 255], "blue (opaque)"),
        ([255, 255, 0, 128], "yellow (semi-transparent)"),
        ([255, 0, 255, 128], "magenta (semi-transparent)"),
        ([0, 255, 255, 128], "cyan (semi-transparent)"),
        ([0, 0, 0, 0], "black (transparent)"),
        ([255, 255, 255, 255], "white (opaque)"),
        ([128, 128, 128, 64], "gray (mostly transparent)"),
        ([192, 192, 192, 192], "silver (semi-transparent)"),
        ([128, 0, 0, 255], "maroon (opaque)"),
        ([128, 128, 0, 255], "olive (opaque)"),
        ([0, 128, 0, 255], "dark green (opaque)"),
        ([0, 0, 128, 255], "navy (opaque)"),
        ([128, 0, 128, 255], "purple (opaque)"),
    ];

    for (bytes, name) in colours {
        let rgba = Rgba::<f32>::from_bytes(bytes);
        let round_trip = rgba.to_bytes();

        assert_eq!(round_trip, bytes, "Failed for colour: {}", name);

        // Also verify the expected float values
        let expected_red = bytes[0] as f32 / 255.0;
        let expected_green = bytes[1] as f32 / 255.0;
        let expected_blue = bytes[2] as f32 / 255.0;
        let expected_alpha = bytes[3] as f32 / 255.0;

        assert!(
            (rgba.red() - expected_red).abs() < Rgba::<f32>::tolerance(),
            "Failed red component for colour: {}",
            name
        );
        assert!(
            (rgba.green() - expected_green).abs() < Rgba::<f32>::tolerance(),
            "Failed green component for colour: {}",
            name
        );
        assert!(
            (rgba.blue() - expected_blue).abs() < Rgba::<f32>::tolerance(),
            "Failed blue component for colour: {}",
            name
        );
        assert!(
            (rgba.alpha() - expected_alpha).abs() < Rgba::<f32>::tolerance(),
            "Failed alpha component for colour: {}",
            name
        );
    }
}

// Test edge cases with extreme values
#[test]
fn test_rgba_bytes_extreme_values() {
    // Test all combinations of min (0) and max (255) values
    let extremes = [
        ([0, 0, 0, 0], [0.0, 0.0, 0.0, 0.0]),         // black transparent
        ([255, 0, 0, 0], [1.0, 0.0, 0.0, 0.0]),       // red transparent
        ([0, 255, 0, 0], [0.0, 1.0, 0.0, 0.0]),       // green transparent
        ([0, 0, 255, 0], [0.0, 0.0, 1.0, 0.0]),       // blue transparent
        ([0, 0, 0, 255], [0.0, 0.0, 0.0, 1.0]),       // black opaque
        ([255, 255, 0, 0], [1.0, 1.0, 0.0, 0.0]),     // yellow transparent
        ([255, 0, 255, 0], [1.0, 0.0, 1.0, 0.0]),     // magenta transparent
        ([0, 255, 255, 0], [0.0, 1.0, 1.0, 0.0]),     // cyan transparent
        ([255, 0, 0, 255], [1.0, 0.0, 0.0, 1.0]),     // red opaque
        ([0, 255, 0, 255], [0.0, 1.0, 0.0, 1.0]),     // green opaque
        ([0, 0, 255, 255], [0.0, 0.0, 1.0, 1.0]),     // blue opaque
        ([255, 255, 0, 255], [1.0, 1.0, 0.0, 1.0]),   // yellow opaque
        ([255, 0, 255, 255], [1.0, 0.0, 1.0, 1.0]),   // magenta opaque
        ([0, 255, 255, 255], [0.0, 1.0, 1.0, 1.0]),   // cyan opaque
        ([255, 255, 255, 255], [1.0, 1.0, 1.0, 1.0]), // white opaque
    ];

    for (bytes, floats) in extremes {
        let rgba = Rgba::<f32>::from_bytes(bytes);

        assert_eq!(rgba.red(), floats[0]);
        assert_eq!(rgba.green(), floats[1]);
        assert_eq!(rgba.blue(), floats[2]);
        assert_eq!(rgba.alpha(), floats[3]);

        let bytes_result = rgba.to_bytes();
        assert_eq!(bytes_result, bytes);
    }
}
