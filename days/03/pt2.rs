mod util;

use util::parse_conditional_mult_instructions;

fn main() {
    let input = include_str!("../../assets/03-1");
    let res = parse_conditional_mult_instructions(input);
    println!("{res}");
}
