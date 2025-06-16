use chromatic::{ChromaticError, Colour, ColourMap, Hsv};

fn main() -> Result<(), ChromaticError> {
    // let a = [
    //     Hsv::new(0.0, 1.0, 1.0)?,
    //     Hsv::new(90.0, 1.0, 1.0)?,
    //     Hsv::new(180.0, 1.0, 1.0)?,
    //     Hsv::new(270.0, 1.0, 1.0)?,
    //     Hsv::new(360.0, 1.0, 1.0)?,
    // ];
    // let cmap = ColourMap::new(&a)?;

    let colours = [
        "#ff0000", // Red
        "#00ff00", // Green
        "#0000ff", // Blue
    ];
    let cmap = ColourMap::<Hsv<f32>, f32, 3>::from_hex(&colours)?;

    for i in 0..=100 {
        let pos = i as f32 / 100.0;
        let colour = cmap.sample(pos)?;
        println!("{} {}", colour, colour.to_hex()?);
    }

    println!("{}", cmap);

    Ok(())
}
