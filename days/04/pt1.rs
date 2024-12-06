mod util;

use util::count_xmas;

fn main() {
    let input = include_str!("../../assets/04-1");
    let res = count_xmas(input);
    println!("{res}");
}
