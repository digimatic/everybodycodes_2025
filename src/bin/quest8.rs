use everybodycodes_2025::parse_utils;
use std::{fs, mem::swap};

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"1,5,2,6,8,4,1,7,3";
    #[allow(dead_code)]
    const EXAMPLE2_INPUT: &str = r"1,5,2,6,8,4,1,7,3,5,7,8,2";
    #[allow(dead_code)]
    const EXAMPLE3_INPUT: &str = r"1,5,2,6,8,4,1,7,3,6";

    #[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT, 8);
        assert_eq!(4, r);
    }

    #[test]
    pub fn test2() {
        let r = solve2(EXAMPLE2_INPUT, 8);
        assert_eq!(21, r);
    }

    #[test]
    pub fn test3() {
        let r = solve3(EXAMPLE3_INPUT, 8);
        assert_eq!(7, r);
    }
}

fn solve(input_file: &str, nail_count: usize) -> usize {
    let xss = parse_utils::parse_numbers(input_file);

    let mut count = 0;
    for i in 1..xss.len() {
        let n1 = xss[i - 1] as i32;
        let n2 = xss[i] as i32;
        if (n2 - n1).abs() == (nail_count / 2) as i32 {
            count += 1;
        }
    }
    count
}

fn solve2(input_file: &str, _nail_count: usize) -> usize {
    let xss = parse_utils::parse_numbers(input_file);

    let mut count = 0;
    let mut strings = Vec::new();
    for i in 1..xss.len() {
        let mut n1 = xss[i - 1] as i32;
        let mut n2 = xss[i] as i32;
        if n2 < n1 {
            swap(&mut n1, &mut n2);
        }

        for &(s1, s2) in strings.iter() {
            if n1 < s1 && n2 > s1 && n2 < s2 || n1 > s1 && n1 < s2 && n2 > s2 {
                count += 1;
            }
        }
        strings.push((n1, n2))
    }
    count
}
fn solve3(input_file: &str, nail_count: usize) -> usize {
    let xss = parse_utils::parse_numbers(input_file);

    let mut strings = Vec::new();
    for i in 1..xss.len() {
        let mut n1 = xss[i - 1] as i32 - 1;
        let mut n2 = xss[i] as i32 - 1;
        if n2 < n1 {
            swap(&mut n1, &mut n2);
        }
        strings.push((n1, n2))
    }

    let mut best_count = 0;
    for n1 in 0..nail_count as i32 {
        for n2 in n1 + 1..nail_count as i32 {
            let mut count = 0;
            for &(s1, s2) in strings.iter() {
                if n1 < s1 && n2 > s1 && n2 < s2
                    || n1 > s1 && n1 < s2 && n2 > s2
                    || n1 == s1 && n2 == s2
                {
                    count += 1;
                }
            }
            if count > best_count {
                best_count = count;
            }
        }
    }
    best_count
}

fn main() {
    let input_file = fs::read_to_string("everybody_codes_e2025_q08_p1.txt").unwrap();
    let r = solve(&input_file, 32);
    println!("{}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q08_p2.txt").unwrap();
    let r = solve2(&input_file, 256);
    println!("{}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q08_p3.txt").unwrap();
    let r = solve3(&input_file, 256);
    println!("{}", r);
}
