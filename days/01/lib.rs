pub fn total_distance(input: &str) -> usize {
    let (mut left, mut right) = input
        .lines()
        .map(|l| {
            let mut row = l.split_ascii_whitespace();

            (
                row.next().unwrap().parse().unwrap(),
                row.next().unwrap().parse().unwrap(),
            )
        })
        .unzip::<usize, usize, Vec<_>, Vec<_>>();

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        assert_eq!(total_distance(input), 11);
    }
}
