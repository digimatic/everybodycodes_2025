use std::cmp::Reverse;
use std::collections::HashMap;
use std::collections::{BinaryHeap, HashSet};
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"L6,L3,L6,R3,L6,L3,L3,R6,L6,R6,L6,L6,R3,L3,L3,R3,R3,L6,L6,L3";

    #[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT);
        assert_eq!(16, r);
    }

    #[test]
    pub fn test2() {
        let r = solve3(EXAMPLE1_INPUT);
        assert_eq!(16, r);
    }
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
    grid.insert(*start, 'S');
    for (turn_op, distance) in instructions.iter() {
        direction = turn(*turn_op, &direction);
        for _ in 1..=*distance {
            p.0 += direction.0;
            p.1 += direction.1;
            grid.insert(p, '#');
        }
    }
    grid.insert(p, 'E');
    (p, grid)
}

#[allow(dead_code)]
fn print_grid(grid: &Grid) {
    if grid.is_empty() {
        println!("Grid is empty");
        return;
    }
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

    dijkstra(start_pos, end_pos, &grid)
}

type Lines = Vec<(Position, Position)>;

fn run_instructions2(instructions: &Instructions, start: &Position) -> (Position, Lines) {
    let turn = |turn_op: char, dir: &Direction| -> Direction {
        match turn_op {
            'L' => (dir.1, -dir.0),
            'R' => (-dir.1, dir.0),
            _ => panic!("Invalid turn op"),
        }
    };
    let mut direction = (0, -1);
    let mut p = *start;
    let mut lines = Lines::new();
    for (turn_op, distance) in instructions.iter() {
        direction = turn(*turn_op, &direction);
        let p2 = (
            p.0 + direction.0 * *distance as i64,
            p.1 + direction.1 * *distance as i64,
        );
        lines.push((p, p2));
        p = p2;
    }
    (p, lines)
}

fn compress_1d(mut xs: Vec<i64>) -> (HashMap<i64, i64>, HashMap<i64, i64>) {
    xs.sort();
    xs.dedup();
    let fwdmap = xs
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x, (3 * i + 1) as i64))
        .collect::<HashMap<_, _>>();
    let revmap = fwdmap
        .iter()
        .flat_map(|(&old, &new)| vec![(new, old), (new - 1, old - 1), (new + 1, old + 1)])
        .collect::<HashMap<i64, i64>>();
    (fwdmap, revmap)
}

#[allow(clippy::type_complexity)]
fn compress_lines(
    lines: &Lines,
) -> (
    Lines,
    HashMap<i64, i64>,
    HashMap<i64, i64>,
    HashMap<i64, i64>,
    HashMap<i64, i64>,
) {
    let nodes = lines
        .iter()
        .flat_map(|&(a, b)| [a, b])
        .collect::<HashSet<Position>>();
    let x_coords: Vec<i64> = nodes.iter().map(|&(x, _)| x).collect();
    let y_coords: Vec<i64> = nodes.iter().map(|&(_, y)| y).collect();
    let (xmap, xrevmap) = compress_1d(x_coords);
    let (ymap, yrevmap) = compress_1d(y_coords);
    let lines = lines
        .iter()
        .map(|&((x1, y1), (x2, y2))| ((xmap[&x1], ymap[&y1]), (xmap[&x2], ymap[&y2])))
        .collect();
    (lines, xmap, ymap, xrevmap, yrevmap)
}

fn draw_line(grid: &mut Grid, p1: &Position, p2: &Position, symbol: char) {
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    if x1 == x2 {
        // Vertical line
        let start_y = (*y1).min(*y2);
        let end_y = (*y1).max(*y2);
        for y in start_y..=end_y {
            grid.insert((*x1, y), symbol);
        }
    } else if y1 == y2 {
        // Horizontal line
        let start_x = (*x1).min(*x2);
        let end_x = (*x1).max(*x2);
        for x in start_x..=end_x {
            grid.insert((x, *y1), symbol);
        }
    }
}

fn manhattan_distance(a: Position, b: Position) -> usize {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as usize
}

fn dijkstra2(
    start_pos: Position,
    end_pos: Position,
    grid: &Grid,
    xrevmap: &HashMap<i64, i64>,
    yrevmap: &HashMap<i64, i64>,
) -> usize {
    let mut heap = BinaryHeap::new();
    let mut visited = HashMap::new();
    visited.insert(start_pos, 0);
    heap.push(Reverse((0, start_pos)));

    while let Some(Reverse((cost, pos))) = heap.pop() {
        if pos == end_pos {
            return cost;
        }

        // Check all 4 directions
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_pos = (pos.0 + dx, pos.1 + dy);

            if *grid.get(&new_pos).unwrap_or(&' ') != '#' {
                let real_pos = (xrevmap[&pos.0], yrevmap[&pos.1]);
                let real_newpos = (xrevmap[&new_pos.0], yrevmap[&new_pos.1]);
                let new_cost = cost + manhattan_distance(real_pos, real_newpos);

                let old_cost = *visited.get(&new_pos).unwrap_or(&usize::MAX);
                if new_cost < old_cost {
                    visited.insert(new_pos, new_cost);
                    heap.push(Reverse((new_cost, new_pos)));
                }
            }
        }
    }

    usize::MAX // No path found
}

fn solve3(input_file: &str) -> usize {
    let instruction = parse_path(input_file);
    let start_pos = (0, 0);
    let (end_pos, lines) = run_instructions2(&instruction, &start_pos);
    let (compressed_lines, xmap, ymap, xrevmap, yrevmap) = compress_lines(&lines);
    let mut grid = Grid::new();
    for (p1, p2) in compressed_lines.iter() {
        draw_line(&mut grid, p1, p2, '#');
    }

    let start_pos = (xmap[&0], ymap[&0]);
    let end_pos = (xmap[&end_pos.0], ymap[&end_pos.1]);
    draw_line(&mut grid, &start_pos, &start_pos, 'S');
    draw_line(&mut grid, &end_pos, &end_pos, 'E');

    // print_grid(&grid);

    dijkstra2(start_pos, end_pos, &grid, &xrevmap, &yrevmap)
}

fn main() {
    let input_file = fs::read_to_string("everybody_codes_e2025_q15_p1.txt").unwrap();
    let r = solve(&input_file);
    println!("Part 1: {}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q15_p2.txt").unwrap();
    let r = solve(&input_file);
    println!("Part 1: {}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q15_p3.txt").unwrap();
    let r = solve3(&input_file);
    println!("Part 3: {}", r);
}
