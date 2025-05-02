use chromatic::{Colour, ColourMap, GreyAlpha};

fn main() {
    let col_a = GreyAlpha::<f32>::new(0.0, 1.0);
    let col_b = GreyAlpha::<f32>::new(1.0, 0.1);
    let positions = [0.0f32, 1.0f32];

    let cmap = ColourMap::new(&[col_a, col_b], &positions);

    for i in 0..=100 {
        let pos = i as f32 / 100.0;
        let colour = cmap.sample(pos);
        println!("{} {}", colour, colour.to_hex());
    }

    println!("{}", cmap);
}
