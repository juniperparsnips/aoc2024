mod util;

use util::num_safe_dampened;

fn main() {
    let input = include_str!("../../assets/02-1");
    let safe = num_safe_dampened(input);
    println!("{safe}");
}
