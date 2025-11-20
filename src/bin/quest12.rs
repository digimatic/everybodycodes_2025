use everybodycodes_2025::parse_utils;
use std::{collections::HashSet, fs};

//#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"989601
857782
746543
766789";
    #[allow(dead_code)]
    const EXAMPLE2_INPUT: &str = r"9589233445
9679121695
8469121876
8352919876
7342914327
7234193437
6789193538
6781219648
5691219769
5443329859";
    #[allow(dead_code)]
    const EXAMPLE3_INPUT: &str = r"5411
3362
5235
3112";
    #[allow(dead_code)]
    const EXAMPLE4_INPUT: &str = r"41951111131882511179
32112222211518122215
31223333322115122219
31234444432147511128
91223333322176121892
61112222211166431583
14661111166111111746
11111119142122222177
41222118881233333219
71222127839122222196
56111126279711111517";

    //#[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT);
        assert_eq!(16, r);
    }

    //#[test]
    pub fn test2() {
        let r = solve2(EXAMPLE2_INPUT);
        assert_eq!(58, r);
    }

    //#[test]
    pub fn test3() {
        let r = solve3(EXAMPLE3_INPUT);
        assert_eq!(14, r);
    }

    //#[test]
    pub fn test4() {
        let r = solve3(EXAMPLE4_INPUT);
        assert_eq!(136, r);
    }
}

fn solve(input_file: &str) -> usize {
    let xss = parse_map(input_file);
    walk_barrels(&xss)
}

fn parse_map(input_file: &str) -> Vec<Vec<u8>> {
    let xss = input_file
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>();
    xss
}
type vec2i = (i32, i32);

fn walk_barrels(grid: &[Vec<u8>]) -> usize {
    let p: vec2i = (0, 0);
    let mut visited = HashSet::<vec2i>::new();
    let mut s = Vec::new();
    let w = grid[0].len() as i32;
    let h = grid.len() as i32;
    s.push(p);
    while let Some(p) = s.pop() {
        if visited.contains(&p) {
            continue;
        }
        visited.insert(p);
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let p2 = (p.0 + dx, p.1 + dy);
            if p2.0 >= 0
                && p2.0 < w
                && p2.1 >= 0
                && p2.1 < h
                && grid[p2.1 as usize][p2.0 as usize] <= grid[p.1 as usize][p.0 as usize]
            {
                s.push(p2)
            }
        }
    }
    visited.len()
}

fn walk_barrels2(grid: &[Vec<u8>]) -> usize {
    let p: vec2i = (0, 0);
    let mut visited = HashSet::<vec2i>::new();
    let mut s = Vec::new();
    let w = grid[0].len() as i32;
    let h = grid.len() as i32;
    s.push(p);
    s.push((w-1, h-1));
    while let Some(p) = s.pop() {
        if visited.contains(&p) {
            continue;
        }
        visited.insert(p);
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let p2 = (p.0 + dx, p.1 + dy);
            if p2.0 >= 0
                && p2.0 < w
                && p2.1 >= 0
                && p2.1 < h
                && grid[p2.1 as usize][p2.0 as usize] <= grid[p.1 as usize][p.0 as usize]
            {
                s.push(p2)
            }
        }
    }
    visited.len()
}

fn solve2(input_file: &str) -> usize {
    let xss = parse_map(input_file);
    walk_barrels2(&xss)
}

fn solve3(input_file: &str) -> usize {
    let xss = parse_map(input_file);
    walk_barrels3(&xss)
}
fn main() {
    tests::test1();
    tests::test2();
    tests::test3();
    tests::test4();

    let input_file = fs::read_to_string("everybody_codes_e2025_q12_p1.txt").unwrap();
    let r = solve(&input_file);
    println!("Part 1: {}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q12_p2.txt").unwrap();
    let r = solve2(&input_file);
    println!("Part 2: {}", r);

    // let input_file = fs::read_to_string("everybody_codes_e2025_q12_p3.txt").unwrap();
    // let r = solve3(&input_file);
    // println!("Part 3: {}", r);
}
