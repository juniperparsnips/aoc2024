use regex::Regex;

pub fn parse_mult_instructions(input: &str) -> usize {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [a, b])| {
            let x: usize = a.parse().unwrap();
            let y: usize = b.parse().unwrap();
            x * y
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
        assert_eq!(parse_mult_instructions(input), 161);
    }
}
