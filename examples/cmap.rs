use chromatic::{Colour, ColourMap, Rgb};

fn main() {
    let cmap = ColourMap::new_uniform(&[
        Rgb::new(0.0, 0.0, 0.5), // dark blue
        Rgb::new(0.0, 0.0, 1.0), // blue
        Rgb::new(0.0, 1.0, 1.0), // cyan
        Rgb::new(0.0, 1.0, 0.0), // green
        Rgb::new(1.0, 1.0, 0.0), // yellow
        Rgb::new(1.0, 0.0, 0.0), // red
        Rgb::new(0.5, 0.0, 0.0), // dark red
    ]);

    for i in 0..=100 {
        let pos = i as f32 / 100.0;
        let colour = cmap.sample(pos);
        println!("{} {}", colour, colour.to_hex());
    }

    println!("{}", cmap);
}
