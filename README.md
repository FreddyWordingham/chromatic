# Chromatic

[![Crates.io](https://img.shields.io/crates/v/chromatic.svg)](https://crates.io/crates/chromatic)
[![Documentation](https://docs.rs/chromatic/badge.svg)](https://docs.rs/chromatic)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A comprehensive Rust library with a straightforward API for working with multiple colour spaces.

## Features

- **Extensive colour space support**: RGB, sRGB, HSL, HSV, Lab, XYZ, and greyscale
- **Alpha channel variants**: All colour spaces have alpha channel counterparts (e.g., RGBA, HSLA)
- **Generic type representation**: Use any floating-point type as the underlying representation (f32, f64, etc.)
- **Flexible colourmap creation**: Build and sample from colour gradients with any colour space
- **Rich conversion system**: Convert between any supported colour spaces
- **Terminal output**: Display colours directly in terminals that support 24-bit colour
- **Format flexibility**: Convert to/from hex strings and byte arrays
- **Complete colour handling**: Linear interpolation, perceptual colour differences, proper gamma handling for sRGB

## Installation

Add chromatic to your `Cargo.toml`:

```toml
[dependencies]
chromatic = "0.1.0"
num-traits = "0.2"
```

## Quick Start

```rust
use chromatic::{Colour, ColourMap, Rgb, Hsv, Lab, Xyz, Convert};
use num_traits::Float;

// Create colours in different spaces
let red = Rgb::new(1.0, 0.0, 0.0);
let green = Rgb::new(0.0, 1.0, 0.0);
let blue = Rgb::new(0.0, 0.0, 1.0);

// Convert between colour spaces
let red_hsv = red.to_hsv();
let red_lab = red.to_lab();

// Create a colour map
let colours = [red, green, blue];
let positions = [0.0, 0.5, 1.0];
let cmap = ColourMap::<Rgb<f32>, f32, 3>::new(&colours, &positions);

// Sample colours from the map
let orange = cmap.sample(0.25); // Between red and green
let teal = cmap.sample(0.75);   // Between green and blue

// Display colours in terminal
println!("Red: {}", red);
println!("Orange: {}", orange);
println!("Green: {}", green);
println!("Teal: {}", teal);
println!("Blue: {}", blue);
println!("Colour map: {}", cmap);
```

## Colour Spaces

Chromatic provides the following colour spaces:

| Space | Description            | Components                 |
| ----- | ---------------------- | -------------------------- |
| Grey  | Greyscale              | Monochromaic intensity     |
| Rgb   | Linear RGB             | Red, Green, Blue           |
| Srgb  | sRGB (gamma-corrected) | Red, Green, Blue           |
| Hsl   | HSL                    | Hue, Saturation, Lightness |
| Hsv   | HSV                    | Hue, Saturation, Value     |
| Lab   | CIE L*a*b\*            | Lightness, a*, b*          |
| Xyz   | CIE XYZ                | X, Y, Z                    |

Each colour space also has an alpha variant (e.g., GreyAlpha, RgbAlpha, etc.).

## Working with Colour Maps

Colour maps allow you to create gradients between multiple colours:

```rust
use chromatic::{Colour, ColourMap, Hsv};

// Create a rainbow colour map in HSV space
let rainbow = [
    Hsv::new(0.0, 1.0, 1.0),    // Red
    Hsv::new(60.0, 1.0, 1.0),   // Yellow
    Hsv::new(120.0, 1.0, 1.0),  // Green
    Hsv::new(180.0, 1.0, 1.0),  // Cyan
    Hsv::new(240.0, 1.0, 1.0),  // Blue
    Hsv::new(300.0, 1.0, 1.0),  // Magenta
    Hsv::new(360.0, 1.0, 1.0),  // Red again
];

// Create a uniformly spaced colour map
let cmap = ColourMap::<Hsv<f32>, f32, 3>::new_uniform(&rainbow);

// Print the colour map to see the gradient
println!("{}", cmap);

// Sample a colour from the map
let sampled_colour = cmap.sample(0.5);
```

## Colour Conversions

All colours can be converted between spaces using the `Convert` trait:

```rust
use chromatic::{Colour, Convert, Rgb, Hsv, Lab};

let red = Rgb::new(1.0, 0.0, 0.0);

// Convert to other spaces
let red_hsv = red.to_hsv();
let red_lab = red.to_lab();

// Convert to string representation
let hex = red.to_hex();
println!("Red in hex: {}", hex); // #FF0000

// Parse colour from hex
let parsed_red = Rgb::<f32>::from_hex("#FF0000").unwrap();
```

## Perceptual Colour Differences

Chromatic includes methods to calculate perceptual colour differences:

```rust
use chromatic::{Lab, Convert, Rgb};

let colour1 = Rgb::new(0.8, 0.3, 0.4);
let colour2 = Rgb::new(0.7, 0.4, 0.45);

// Convert to Lab for perceptual difference calculation
let lab1 = colour1.to_lab();
let lab2 = colour2.to_lab();

// Calculate perceptual colour difference using Delta E
let basic_delta_e = lab1.delta_e(&lab2);
let improved_delta_e = lab1.delta_e94(&lab2);

println!("Basic Delta E: {}", basic_delta_e);
println!("Improved Delta E (CIE94): {}", improved_delta_e);
```

## Terminal Visualization

All colour types implement `Display` and can be directly printed in terminals that support 24-bit colour:

```rust
use chromatic::{ColourMap, Rgb};

// Create a simple gradient from black to white
let black = Rgb::new(0.0, 0.0, 0.0);
let white = Rgb::new(1.0, 1.0, 1.0);
let gradient = [black, white];

let cmap = ColourMap::<Rgb<f64>, f64, 3>::new_uniform(&gradient);

// Print the entire colour map to visualize the gradient
println!("{}", cmap);
```

## Generic Type Support

Chromatic supports any floating-point type that implements the `Float` trait from `num_traits`:

```rust
use chromatic::{Rgb, Hsv, Convert};
use num_traits::Float;

// Use f32 for memory efficiency
let red_f32 = Rgb::<f32>::new(1.0, 0.0, 0.0);

// Use f64 for higher precision
let red_f64 = Rgb::<f64>::new(1.0, 0.0, 0.0);

// Function that works with any float type
fn blend_colours<T: Float + Send + Sync>(colour1: &Rgb<T>, colour2: &Rgb<T>, factor: T) -> Rgb<T> {
    Rgb::lerp(colour1, colour2, factor)
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
