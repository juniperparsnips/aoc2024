use util::count_xed_mas;

mod util;

fn main() {
    let input = include_str!("../../assets/04-1");
    let res = count_xed_mas(input);
    println!("{res}");
}
