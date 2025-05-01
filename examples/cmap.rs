use chromatic::{ColourMap, GreyAlpha};

fn main() {
    let grey1 = GreyAlpha::<f32>::new(0.0, 1.0);
    let grey2 = GreyAlpha::<f32>::new(1.0, 1.0);
    let positions = [0.0f32, 1.0f32];

    let cmap = ColourMap::new(&[grey1, grey2], &positions);

    for i in 0..=10 {
        let pos = i as f32 / 10.0;
        let colour = cmap.sample(pos);
        println!("Position: {}, Colour: {:?}", pos, colour);
    }
}
