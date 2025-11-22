use everybodycodes_2025::parse_utils;
use std::{collections::HashMap, fs};

//#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"L6,L3,L6,R3,L6,L3,L3,R6,L6,R6,L6,L6,R3,L3,L3,R3,R3,L6,L6,L3";
    #[allow(dead_code)]
    const EXAMPLE2_INPUT: &str = r"";
    #[allow(dead_code)]
    const EXAMPLE3_INPUT: &str = r"";
    #[allow(dead_code)]
    const EXAMPLE4_INPUT: &str = r"";

    //#[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT);
        assert_eq!(16, r);
    }

    //#[test]
    // pub fn test2() {
    //     let r = solve2(EXAMPLE2_INPUT);
    //     assert_eq!("", r);
    // }

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

type Instructions = Vec<(char, usize)>;
type Position = (i64, i64);
type Direction = (i64, i64);
type Grid = HashMap<Position, char>;
// y is down

fn parse_path(input_file: &str) -> Instructions {
    input_file
        .split(',')
        .map(|s| (s.chars().next().unwrap(), s[1..].parse::<usize>().unwrap()))
        .collect::<Vec<_>>()
}

fn run_instructions(instructions: &Instructions, start: &Position) -> (Position, Grid) {
    let turn = |turn_op: char, dir: &Direction| -> Direction {
        match turn_op {
            'L' => (dir.1, -dir.0),
            'R' => (-dir.1, dir.0),
            _ => panic!("Invalid turn op"),
        }
    };
    let mut direction = (0, -1);
    let mut p = *start;
    let mut grid = Grid::new();
    grid.insert(start.clone(), 'S');
    for (turn_op, distance) in instructions.iter() {
        direction = turn(*turn_op, &direction);
        for i in 1..=*distance {
            p.0 += direction.0;
            p.1 += direction.1;
            grid.insert(p.clone(), '#');
        }
    }
    grid.insert(p.clone(), 'E');
    (p, grid)
}

fn print_grid(grid: &Grid) {
    let min_x = grid.keys().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let min_y = grid.keys().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let max_x = grid.keys().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let max_y = grid.keys().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!("{}", grid.get(&(x, y)).unwrap_or(&' '));
        }
        println!();
    }
}

fn dijkstra(start_pos: Position, end_pos: Position, grid: &Grid) -> usize {
    use std::cmp::Reverse;
    use std::collections::{BinaryHeap, HashSet};

    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();

    heap.push(Reverse((0, start_pos)));

    while let Some(Reverse((cost, pos))) = heap.pop() {
        if pos == end_pos {
            return cost;
        }

        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);

        // Check all 4 directions
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_pos = (pos.0 + dx, pos.1 + dy);

            if !visited.contains(&new_pos) {
                if let Some(cell) = grid.get(&new_pos) {
                    // Can only move to S or E positions
                    if *cell == 'S' || *cell == 'E' {
                        heap.push(Reverse((cost + 1, new_pos)));
                    }
                } else {
                    // Empty space, can move there
                    heap.push(Reverse((cost + 1, new_pos)));
                }
            }
        }
    }

    usize::MAX // No path found
}

fn solve(input_file: &str) -> usize {
    let instruction = parse_path(input_file);
    let start_pos = (0, 0);
    let (end_pos, grid) = run_instructions(&instruction, &start_pos);
    println!("Grid size: {}", grid.len());
    // print_grid(&grid);

    dijkstra(start_pos, end_pos, &grid)
}

fn main() {
    tests::test1();
    // tests::test2();
    // tests::test3();
    // tests::test4();

    let input_file = fs::read_to_string("everybody_codes_e2025_q15_p1.txt").unwrap();
    let r = solve(&input_file);
    println!("Part 1: {}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q15_p2.txt").unwrap();
    let r = solve(&input_file);
    println!("Part 2: {}", r);

    // let input_file = fs::read_to_string("everybody_codes_e2025_q15_p3.txt").unwrap();
    // let r = solve(&input_file);
    // println!("Part 3: {}", r);
}
