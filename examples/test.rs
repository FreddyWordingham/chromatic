use chromatic::Grey;
use core::str::FromStr;

fn main() {
    let g = Grey::<f32>::from_str("#XY");
    println!("{:?}", g);
}
