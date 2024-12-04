#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
            let levels: Vec<_> = report
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

pub fn num_safe_dampened(input: &str) -> usize {
    input
        .lines()
        .map(|report| {
            let levels = report
                .split_ascii_whitespace()
                .map(|level| level.parse().unwrap())
                .collect();
            if is_safe_dampened(levels) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn test_level(a: usize, b: usize, dir: &mut Option<Direction>) -> bool {
    let this_dir = if a > b {
        Direction::Decreasing
    } else {
        Direction::Increasing
    };

    if let Some(prev_dir) = dir {
        if *prev_dir != this_dir {
            return false;
        }
    } else {
        *dir = Some(this_dir);
    }

    if a.abs_diff(b) > MAX_CHANGE || a.abs_diff(b) < MIN_CHANGE {
        return false;
    }

    return true;
}

fn is_safe(levels: Vec<usize>) -> bool {
    let mut direction = None;
    for window in levels.windows(2) {
        if !test_level(window[0], window[1], &mut direction) {
            return false;
        }
    }

    return true;
}

fn is_split_safe(a: &[usize], b: &[usize]) -> bool {
    let mut direction = None;

    for window in a.windows(2) {
        if !test_level(window[0], window[1], &mut direction) {
            return false;
        }
    }

    if a.len() > 0 && b.len() > 0 {
        if !test_level(a[a.len() - 1], b[0], &mut direction) {
            return false;
        }
    }

    for window in b.windows(2) {
        if !test_level(window[0], window[1], &mut direction) {
            return false;
        }
    }

    return true;
}

fn is_safe_dampened(levels: Vec<usize>) -> bool {
    let mut direction: Option<Direction> = None;

    for i in 0..levels.len() - 1 {
        let x = levels[i];
        let y = levels[i + 1];

        if test_level(x, y, &mut direction) {
            continue;
        }

        if i == levels.len() - 2 {
            return true;
        }

        let mut splits = vec![
            (&levels[0..i], &levels[i + 1..]),     // skip x
            (&levels[0..i + 1], &levels[i + 2..]), // skip y
        ];

        if i != 0 {
            splits.push((&levels[0..i - 1], &levels[i..])); // skip w
        }

        return splits.iter().any(|(a, b)| is_split_safe(a, b));
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

    #[test]
    fn ex2() {
        let input = r#"7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9"#;
        assert_eq!(num_safe_dampened(input), 4);
    }

    #[test]
    fn sub2() {
        assert!(is_safe_dampened(vec![7, 6, 4, 2, 1]));
        assert!(!is_safe_dampened(vec![1, 2, 7, 8, 9]));
        assert!(!is_safe_dampened(vec![9, 7, 6, 2, 1]));
        assert!(is_safe_dampened(vec![1, 3, 2, 4, 5])); // remove i2
        assert!(is_safe_dampened(vec![8, 6, 4, 4, 1])); // remove i2 or i3
        assert!(is_safe_dampened(vec![1, 3, 6, 7, 9]));
        assert!(!is_safe_dampened(vec![6, 4, 4, 4, 1]));
        assert!(is_safe_dampened(vec![1, 5, 6, 7, 9])); // remove i0
        assert!(is_safe_dampened(vec![1, 5, 2, 3, 4])); // remove i1
        assert!(is_safe_dampened(vec![5, 1, 6, 7, 9])); // remove i1
        assert!(is_safe_dampened(vec![1, 0, 4, 5, 6])); // remove i1
        assert!(!is_safe_dampened(vec![1, 0, 5, 6, 7]));
        assert!(is_safe_dampened(vec![1, 2, 3, 4, 10])); // remove i4
        assert!(!is_safe_dampened(vec![1, 2, 3, 9, 15])); //
        assert!(!is_safe_dampened(vec![53, 54, 56, 58, 59, 56, 54]));
        assert!(!is_safe_dampened(vec![81, 84, 86, 87, 90, 90, 91, 90]));

        assert!(is_safe_dampened(vec![48, 46, 47, 49, 51, 54, 56])); // remove i0
        assert!(is_safe_dampened(vec![1, 1, 2, 3, 4, 5])); // remove i0
        assert!(is_safe_dampened(vec![1, 2, 3, 4, 5, 5])); // remove i4
        assert!(is_safe_dampened(vec![5, 1, 2, 3, 4, 5])); // remove i0
        assert!(is_safe_dampened(vec![1, 4, 3, 2, 1])); // remove i0
        assert!(is_safe_dampened(vec![1, 6, 7, 8, 9])); // remove i0
        assert!(is_safe_dampened(vec![1, 2, 3, 4, 3])); // remove i4
        assert!(is_safe_dampened(vec![9, 8, 7, 6, 7])); // remove i4
        assert!(is_safe_dampened(vec![7, 10, 8, 10, 11])); // remove i1
        assert!(is_safe_dampened(vec![29, 28, 27, 25, 26, 25, 22, 20])); // remove i3 (first 25)
        assert!(is_safe_dampened(vec![6, 1, 4, 5]));
        assert!(is_safe_dampened(vec![3, 1, 4, 5]));
    }
}
