# chromatic

A small utility library for building and sampling colour maps in Rust.

## Features

- Support for various colour types: Grey, GreyAlpha, RGB, RGBA
- Linear interpolation between colours
- Parsing colours from hex strings
- Type-flexible with support for different numeric representations (u8, u16, f32, etc.)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
chromatic = "0.0.0"
```

Basic example:

```rust
use chromatic::{Rgb, Colour};
use std::str::FromStr;

fn main() {
    // Create colours from hex strings
    let red = Rgb::<f32>::from_str("#FF0000").unwrap();
    let blue = Rgb::<f32>::from_str("#0000FF").unwrap();

    // Linear interpolation
    let purple = red.lerp(&blue, 0.5);

    println!("Purple RGB: ({}, {}, {})", purple.r(), purple.g(), purple.b());
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
