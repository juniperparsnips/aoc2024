mod util;

use util::total_distance;

fn main() {
    let input = include_str!("../../assets/01-1");
    let distance = total_distance(input);
    println!("{distance}");
}
