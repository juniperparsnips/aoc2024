use std::collections::{HashMap, HashSet};

type Rules = HashMap<usize, HashSet<usize>>;

pub fn run_part_1(input: &str) -> usize {
    let (rules, updates) = parse_input(input);

    updates
        .iter()
        .filter(|u| check_order(&rules, u).is_none())
        .map(|u| {
            let midpoint = u.len() / 2;
            u[midpoint]
        })
        .sum()
}

pub fn run_part_2(input: &str) -> usize {
    let (rules, mut updates) = parse_input(input);

    updates
        .iter_mut()
        .filter(|u| check_order(&rules, u).is_some())
        .map(|u| {
            while let Some((i, j)) = check_order(&rules, u) {
                u.swap(i, j);
            }

            let midpoint = u.len() / 2;
            u[midpoint]
        })
        .sum()
}

fn check_order(rules: &Rules, update: &[usize]) -> Option<(usize, usize)> {
    for i in 0..update.len() - 1 {
        let before = update[i];
        for j in i + 1..update.len() {
            let after = update[j];
            if let Some(char_rules) = rules.get(&after) {
                if char_rules.contains(&before) {
                    return Some((i, j));
                }
            }
        }
    }

    return None;
}

fn parse_input(input: &str) -> (Rules, Vec<Vec<usize>>) {
    let mut parts_split = input.split("\n\n");
    let raw_rules: Vec<(usize, usize)> = parts_split
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut sides = l.split("|");
            let left = sides.next().unwrap().parse().unwrap();
            let right = sides.next().unwrap().parse().unwrap();
            (left, right)
        })
        .collect();
    let updates: Vec<Vec<usize>> = parts_split
        .next()
        .unwrap()
        .lines()
        .map(|l| l.split(",").map(|c| c.parse().unwrap()).collect())
        .collect();

    // Essentially functions as a di-graph. Must be non-cyclical
    let mut rules: Rules = HashMap::new();
    for (l, r) in raw_rules {
        if let Some(set) = rules.get_mut(&l) {
            set.insert(r);
        } else {
            let mut new_set = HashSet::new();
            new_set.insert(r);
            rules.insert(l, new_set);
        }
    }

    (rules, updates)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        assert_eq!(run_part_1(input), 143);
    }

    #[test]
    fn ex2() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        assert_eq!(run_part_2(input), 123);
    }
}
