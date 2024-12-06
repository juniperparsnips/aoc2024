const NEEDLE: [char; 4] = ['X', 'M', 'A', 'S'];

pub fn count_xmas(input: &str) -> usize {
    let grid: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut total = 0;
    for i in 0..grid.len() {
        let row = &grid[i];
        for j in 0..row.len() {
            let current = row[j];
            if current != NEEDLE[0] {
                continue;
            }

            if scan_right(&row[j..]) {
                total += 1;
            }
            if scan_left(&row[..j + 1]) {
                total += 1;
            }
            if scan_down(&grid[i..], j) {
                total += 1;
            }
            if scan_up(&grid[..i + 1], j) {
                total += 1;
            }
            if scan_south_east(&grid[i..], j) {
                total += 1;
            }
            if scan_south_west(&grid[i..], j) {
                total += 1;
            }
            if scan_north_east(&grid[..i + 1], j) {
                total += 1;
            }

            if scan_north_west(&grid[..i + 1], j) {
                total += 1;
            }
        }
    }
    total
}

fn scan_right(row: &[char]) -> bool {
    if row.len() < NEEDLE.len() {
        return false;
    }

    for i in 0..NEEDLE.len() {
        if row[i] != NEEDLE[i] {
            return false;
        }
    }

    return true;
}

fn scan_left(row: &[char]) -> bool {
    if row.len() < NEEDLE.len() {
        return false;
    }

    for i in 0..NEEDLE.len() {
        if row[row.len() - (i + 1)] != NEEDLE[i] {
            return false;
        }
    }

    return true;
}

fn scan_down(grid: &[Vec<char>], col_idx: usize) -> bool {
    if grid.len() < NEEDLE.len() {
        return false;
    }

    for i in 0..NEEDLE.len() {
        if grid[i][col_idx] != NEEDLE[i] {
            return false;
        }
    }

    return true;
}

fn scan_up(grid: &[Vec<char>], col_idx: usize) -> bool {
    if grid.len() < NEEDLE.len() {
        return false;
    }

    for i in 0..NEEDLE.len() {
        if grid[grid.len() - (i + 1)][col_idx] != NEEDLE[i] {
            return false;
        }
    }

    return true;
}

fn scan_south_east(grid: &[Vec<char>], col_idx: usize) -> bool {
    if grid.len() < NEEDLE.len() || col_idx + NEEDLE.len() > grid[0].len() {
        return false;
    }

    for i in 0..NEEDLE.len() {
        if grid[i][col_idx + i] != NEEDLE[i] {
            return false;
        }
    }

    return true;
}

fn scan_south_west(grid: &[Vec<char>], col_idx: usize) -> bool {
    if grid.len() < NEEDLE.len() || col_idx < NEEDLE.len() - 1 {
        return false;
    }

    for i in 0..NEEDLE.len() {
        if grid[i][col_idx - i] != NEEDLE[i] {
            return false;
        }
    }

    return true;
}

fn scan_north_east(grid: &[Vec<char>], col_idx: usize) -> bool {
    if grid.len() < NEEDLE.len() || col_idx + NEEDLE.len() > grid[0].len() {
        return false;
    }

    for i in 0..NEEDLE.len() {
        if grid[grid.len() - (i + 1)][col_idx + i] != NEEDLE[i] {
            return false;
        }
    }

    return true;
}

fn scan_north_west(grid: &[Vec<char>], col_idx: usize) -> bool {
    if grid.len() < NEEDLE.len() || col_idx < NEEDLE.len() - 1 {
        return false;
    }

    for i in 0..NEEDLE.len() {
        if grid[grid.len() - (i + 1)][col_idx - i] != NEEDLE[i] {
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
}
