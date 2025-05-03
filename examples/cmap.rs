use chromatic::{Colour, ColourMap, Hsv};

fn main() {
    let cmap = ColourMap::new_uniform(&[
        Hsv::new(0.0, 1.0, 1.0),
        Hsv::new(90.0, 1.0, 1.0),
        Hsv::new(180.0, 1.0, 1.0),
        Hsv::new(270.0, 1.0, 1.0),
        Hsv::new(360.0, 1.0, 1.0),
    ]);

    for i in 0..=100 {
        let pos = i as f32 / 100.0;
        let colour = cmap.sample(pos);
        println!("{} {}", colour, colour.to_hex());
    }

    println!("{}", cmap);
}
