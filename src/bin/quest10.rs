use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

use everybodycodes_2025::sorted_set::SortedSet;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"...SSS.......
.S......S.SS.
..S....S...S.
..........SS.
..SSSS...S...
.....SS..S..S
SS....D.S....
S.S..S..S....
....S.......S
.SSS..SS.....
.........S...
.......S....S
SS.....S..S..";
    #[allow(dead_code)]
    const EXAMPLE2_INPUT: &str = r"...SSS##.....
.S#.##..S#SS.
..S.##.S#..S.
.#..#S##..SS.
..SSSS.#.S.#.
.##..SS.#S.#S
SS##.#D.S.#..
S.S..S..S###.
.##.S#.#....S
.SSS.#SS..##.
..#.##...S##.
.#...#.S#...S
SS...#.S.#S..";
    #[allow(dead_code)]
    const EXAMPLE3_INPUT: &str = r"SSS
..#
#.#
#D.";
    #[allow(dead_code)]
    const EXAMPLE4_INPUT: &str = r"SSS
..#
..#
.##
.D#";

    const EXAMPLE5_INPUT: &str = r"..S..
.....
..#..
.....
..D..";

    const EXAMPLE6_INPUT: &str = r".SS.S
#...#
...#.
##..#
.####
##D.#";

    const EXAMPLE7_INPUT: &str = r"SSS.S
.....
#.#.#
.#.#.
#.D.#";

    #[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT, 3);
        assert_eq!(27, r);
    }

    #[test]
    pub fn test2() {
        let r = solve2(EXAMPLE2_INPUT, 3);
        assert_eq!(27, r);
    }

    #[test]
    pub fn test3() {
        let r = solve3(EXAMPLE3_INPUT);
        assert_eq!(15, r);
    }

    #[test]
    pub fn test4() {
        let r = solve3(EXAMPLE4_INPUT);
        assert_eq!(8, r);
    }
    #[test]
    pub fn test5() {
        let r = solve3(EXAMPLE5_INPUT);
        assert_eq!(44, r);
    }
    #[test]
    pub fn test6() {
        let r = solve3(EXAMPLE6_INPUT);
        assert_eq!(4406, r);
    }
    #[test]
    pub fn test7() {
        let r = solve3(EXAMPLE7_INPUT);
        assert_eq!(13033988838, r);
    }
}

type Vec2i = (i32, i32);

fn next(position: Vec2i, w: i32, h: i32) -> Vec<Vec2i> {
    vec![
        (-2, 1),
        (-2, -1),
        (2, 1),
        (2, -1),
        (1, -2),
        (-1, -2),
        (1, 2),
        (-1, 2),
    ]
    .into_iter()
    .map(|(dx, dy)| (position.0 + dx, position.1 + dy))
    .filter(|(x, y)| *x >= 0 && *x < w && *y >= 0 && *y < h)
    .collect()
}

fn solve(input_file: &str, max_steps: usize) -> usize {
    let board = input_file
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let w = board[0].len() as i32;
    let h = board.len() as i32;
    let initial_position: Vec2i = ((w / 2), (h / 2));
    let mut visited: HashSet<Vec2i> = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((initial_position, 0));
    let read_board = |position: Vec2i| -> char {
        let (x, y) = position;
        board[y as usize][x as usize]
    };
    let mut sheep_count = 0;
    while !q.is_empty() {
        let (position, steps) = q.pop_front().unwrap();
        if steps > max_steps {
            continue;
        }
        if visited.contains(&position) {
            continue;
        }
        visited.insert(position);
        if read_board(position) == 'S' {
            sheep_count += 1;
        }
        next(position, w, h).into_iter().for_each(|p| {
            q.push_back((p, steps + 1));
        });
    }
    // println!("sheep_count: {}", sheep_count);
    sheep_count
}

fn all_next_dragon(positions: HashSet<Vec2i>, w: i32, h: i32) -> HashSet<Vec2i> {
    let mut new_positions: HashSet<Vec2i> = HashSet::new();
    for position in positions.into_iter() {
        next(position, w, h).into_iter().for_each(|p| {
            new_positions.insert(p);
        });
    }
    new_positions
}

fn solve2(input_file: &str, max_steps: usize) -> usize {
    let (board, w, h) = parse_board(input_file);
    let (mut sheep_positions, mut dragon_positions, hideout_positions) =
        extract_positions(&board, w, h);

    let elimminate_sheeps = |dragon_positions: &HashSet<Vec2i>,
                             sheep_positions: &mut HashSet<Vec2i>,
                             hideout_positions: &HashSet<Vec2i>|
     -> usize {
        let mut eliminated_sheeps = HashSet::new();
        for pos in sheep_positions.iter() {
            if dragon_positions.contains(pos) && !hideout_positions.contains(pos) {
                eliminated_sheeps.insert(*pos);
            }
        }
        for pos in &eliminated_sheeps {
            // println!("Eleminated sheep at ({},{})", pos.0, pos.1);
            sheep_positions.remove(pos);
        }
        eliminated_sheeps.len()
    };

    let mut eaten_sheep_count = 0;
    for _round in 0..max_steps {
        // println!("Round: {}", round+1);
        dragon_positions = all_next_dragon(dragon_positions, w, h);
        let n = elimminate_sheeps(&dragon_positions, &mut sheep_positions, &hideout_positions);
        // println!("Eleminated {} sheep", n);
        eaten_sheep_count += n;

        sheep_positions = sheep_positions
            .into_iter()
            .map(|(x, y)| (x, y + 1))
            .filter(|(_, y)| *y < h)
            .collect::<HashSet<_>>();

        let n = elimminate_sheeps(&dragon_positions, &mut sheep_positions, &hideout_positions);
        // println!("Eleminated {} sheep", n);
        eaten_sheep_count += n;
    }

    // println!("sheep_count: {}", eaten_sheep_count);
    eaten_sheep_count
}

fn parse_board(input_file: &str) -> (Vec<Vec<char>>, i32, i32) {
    let board = input_file
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let w = board[0].len() as i32;
    let h = board.len() as i32;
    (board, w, h)
}

fn extract_positions(
    board: &[Vec<char>],
    w: i32,
    h: i32,
) -> (HashSet<Vec2i>, HashSet<Vec2i>, HashSet<Vec2i>) {
    let mut sheep_positions = HashSet::new();
    let mut dragon_positions = HashSet::new();
    let mut hideout_positions = HashSet::new();
    for y in 0..h {
        for x in 0..w {
            let c = read_board((x, y), board);
            if c == 'S' {
                sheep_positions.insert((x, y));
            }
            if c == 'D' {
                dragon_positions.insert((x, y));
            }
            if c == '#' {
                hideout_positions.insert((x, y));
            }
        }
    }
    (sheep_positions, dragon_positions, hideout_positions)
}

fn read_board(position: Vec2i, board: &[Vec<char>]) -> char {
    let (x, y) = position;
    board[y as usize][x as usize]
}

fn solve3(input_file: &str) -> usize {
    let (board, w, h) = parse_board(input_file);
    let (sheep_positions, dragon_positions, hideout_positions) = extract_positions(&board, w, h);

    let mut sheep_cache = HashMap::new();
    let mut dragon_cache = HashMap::new();
    let dragon_position = *dragon_positions.iter().next().unwrap();

    do_sheep_pass(
        dragon_position,
        sheep_positions,
        &hideout_positions,
        w,
        h,
        &mut sheep_cache,
        &mut dragon_cache,
    )
}

fn do_sheep_pass(
    dragon_position: Vec2i,
    sheep_positions: HashSet<Vec2i>,
    hideout_positions: &HashSet<Vec2i>,
    w: i32,
    h: i32,
    sheep_cache: &mut HashMap<(Vec2i, SortedSet<Vec2i>), usize>,
    dragon_cache: &mut HashMap<(Vec2i, SortedSet<Vec2i>), usize>,
) -> usize {
    let key = (dragon_position, SortedSet::from(&sheep_positions));
    if let Some(&cached_value) = sheep_cache.get(&key) {
        return cached_value;
    }

    let mut total_count = 0;
    let mut any_sheep_moved = false;

    for sheep_position in sheep_positions.iter() {
        let new_sheep_position = (sheep_position.0, sheep_position.1 + 1);
        if new_sheep_position.1 == h {
            // sheep escaped
            any_sheep_moved = true;
        } else if new_sheep_position == dragon_position
            && !hideout_positions.contains(&new_sheep_position)
        {
            // don't move into dragon (unless it's the hideout)
        } else {
            let mut new_sheep_positions = sheep_positions.clone();
            new_sheep_positions.remove(sheep_position);
            new_sheep_positions.insert(new_sheep_position);
            any_sheep_moved = true;
            total_count += do_dragon_pass(
                dragon_position,
                new_sheep_positions,
                hideout_positions,
                w,
                h,
                sheep_cache,
                dragon_cache,
            );
        }
    }

    if !any_sheep_moved {
        total_count += do_dragon_pass(
            dragon_position,
            sheep_positions.clone(),
            hideout_positions,
            w,
            h,
            sheep_cache,
            dragon_cache,
        );
    }

    sheep_cache.insert(
        (dragon_position, SortedSet::from(&sheep_positions)),
        total_count,
    );
    total_count
}

fn do_dragon_pass(
    dragon_position: Vec2i,
    sheep_positions: HashSet<Vec2i>,
    hideout_positions: &HashSet<Vec2i>,
    w: i32,
    h: i32,
    sheep_cache: &mut HashMap<(Vec2i, SortedSet<Vec2i>), usize>,
    dragon_cache: &mut HashMap<(Vec2i, SortedSet<Vec2i>), usize>,
) -> usize {
    let key = (dragon_position, SortedSet::from(&sheep_positions));
    if let Some(&cached_value) = dragon_cache.get(&key) {
        return cached_value;
    }

    let mut total_count = 0;
    let new_dragon_positions = next(dragon_position, w, h);

    for new_dragon_position in new_dragon_positions {
        if sheep_positions.contains(&new_dragon_position)
            && !hideout_positions.contains(&new_dragon_position)
        {
            // eat a sheep
            let mut new_sheep_positions = sheep_positions.clone();
            new_sheep_positions.remove(&new_dragon_position);
            if new_sheep_positions.is_empty() {
                // all sheep eaten
                total_count += 1;
            } else {
                total_count += do_sheep_pass(
                    new_dragon_position,
                    new_sheep_positions,
                    hideout_positions,
                    w,
                    h,
                    sheep_cache,
                    dragon_cache,
                );
            }
        } else {
            total_count += do_sheep_pass(
                new_dragon_position,
                sheep_positions.clone(),
                hideout_positions,
                w,
                h,
                sheep_cache,
                dragon_cache,
            );
        }
    }

    dragon_cache.insert(
        (dragon_position, SortedSet::from(&sheep_positions)),
        total_count,
    );
    total_count
}

fn main() {
    let input_file = fs::read_to_string("everybody_codes_e2025_q10_p1.txt").unwrap();
    let r = solve(&input_file, 4);
    println!("Part 1: {}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q10_p2.txt").unwrap();
    let r = solve2(&input_file, 20);
    println!("Part 2: {}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q10_p3.txt").unwrap();
    let r = solve3(&input_file);
    println!("Part 3: {}", r);
}
