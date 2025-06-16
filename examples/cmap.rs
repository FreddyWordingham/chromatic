use chromatic::{ChromaticError, Colour, HsvAlphaMap};

fn main() -> Result<(), ChromaticError> {
    let colours = [
        "#ff0000ff", // Red
        "#00ff00ff", // Green
        "#0000ffff", // Blue
    ];
    let cmap = HsvAlphaMap::from_hex(&colours)?;

    for i in 0..=100 {
        let pos = i as f32 / 100.0;
        let colour = cmap.sample(pos)?;
        println!("{} {}", colour, colour.to_hex()?);
    }

    println!("{}", cmap);

    Ok(())
}
