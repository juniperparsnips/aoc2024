pub fn count_xmas(input: &str) -> usize {
    let needle = ['X', 'M', 'A', 'S'];
    let grid: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut total = 0;
    for i in 0..grid.len() {
        let row = &grid[i];
        for j in 0..row.len() {
            let current = row[j];
            if current != needle[0] {
                continue;
            }

            if scan_right(&row[j..], &needle) {
                total += 1;
            }
            if scan_left(&row[..j + 1], &needle) {
                total += 1;
            }
            if scan_down(&grid[i..], j, &needle) {
                total += 1;
            }
            if scan_up(&grid[..i + 1], j, &needle) {
                total += 1;
            }
            if scan_south_east(&grid[i..], j, &needle) {
                total += 1;
            }
            if scan_south_west(&grid[i..], j, &needle) {
                total += 1;
            }
            if scan_north_east(&grid[..i + 1], j, &needle) {
                total += 1;
            }
            if scan_north_west(&grid[..i + 1], j, &needle) {
                total += 1;
            }
        }
    }
    total
}

pub fn count_xed_mas(input: &str) -> usize {
    let grid: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

    let needle = ['M', 'A', 'S'];

    let mut total = 0;
    for i in 1..grid.len() - 1 {
        let row = &grid[i];
        for j in 1..row.len() - 1 {
            let current = row[j];
            if current != 'A' {
                continue;
            }

            let mut here = 0;

            if scan_south_east(&grid[i - 1..], j - 1, &needle) {
                here += 1;
            }
            if scan_south_west(&grid[i - 1..], j + 1, &needle) {
                here += 1;
            }
            if scan_north_east(&grid[..i + 2], j - 1, &needle) {
                here += 1;
            }
            if scan_north_west(&grid[..i + 2], j + 1, &needle) {
                here += 1;
            }

            if here >= 2 {
                total += 1;
            }
        }
    }
    total
}

fn scan_right(row: &[char], needle: &[char]) -> bool {
    if row.len() < needle.len() {
        return false;
    }

    for i in 0..needle.len() {
        if row[i] != needle[i] {
            return false;
        }
    }

    return true;
}

fn scan_left(row: &[char], needle: &[char]) -> bool {
    if row.len() < needle.len() {
        return false;
    }

    for i in 0..needle.len() {
        if row[row.len() - (i + 1)] != needle[i] {
            return false;
        }
    }

    return true;
}

fn scan_down(grid: &[Vec<char>], col_idx: usize, needle: &[char]) -> bool {
    if grid.len() < needle.len() {
        return false;
    }

    for i in 0..needle.len() {
        if grid[i][col_idx] != needle[i] {
            return false;
        }
    }

    return true;
}

fn scan_up(grid: &[Vec<char>], col_idx: usize, needle: &[char]) -> bool {
    if grid.len() < needle.len() {
        return false;
    }

    for i in 0..needle.len() {
        if grid[grid.len() - (i + 1)][col_idx] != needle[i] {
            return false;
        }
    }

    return true;
}

fn scan_south_east(grid: &[Vec<char>], col_idx: usize, needle: &[char]) -> bool {
    if grid.len() < needle.len() || col_idx + needle.len() > grid[0].len() {
        return false;
    }

    for i in 0..needle.len() {
        if grid[i][col_idx + i] != needle[i] {
            return false;
        }
    }

    return true;
}

fn scan_south_west(grid: &[Vec<char>], col_idx: usize, needle: &[char]) -> bool {
    if grid.len() < needle.len() || col_idx < needle.len() - 1 {
        return false;
    }

    for i in 0..needle.len() {
        if grid[i][col_idx - i] != needle[i] {
            return false;
        }
    }

    return true;
}

fn scan_north_east(grid: &[Vec<char>], col_idx: usize, needle: &[char]) -> bool {
    if grid.len() < needle.len() || col_idx + needle.len() > grid[0].len() {
        return false;
    }

    for i in 0..needle.len() {
        if grid[grid.len() - (i + 1)][col_idx + i] != needle[i] {
            return false;
        }
    }

    return true;
}

fn scan_north_west(grid: &[Vec<char>], col_idx: usize, needle: &[char]) -> bool {
    if grid.len() < needle.len() || col_idx < needle.len() - 1 {
        return false;
    }

    for i in 0..needle.len() {
        if grid[grid.len() - (i + 1)][col_idx - i] != needle[i] {
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
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        assert_eq!(count_xmas(input), 18);
    }

    #[test]
    fn ex1_1() {
        let input = r#"....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX"#;
        assert_eq!(count_xmas(input), 18);
    }

    #[test]
    fn edge1() {
        let input = r#"X...
.M..
..A.
...S"#;
        assert_eq!(count_xmas(input), 1);
    }

    #[test]
    fn edge2() {
        let input = r#"SAMX"#;
        assert_eq!(count_xmas(input), 1);
    }

    #[test]
    fn edge3() {
        let input = r#"S
A
M
X"#;
        assert_eq!(count_xmas(input), 1);
    }

    #[test]
    fn edge4() {
        let input = r#"...S
..A.
.M..
X..."#;
        assert_eq!(count_xmas(input), 1);
    }

    #[test]
    fn edge5() {
        let input = r#"...X
..M.
.A..
S..."#;
        assert_eq!(count_xmas(input), 1);
    }

    #[test]
    fn edge6() {
        let input = r#"S...
.A..
..M.
...X"#;
        assert_eq!(count_xmas(input), 1);
    }

    #[test]
    fn ex2() {
        let input = r#".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
.........."#;
        assert_eq!(count_xed_mas(input), 9);
    }

    #[test]
    fn edge2_1() {
        let input = r#"M.M
.A.
S.S"#;
        assert_eq!(count_xed_mas(input), 1);
    }

    #[test]
    fn edge2_2() {
        let input = r#"S.S
.A.
M.M"#;
        assert_eq!(count_xed_mas(input), 1);
    }

    #[test]
    fn edge2_3() {
        let input = r#"A"#;
        assert_eq!(count_xed_mas(input), 0);
    }

    #[test]
    fn edge2_4() {
        let input = r#".A"#;
        assert_eq!(count_xed_mas(input), 0);
    }

    #[test]
    fn edge2_5() {
        let input = r#".
.A"#;
        assert_eq!(count_xed_mas(input), 0);
    }

    #[test]
    fn edge2_6() {
        let input = r#".
A"#;
        assert_eq!(count_xed_mas(input), 0);
    }
}
