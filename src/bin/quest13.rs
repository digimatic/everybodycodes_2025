use everybodycodes_2025::parse_utils;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"72
58
47
61
67";
    const EXAMPLE2_INPUT: &str = r"10-15
12-13
20-21
19-23
30-37";

    #[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT);
        assert_eq!(67, r);
    }

    #[test]
    pub fn test_pos_to_num() {
        let xs = parse(EXAMPLE1_INPUT);
        assert_eq!(pos_to_num(&xs, xs.len() as u64, 0), 1);
        assert_eq!(pos_to_num(&xs, xs.len() as u64, 1), 72);
        assert_eq!(pos_to_num(&xs, xs.len() as u64, 2), 47);
        assert_eq!(pos_to_num(&xs, xs.len() as u64, 3), 67);
        assert_eq!(pos_to_num(&xs, xs.len() as u64, 4), 61);
        assert_eq!(pos_to_num(&xs, xs.len() as u64, 5), 58);
        assert_eq!(pos_to_num(&xs, xs.len() as u64, 6), 1);
        assert_eq!(pos_to_num(&xs, xs.len() as u64, 12), 1);
    }

    #[test]
    pub fn test2() {
        let r = solve2(EXAMPLE2_INPUT);
        assert_eq!(30, r);
    }
}

fn parse(input_file: &str) -> Vec<u64> {
    input_file
        .lines()
        .flat_map(parse_utils::parse_numbers)
        .collect::<Vec<_>>()
}

fn pos_to_num(xs: &[u64], n: u64, pos: u64) -> u64 {
    if pos.is_multiple_of(n + 1) {
        return 1;
    }
    let pos = pos % (n + 1);
    let i = pos.saturating_sub(1);
    let index = if i <= n / 2 {
        i * 2
    } else {
        1 + 2 * (n - 1 - i)
    };
    xs[index as usize]
}

fn solve(input_file: &str) -> u64 {
    let xs = parse(input_file);
    pos_to_num(&xs, xs.len() as u64, 2025u64)
}

fn parse2(input_file: &str) -> Vec<(u64, u64)> {
    input_file
        .lines()
        .map(|x| {
            let ys = parse_utils::parse_numbers(x);
            (ys[0], ys[1])
        })
        .collect::<Vec<_>>()
}

fn pos_to_num2(xs: &[(u64, u64)], pos: u64) -> u64 {
    let n = xs.iter().map(|&(a, b)| 1 + b - a).sum::<u64>();
    let mut pos = pos % (n + 1);
    if pos == 0 {
        return 1;
    }
    pos -= 1;

    let mut clockwise = true;
    let mut i = 0;
    loop {
        if clockwise {
            if pos < (1 + xs[i].1 - xs[i].0) {
                return xs[i].0 + pos;
            } else {
                pos -= 1 + xs[i].1 - xs[i].0;
            }
            i += 2;
        } else {
            if pos < (1 + xs[i].1 - xs[i].0) {
                return xs[i].1 - pos;
            } else {
                pos -= 1 + xs[i].1 - xs[i].0;
            }
            i -= 2;
        }
        if i >= xs.len() {
            clockwise = !clockwise;
            if xs.len().is_multiple_of(2) {
                i = xs.len() - 1;
            } else {
                i = xs.len() - 2;
            }
        }
    }
}

fn solve2(input_file: &str) -> u64 {
    let xs = parse2(input_file);
    pos_to_num2(&xs, 20252025)
}

fn solve3(input_file: &str) -> u64 {
    let xs = parse2(input_file);
    pos_to_num2(&xs, 202520252025)
}

fn main() {
    let input_file = fs::read_to_string("everybody_codes_e2025_q13_p1.txt").unwrap();
    let r = solve(&input_file);
    println!("Part 1: {}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q13_p2.txt").unwrap();
    let r = solve2(&input_file);
    println!("Part 2: {}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q13_p3.txt").unwrap();
    let r = solve3(&input_file);
    println!("Part 3: {}", r);
}
