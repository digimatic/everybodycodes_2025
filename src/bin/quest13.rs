use everybodycodes_2025::parse_utils;
use std::{collections::VecDeque, fs};

//#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"72
58
47
61
67";
    #[allow(dead_code)]
    const EXAMPLE2_INPUT: &str = r"";
    #[allow(dead_code)]
    const EXAMPLE3_INPUT: &str = r"";
    #[allow(dead_code)]
    const EXAMPLE4_INPUT: &str = r"";

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

    //#[test]
    pub fn test2() {
        // let r = solve2(EXAMPLE2_INPUT);
        // assert_eq!(, r);
    }

    //#[test]
    // pub fn test3() {
    //     let r = solve3(EXAMPLE3_INPUT);
    //     assert_eq!("", r);
    // }

    //#[test]
    // pub fn test4() {
    //     let r = solve3(EXAMPLE4_INPUT);
    //     assert_eq!("", r);
    // }
}

fn parse(input_file: &str) -> Vec<u64> {
    let xs = input_file
        .lines()
        .flat_map(|x| parse_utils::parse_numbers(x))
        .collect::<Vec<_>>();
    xs
}

fn pos_to_num(xs: &[u64], n: u64, pos: u64) -> u64 {
    if (pos % (n + 1)) == 0 {
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

fn main() {
    // tests::test1();
    tests::test2();
    // tests::test3();
    // tests::test4();

    let input_file = fs::read_to_string("everybody_codes_e2025_q13_p1.txt").unwrap();
    let r = solve(&input_file);
    println!("Part 1: {}", r);

    // let input_file = fs::read_to_string("everybody_codes_e2025_q0?_p2.txt").unwrap();
    // let r = solve2(&input_file);
    // println!("Part 2: {}", r);

    // let input_file = fs::read_to_string("everybody_codes_e2025_q0?_p3.txt").unwrap();
    // let r = solve3(&input_file);
    // println!("Part 3: {}", r);
}
