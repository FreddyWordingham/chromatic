# Chromatic

[![Crates.io](https://img.shields.io/crates/v/chromatic.svg)](https://crates.io/crates/chromatic)
[![Documentation](https://docs.rs/chromatic/badge.svg)](https://docs.rs/chromatic)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A comprehensive Rust library for working with colours across multiple colour spaces, featuring robust conversions, colour maps, and terminal visualization.

## ✨ Features

- **🎨 Complete colour space support**: RGB, sRGB, HSL, HSV, Lab, XYZ, and greyscale
- **🔍 Alpha channel variants**: All colour spaces include transparency support (RGBA, HSLA, etc.)
- **🧮 Generic numeric types**: Use any floating-point type (f32, f64, custom) as the underlying representation
- **🌈 Powerful colour maps**: Create and sample gradients with custom positioning and interpolation
- **🔄 Universal conversions**: Convert seamlessly between any supported colour spaces
- **🖥️ Terminal visualization**: Rich ANSI display support for visualizing colours and gradients
- **📐 Perceptual accuracy**: Delta E colour difference calculations and proper gamma handling
- **🛡️ Memory safe**: Zero unsafe code with comprehensive error handling
- **⚡ Performance focused**: Efficient algorithms with minimal allocations

## 📦 Installation

Add chromatic to your `Cargo.toml`:

```toml
[dependencies]
chromatic = "0.1.0"
```

## 🚀 Quick Start

```rust
use chromatic::{Colour, ColourMap, Rgb, Hsv, Lab, Convert};

// Create colours in different spaces
let red = Rgb::new(1.0, 0.0, 0.0)?;
let green = Rgb::new(0.0, 1.0, 0.0)?;
let blue = Rgb::new(0.0, 0.0, 1.0)?;

// Convert between colour spaces
let red_hsv = red.to_hsv()?;
let red_lab = red.to_lab()?;

// Create a colour map and sample from it
let colours = [red, green, blue];
let cmap = ColourMap::new(&colours)?;

let orange = cmap.sample(0.25)?; // Between red and green
let teal = cmap.sample(0.75)?;   // Between green and blue

// Display colours in terminal (24-bit colour support)
println!("🔴 Red: {}", red);
println!("🟠 Orange: {}", orange);
println!("🟢 Green: {}", green);
println!("🔵 Teal: {}", teal);
println!("🌈 Full gradient: {}", cmap);
```

## 🎨 Supported Colour Spaces

| Space    | Description                          | Components                          | Range              |
| -------- | ------------------------------------ | ----------------------------------- | ------------------ |
| **Grey** | Greyscale                            | Intensity                           | [0, 1]             |
| **Rgb**  | Linear RGB                           | Red, Green, Blue                    | [0, 1] each        |
| **Srgb** | Standard RGB (gamma-corrected)       | Red, Green, Blue                    | [0, 1] each        |
| **Hsl**  | Hue, Saturation, Lightness           | H: [0°, 360°), S,L: [0, 1]          | Cylindrical        |
| **Hsv**  | Hue, Saturation, Value               | H: [0°, 360°), S,V: [0, 1]          | Cylindrical        |
| **Lab**  | CIE L\*a\*b\* (perceptually uniform) | L\*: [0, 100], a\*,b\*: [-128, 127] | Perceptual         |
| **Xyz**  | CIE XYZ (device-independent)         | X,Y,Z: [0, 1]                       | Linear tristimulus |

Each colour space has an alpha variant (e.g., `RgbAlpha`, `HslAlpha`) for transparency support.

## 🌈 Advanced Colour Maps

Create sophisticated gradients with custom positioning:

```rust
use chromatic::{ColourMap, Hsv, Colour};

// Create a sunset gradient with custom positions
let sunset_colours = [
    Hsv::new(240.0, 0.8, 0.3)?, // Deep blue
    Hsv::new(280.0, 0.9, 0.5)?, // Purple
    Hsv::new(320.0, 1.0, 0.7)?, // Magenta
    Hsv::new(15.0, 1.0, 0.9)?,  // Orange
    Hsv::new(45.0, 0.8, 1.0)?,  // Yellow
];

let positions = [(sunset_colours[0], 0.0),
                 (sunset_colours[1], 0.2),
                 (sunset_colours[2], 0.4),
                 (sunset_colours[3], 0.7),
                 (sunset_colours[4], 1.0)];

let sunset_map = ColourMap::from_positions(&positions)?;

// Sample multiple colours for smooth transitions
let gradient_samples = sunset_map.sample_n(50)?;

// Create gradients with custom interpolation
let smooth_gradient = sunset_map.sample_with(0.5, |c1, c2, t| {
    // Custom interpolation logic
    Colour::lerp(c1, c2, t * t) // Ease-in quadratic
})?;
```

## 🔄 Colour Space Conversions

Seamless conversions between all supported colour spaces:

```rust
use chromatic::{Rgb, Hsv, Lab, Srgb, Convert};

let color = Rgb::new(0.8, 0.3, 0.6)?;

// Convert to different spaces
let hsv = color.to_hsv()?;        // For hue-based operations
let lab = color.to_lab()?;        // For perceptual calculations
let srgb = color.to_srgb()?;      // For display/web use
let xyz = color.to_xyz()?;        // For device-independent work

// String representations
let hex = color.to_hex()?;        // "#CC4D99"
let bytes = color.to_bytes()?;    // [204, 77, 153]

// Parse from various formats
let from_hex = Rgb::<f32>::from_hex("#CC4D99")?;
let from_bytes = Rgb::<f32>::from_bytes([204, 77, 153])?;
```

## 🎯 Perceptual Colour Operations

Calculate perceptually accurate colour differences and perform intelligent mixing:

```rust
use chromatic::{Lab, Rgb, Colour, Convert};

let color1 = Rgb::new(0.8, 0.2, 0.3)?;
let color2 = Rgb::new(0.7, 0.3, 0.4)?;

// Convert to Lab for perceptual accuracy
let lab1 = color1.to_lab()?;
let lab2 = color2.to_lab()?;

// Calculate colour differences
let delta_e76 = lab1.delta_e(&lab2);      // Basic Delta E
let delta_e94 = lab1.delta_e94(&lab2)?;   // Improved CIE94 formula

println!("Colour difference: {:.2}", delta_e94);
// < 1.0: Not perceptible
// 1-2: Perceptible with close observation
// 2-10: Perceptible at a glance
// > 10: Very different colours

// Advanced colour mixing
let mixed = Rgb::mix(&[color1, color2], &[0.7, 0.3])?;  // Weighted mix
let blended = Rgb::lerp(&color1, &color2, 0.5)?;        // 50/50 blend

// Create smooth gradients
let gradient = Rgb::gradient(&color1, &color2, 10)?;    // 10-step gradient
```

## 🖥️ Terminal Visualization

Rich terminal output with automatic colour detection:

```rust
use chromatic::{ColourMap, Rgb, Hsl};

// Create a vibrant rainbow
let rainbow = (0..7).map(|i| {
    Hsl::new(i as f32 * 60.0, 1.0, 0.6)
}).collect::<Result<Vec<_>, _>>()?;

let rainbow_map = ColourMap::new(&rainbow)?;

// Print colour map - automatically adapts to terminal width
println!("🌈 Rainbow: {}", rainbow_map);

// Individual colour blocks
for (i, color) in rainbow.iter().enumerate() {
    println!("Color {}: {}", i, color);
}

// Create data visualizations
let data = vec![0.1, 0.3, 0.7, 0.9, 0.4];
for (i, &value) in data.iter().enumerate() {
    let color = rainbow_map.sample(value)?;
    println!("Data point {}: {} (value: {:.1})", i, color, value);
}
```

## 🧮 Generic Type Support

Work with any floating-point precision:

```rust
use chromatic::{Rgb, Hsv, Colour};
use num_traits::Float;

// Memory-efficient f32
let color_f32 = Rgb::<f32>::new(1.0, 0.5, 0.0)?;

// High-precision f64
let color_f64 = Rgb::<f64>::new(1.0, 0.5, 0.0)?;

// Generic functions work with any float type
fn create_gradient<T>(start: Rgb<T>, end: Rgb<T>, steps: usize) -> Result<Vec<Rgb<T>>, chromatic::ChromaticError>
where
    T: Float + Send + Sync,
{
    Rgb::gradient(&start, &end, steps)
}

// Custom float types (requires Float trait implementation)
// let custom_color = Rgb::<YourCustomFloat>::new(...);
```

## 🎨 Practical Examples

### Creating a Heat Map

```rust
use chromatic::{ColourMap, Rgb};

// Create a temperature color map
let heat_colors = [
    Rgb::new(0.0, 0.0, 0.4)?, // Cold (dark blue)
    Rgb::new(0.0, 0.0, 1.0)?, // Blue
    Rgb::new(0.0, 1.0, 1.0)?, // Cyan
    Rgb::new(0.0, 1.0, 0.0)?, // Green
    Rgb::new(1.0, 1.0, 0.0)?, // Yellow
    Rgb::new(1.0, 0.5, 0.0)?, // Orange
    Rgb::new(1.0, 0.0, 0.0)?, // Hot (red)
];

let heat_map = ColourMap::new(&heat_colors)?;

// Map temperature data to colors
let temperatures = vec![15.2, 18.7, 22.1, 28.9, 31.4, 35.8];
let max_temp = 40.0;

for temp in temperatures {
    let normalized = temp / max_temp;
    let color = heat_map.sample(normalized)?;
    println!("🌡️  {:.1}°C: {}", temp, color);
}
```

### Web-Safe Color Palette

```rust
use chromatic::{Srgb, Convert};

// Generate web-safe colors
let base_color = Srgb::from_hex("#3498db")?;

// Create variations
let lighter = {
    let hsl = base_color.to_hsl()?;
    Hsl::new(hsl.hue(), hsl.saturation(), (hsl.lightness() + 0.2).min(1.0))?
        .to_srgb()?
};

let darker = {
    let hsl = base_color.to_hsl()?;
    Hsl::new(hsl.hue(), hsl.saturation(), (hsl.lightness() - 0.2).max(0.0))?
        .to_srgb()?
};

println!("Base color: {} ({})", base_color, base_color.to_hex()?);
println!("Lighter: {} ({})", lighter, lighter.to_hex()?);
println!("Darker: {} ({})", darker, darker.to_hex()?);
```

## 🔧 Error Handling

Chromatic uses a comprehensive error system for robust applications:

```rust
use chromatic::{Rgb, ChromaticError};

match Rgb::new(1.5, 0.0, 0.0) {  // Invalid: > 1.0
    Ok(color) => println!("Color: {}", color),
    Err(ChromaticError::InvalidColour(msg)) => {
        eprintln!("Invalid colour: {}", msg);
    }
    Err(e) => eprintln!("Other error: {}", e),
}

// Parsing errors
match Rgb::<f32>::from_hex("#ZZZZZZ") {
    Ok(color) => println!("Parsed: {}", color),
    Err(ChromaticError::ColourParsing(e)) => {
        eprintln!("Parse error: {}", e);
    }
    Err(e) => eprintln!("Other error: {}", e),
}
```

## 🚀 Performance Tips

- Use `f32` for memory-constrained applications
- Use `f64` for high-precision scientific applications
- Prefer Lab space for perceptual operations
- Use RGB/sRGB for display and web applications
- Cache ColourMap instances for repeated sampling
- Use `sample_n()` for bulk gradient generation

## 📚 API Documentation

For detailed API documentation, visit [docs.rs/chromatic](https://docs.rs/chromatic).

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

<div align="center">

**[📖 Documentation](https://docs.rs/chromatic) • [🐛 Report Bug](https://github.com/your-username/chromatic/issues) • [✨ Request Feature](https://github.com/your-username/chromatic/issues)**

</div>
