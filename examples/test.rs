use chromatic::Rgb;
use core::str::FromStr;

fn main() {
    let result = Rgb::<f32>::from_str("  #FF0080  ");
    println!("{:?}", result);
}
