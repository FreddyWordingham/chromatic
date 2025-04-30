use chromatic::{ColourMap, Rgb};
use core::str::FromStr;

fn main() {
    // Create colors
    let black = Rgb::from_str("#000000").unwrap();
    let red = Rgb::from_str("#ff0000").unwrap();
    let yellow = Rgb::from_str("#ffff00").unwrap();
    let white = Rgb::from_str("#ffffff").unwrap();

    // Create a color map
    let map = ColourMap::new(&[black, red, yellow, white], &[0.0, 0.3, 0.7, 1.0]);

    // Sample the map at different positions
    let color1 = map.sample(0.0); // black
    println!("Color 1: {}", color1);
    let color2 = map.sample(0.5); // somewhere between red and yellow
    println!("Color 2: {}", color2);
    let color3 = map.sample(1.0); // white
    println!("Color 3: {}", color3);
}
