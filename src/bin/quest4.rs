use rational::Rational;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"128
64
32
16
8";

    const EXAMPLE2_INPUT: &str = r"102
75
50
35
13";

    const EXAMPLE3_INPUT: &str = r"5
5|10
10|20
5";

    const EXAMPLE4_INPUT: &str = r"5
7|21
18|36
27|27
10|50
10|50
11";

    #[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT);
        assert_eq!(32400, r);
    }

    #[test]
    pub fn test2() {
        let r = solve(EXAMPLE2_INPUT);
        assert_eq!(15888, r);
    }

    #[test]
    pub fn test3() {
        let r = solve2(EXAMPLE1_INPUT);
        assert_eq!(625000000000, r);
    }

    #[test]
    pub fn test4() {
        let r = solve2(EXAMPLE2_INPUT);
        assert_eq!(1274509803922, r);
    }

    #[test]
    pub fn test5() {
        let r = solve3(EXAMPLE3_INPUT);
        assert_eq!(400, r);
    }

    #[test]
    pub fn test6() {
        let r = solve3(EXAMPLE4_INPUT);
        assert_eq!(6818, r);
    }
}

fn solve(input_file: &str) -> isize {
    let gears = input_file
        .lines()
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    gears[0] * 2025 / gears[gears.len() - 1]
}

fn solve2(input_file: &str) -> isize {
    let gears = input_file
        .lines()
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    (10000000000000 * gears[gears.len() - 1] * 2 + gears[0]) / (2 * gears[0])
}

fn solve3(input_file: &str) -> i128 {
    let mut gears = input_file
        .lines()
        .map(|x| {
            x.split("|")
                .map(|x| x.parse::<i128>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let first_gear = vec![1i128, gears[0][0]];
    let last_gear = vec![gears.last().unwrap()[0], 1i128];
    *gears.first_mut().unwrap() = first_gear;
    *gears.last_mut().unwrap() = last_gear;

    let r = gears.into_iter().fold(Rational::new(1, 1), |acc, x| {
        let xr = Rational::new(x[1], x[0]);
        acc * xr
    });

    (r * Rational::new(100, 1)).floor().numerator()
}

fn main() {
    let input_file = fs::read_to_string("everybody_codes_e2025_q04_p1.txt").unwrap();
    let r = solve(&input_file);
    println!("{}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q04_p2.txt").unwrap();
    let r = solve2(&input_file);
    println!("{}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q04_p3.txt").unwrap();
    let r = solve3(&input_file);
    println!("{}", r);
}
