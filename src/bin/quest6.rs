use everybodycodes_2025::parse_utils;
use std::{collections::HashMap, fs};

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"ABabACacBCbca";
    #[allow(dead_code)]
    const EXAMPLE2_INPUT: &str = r"AABCBABCABCabcabcABCCBAACBCa";
    #[allow(dead_code)]
    const EXAMPLE3_INPUT: &str = r"";

    #[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT);
        assert_eq!(5, r);
    }

    #[test]
    pub fn test2() {
        let r = solve2(EXAMPLE1_INPUT);
        assert_eq!(11, r);
    }

    #[test]
    pub fn test3() {
        let r = solve3(EXAMPLE2_INPUT, 10, 1);
        assert_eq!(34, r);
        let r = solve3(EXAMPLE2_INPUT, 10, 2);
        assert_eq!(72, r);
        let r = solve3(EXAMPLE2_INPUT, 1000, 1000);
        assert_eq!(3442321, r);
    }
}

fn solve(input_file: &str) -> i64 {
    let mut total_mentors = 0;
    let mut a_mentors = 0;
    for c in input_file.chars() {
        if c == 'A' {
            a_mentors += 1;
        } else if c == 'a' {
            total_mentors += a_mentors;
        }
    }
    total_mentors
}

fn solve2(input_file: &str) -> i64 {
    let mut total_mentors = 0;
    let mut mentors = HashMap::new();
    for c in input_file.chars() {
        if c.is_ascii_uppercase() {
            mentors.entry(c).and_modify(|e| *e += 1).or_insert(1);
        } else if c.is_ascii_lowercase() {
            total_mentors += mentors.get(&c.to_ascii_uppercase()).unwrap_or(&0);
        }
    }
    total_mentors
}

fn solve3(input_file: &str, max_distance: usize, repeat_count: usize) -> usize {
    let input = input_file.chars().collect::<Vec<_>>();
    let len = input.len();
    let mut mentor_count = 0;
    for i in 0..(repeat_count * input_file.len()) {
        let mut count = 0;
        let c = input[i % len];
        if c.is_ascii_lowercase() {
            let mentor = c.to_ascii_uppercase();
            for j in (i.saturating_sub(max_distance))..=(i + max_distance).min(repeat_count * len - 1) {
                if input[j % len] == mentor {
                    count += 1;
                }
            }
            mentor_count += count;
        }
    }

    mentor_count
}

fn main() {
    let input_file = fs::read_to_string("everybody_codes_e2025_q06_p1.txt").unwrap();
    let r = solve(&input_file);
    println!("{}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q06_p2.txt").unwrap();
    let r = solve2(&input_file);
    println!("{}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q06_p3.txt").unwrap();
    let r = solve3(&input_file, 1000, 1000);
    println!("{}", r);
}
