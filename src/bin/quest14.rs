use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r".#.##.
##..#.
..##.#
.#.##.
.###..
###.##";
    const EXAMPLE2_INPUT: &str = r"#......#
..#..#..
.##..##.
...##...
...##...
.##..##.
..#..#..
#......#";

    #[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT, 10);
        assert_eq!(200, r);
    }

    #[test]
    pub fn test2() {
        let r = solve3(EXAMPLE2_INPUT);
        assert_eq!(278388552, r);
    }
}

fn read_at(grid: &[Vec<char>], x: isize, y: isize) -> char {
    if x < 0 || y < 0 || x >= grid[0].len() as isize || y >= grid.len() as isize {
        '.'
    } else {
        grid[y as usize][x as usize]
    }
}

fn next(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut next_grid = Vec::new();
    for y in 0..grid.len() as isize {
        let mut row = Vec::new();
        for x in 0..grid[0].len() as isize {
            let odd = (read_at(grid, x - 1, y - 1) == '#')
                ^ (read_at(grid, x - 1, y + 1) == '#')
                ^ (read_at(grid, x + 1, y - 1) == '#')
                ^ (read_at(grid, x + 1, y + 1) == '#');
            if read_at(grid, x, y) == '#' {
                row.push(if odd { '#' } else { '.' });
            } else {
                row.push(if !odd { '#' } else { '.' });
            }
        }
        next_grid.push(row);
    }
    next_grid
}

fn solve(input_file: &str, round_count: usize) -> usize {
    let mut grid = input_file
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut count = 0;
    for _ in 0..round_count {
        grid = next(&grid);
        let n = grid
            .iter()
            .map(|x| x.iter().filter(|&y| *y == '#').count())
            .sum::<usize>();
        count += n;
    }

    count
}

fn has_center(grid: &[Vec<char>], center_grid: &[Vec<char>]) -> bool {
    let x_offset = grid[0].len() / 2 - center_grid[0].len() / 2;
    let y_offset = grid[0].len() / 2 - center_grid[0].len() / 2;
    for y in 0..center_grid.len() {
        for x in 0..center_grid[0].len() {
            if center_grid[y][x] != grid[y_offset + y][x_offset + x] {
                return false;
            }
        }
    }
    true
}

fn solve3(input_file: &str) -> usize {
    let round_count: usize = 1000000000;
    let center_grid = input_file
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut grid = Vec::new();
    for _i in 0..34 {
        let mut row = Vec::new();
        for _j in 0..34 {
            row.push('.');
        }
        grid.push(row);
    }

    let mut matches: Vec<(usize, Vec<Vec<char>>, usize)> = Vec::new();
    let mut count = 0;
    for i in 0..round_count {
        grid = next(&grid);
        if has_center(&grid, &center_grid) {
            let n = grid
                .iter()
                .map(|x| x.iter().filter(|&y| *y == '#').count())
                .sum::<usize>();
            if let Some(&(i0, ref first_match, _)) = matches.first()
                && *first_match == grid
            {
                let rounds_left = round_count - i;
                let cycle_length = i - i0;
                let cycles_left = rounds_left / cycle_length;
                let mut total_count = (1 + cycles_left) * count;
                let rounds_left = rounds_left % cycle_length;
                for (i, _, count) in matches.iter() {
                    let n = i - matches[0].0;
                    if rounds_left < n {
                        break;
                    }
                    total_count += count;
                }
                return total_count;
            }
            matches.push((i, grid.clone(), n));
            count += n;
        }
    }
    panic!("round count exceeded");
}

fn main() {
    let input_file = fs::read_to_string("everybody_codes_e2025_q14_p1.txt").unwrap();
    let r = solve(&input_file, 10);
    println!("Part 1: {}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q14_p2.txt").unwrap();
    let r = solve(&input_file, 2025);
    println!("Part 2: {}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q14_p3.txt").unwrap();
    let r = solve3(&input_file);
    println!("Part 3: {}", r);
}
