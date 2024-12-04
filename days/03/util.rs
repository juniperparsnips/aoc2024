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

pub fn parse_conditional_mult_instructions(input: &str) -> usize {
    let re =
        Regex::new(r"mul\((?<a>[0-9]{1,3}),(?<b>[0-9]{1,3})\)|(?<do>do\(\))|(?<dont>don\'t\(\))")
            .unwrap();

    let mut total = 0;
    let mut enabled = true;
    for c in re.captures_iter(input) {
        if c.name("do").is_some() {
            enabled = true;
        }
        if c.name("dont").is_some() {
            enabled = false;
        }
        if let Some((a, b)) = c.name("a").zip(c.name("b")) {
            if enabled {
                let x: usize = a.as_str().parse().unwrap();
                let y: usize = b.as_str().parse().unwrap();
                total += x * y;
            }
        }
    }

    return total;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
        assert_eq!(parse_mult_instructions(input), 161);
    }

    #[test]
    fn ex2() {
        let input = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
        assert_eq!(parse_conditional_mult_instructions(input), 48);
    }
}
