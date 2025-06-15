use chromatic::{ChromaticError, Colour, ColourMap, Hsv};

fn main() -> Result<(), ChromaticError> {
    let a = [
        Hsv::new(0.0, 1.0, 1.0),
        Hsv::new(90.0, 1.0, 1.0),
        Hsv::new(180.0, 1.0, 1.0),
        Hsv::new(270.0, 1.0, 1.0),
        Hsv::new(360.0, 1.0, 1.0),
    ];
    let cmap = ColourMap::new(&a)?;

    for i in 0..=100 {
        let pos = i as f32 / 100.0;
        let colour = cmap.sample(pos)?;
        println!("{} {}", colour, colour.to_hex());
    }

    println!("{}", cmap);

    // let a = Hsv::new(0.0, 1.0, 1.0);
    // println!("{} {}", a, a.to_hex());

    Ok(())
}
