mod util;

use util::num_safe;

fn main() {
    let input = include_str!("../../assets/02-1");
    let distance = num_safe(input);
    println!("{distance}");
}
