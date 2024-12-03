mod util;

use util::similarity_score;

fn main() {
    let input = include_str!("../../assets/01-1");
    let score = similarity_score(input);
    println!("{score}");
}
