use std::collections::HashSet;
use std::fs;

#[cfg(test)]
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

    #[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT);
        assert_eq!(16, r);
    }

    #[test]
    pub fn test2() {
        let r = solve2(EXAMPLE2_INPUT);
        assert_eq!(58, r);
    }

    #[test]
    pub fn test3() {
        let r = solve3(EXAMPLE3_INPUT);
        assert_eq!(14, r);
    }

    #[test]
    pub fn test4() {
        let r = solve3(EXAMPLE4_INPUT);
        assert_eq!(136, r);
    }
}

type Vec2i = (i32, i32);

fn solve(input_file: &str) -> usize {
    let xss = parse_map(input_file);
    walk_barrels(&xss)
}

fn parse_map(input_file: &str) -> Vec<Vec<u8>> {
    input_file
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>()
}

fn walk_barrels(grid: &[Vec<u8>]) -> usize {
    let p: Vec2i = (0, 0);
    let mut visited = HashSet::<Vec2i>::new();
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
    let mut s = Vec::new();
    let w = grid[0].len() as i32;
    let h = grid.len() as i32;
    s.push((0, 0));
    s.push((w - 1, h - 1));
    walk_concurrenty(grid, s)
}

fn walk_concurrenty(grid: &[Vec<u8>], mut s: Vec<(i32, i32)>) -> usize {
    let mut visited = HashSet::<Vec2i>::new();
    let w = grid[0].len() as i32;
    let h = grid.len() as i32;
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

fn walk_barrels3(grid: &[Vec<u8>], p: Vec2i, visited: &mut HashSet<Vec2i>) -> usize {
    visited.insert(p);
    let w = grid[0].len() as i32;
    let h = grid.len() as i32;

    let mut visited_count = 1;
    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let p2 = (p.0 + dx, p.1 + dy);
        if p2.0 >= 0
            && p2.0 < w
            && p2.1 >= 0
            && p2.1 < h
            && grid[p2.1 as usize][p2.0 as usize] <= grid[p.1 as usize][p.0 as usize]
            && !visited.contains(&p2)
        {
            visited_count += walk_barrels3(grid, p2, visited);
        }
    }
    visited_count
}

fn get_best_strike(
    grid: &[Vec<u8>],
    visited: HashSet<(i32, i32)>,
) -> (usize, (i32, i32), HashSet<(i32, i32)>) {
    let mut results = Vec::new();
    for y in 0..grid.len() as i32 {
        for x in 0..grid[0].len() as i32 {
            let pos = (x, y);
            let mut new_visited = visited.clone();
            let strike = walk_barrels3(grid, pos, &mut new_visited);
            results.push((strike, pos, new_visited));
        }
    }

    results
        .into_iter()
        .max_by_key(|(strike, _, _)| *strike)
        .unwrap()
}

fn solve3(input_file: &str) -> usize {
    let grid = parse_map(input_file);
    let best1 = get_best_strike(&grid, HashSet::new());
    let best2 = get_best_strike(&grid, best1.2);
    let best3 = get_best_strike(&grid, best2.2);

    walk_concurrenty(&grid, vec![best1.1, best2.1, best3.1])
}

fn main() {
    let input_file = fs::read_to_string("everybody_codes_e2025_q12_p1.txt").unwrap();
    let r = solve(&input_file);
    println!("Part 1: {}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q12_p2.txt").unwrap();
    let r = solve2(&input_file);
    println!("Part 2: {}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q12_p3.txt").unwrap();
    let r = solve3(&input_file);
    println!("Part 3: {}", r);
}
