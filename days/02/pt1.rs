mod util;

use util::num_safe;

fn main() {
    let input = include_str!("../../assets/02-1");
    let safe = num_safe(input);
    println!("{safe}");
}
