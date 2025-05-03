# Chromatic

Chromatic is a Rust library for building and sampling colour maps with perceptually uniform interpolation. It provides a flexible, type-safe way to work with various colour representations and create smooth transitions between colours.

## Features

- Multiple colour space representations:
  - Basic colour types: `Rgb`, `Rgba`, `Grey`, `GreyAlpha`
  - Perceptually uniform colour types: `LabRgb`, `LabRgba` (using CIE Lab colour space)
- Flexible `ColourMap` for interpolation between multiple colours
- Perceptually uniform colour mixing with Lab colour space
- Type-safe conversions between colour spaces
- String parsing and formatting (hex notation and comma-separated values)
- Generic implementation with `Float` trait support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
chromatic = "0.1.0"
```

## Examples

### Creating and Using Colors

```rust
use chromatic::{Rgb, Rgba, LabRgb, Grey, Colour};

// Create RGB colours
let red = Rgb::new(1.0, 0.0, 0.0);
let blue = Rgb::new(0.0, 0.0, 1.0);

// Create with alpha
let transparent_green = Rgba::new(0.0, 1.0, 0.0, 0.5);

// Parse from string (hex notation)
let purple = Rgb::<f32>::from_hex("#800080").unwrap();

// Convert between colour spaces
let grey = red.to_grey();
let lab_red = red.to_lab_rgb(); // Better for interpolation
```

### Creating a Color Map

```rust
use chromatic::{ColourMap, Rgb};

// Create colours
let red = Rgb::new(1.0, 0.0, 0.0);
let green = Rgb::new(0.0, 1.0, 0.0);
let blue = Rgb::new(0.0, 0.0, 1.0);

// Create a colour map with positions
let colours = vec![red, green, blue];
let positions = vec![0.0, 0.5, 1.0];
let colour_map = ColourMap::new(&colours, &positions);

// Sample the colour map
let middle = colour_map.sample(0.25); // Returns a colour between red and green
```

### Using Perceptually Uniform Interpolation

```rust
use chromatic::{LabRgb, Colour};

// Create colours in Lab space for better interpolation
let yellow = LabRgb::new(1.0, 1.0, 0.0);
let blue = LabRgb::new(0.0, 0.0, 1.0);

// Interpolate in Lab colour space (perceptually uniform)
let mixed = LabRgb::lerp(&yellow, &blue, 0.5);
```
