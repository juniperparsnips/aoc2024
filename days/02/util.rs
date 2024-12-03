#[derive(PartialEq, Eq)]
enum Direction {
    Increasing,
    Decreasing,
}

const MIN_CHANGE: usize = 1;
const MAX_CHANGE: usize = 3;

pub fn num_safe(input: &str) -> usize {
    input
        .lines()
        .map(|report| {
            let levels = report
                .split_ascii_whitespace()
                .map(|level| level.parse().unwrap())
                .collect();
            if is_safe(levels) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn is_safe(levels: Vec<usize>) -> bool {
    let mut direction = None;
    for i in 0..levels.len() - 1 {
        let x = levels[i];
        let y = levels[i + 1];

        let this_dir = if x > y {
            Direction::Decreasing
        } else {
            Direction::Increasing
        };

        if let Some(prev_dir) = &direction {
            if *prev_dir != this_dir {
                return false;
            }
        } else {
            direction = Some(this_dir);
        }

        if x.abs_diff(y) > MAX_CHANGE || x.abs_diff(y) < MIN_CHANGE {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        assert_eq!(num_safe(input), 2);
    }

    #[test]
    fn sub1() {
        assert!(is_safe(vec![7, 6, 4, 2, 1]));
        assert!(!is_safe(vec![1, 2, 7, 8, 9]));
        assert!(!is_safe(vec![9, 7, 6, 2, 1]));
        assert!(!is_safe(vec![1, 3, 2, 4, 5]));
        assert!(!is_safe(vec![8, 6, 4, 4, 1]));
        assert!(is_safe(vec![1, 3, 6, 7, 9]));
    }
}
