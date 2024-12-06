mod util;

use util::run_part_1;

fn main() {
    let input = include_str!("../../assets/05-1");
    let res = run_part_1(input);
    println!("{res}");
}
