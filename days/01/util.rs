use std::collections::HashMap;

pub fn total_distance(input: &str) -> usize {
    let (mut left, mut right) = split_columns(input);
    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum()
}

pub fn similarity_score(input: &str) -> usize {
    let (mut left, mut right) = split_columns(input);

    let mut right_counts: HashMap<usize, usize> = HashMap::new();
    for i in right {
        let current = right_counts.get(&i).map_or(0, |c| *c);
        right_counts.insert(i, current + 1);
    }

    left.iter()
        .map(|i| i * right_counts.get(&i).map_or(0, |c| *c))
        .sum()
}

fn split_columns(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .lines()
        .map(|l| {
            let mut row = l.split_ascii_whitespace();

            (
                row.next().unwrap().parse().unwrap(),
                row.next().unwrap().parse().unwrap(),
            )
        })
        .unzip::<usize, usize, Vec<_>, Vec<_>>()
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

    #[test]
    fn ex2() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        assert_eq!(similarity_score(input), 31);
    }
}
