use std::{
    collections::{HashSet, VecDeque},
    fs,
};

use everybodycodes_2025::sorted_set::SortedSet;

//#[cfg(test)]
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

    //#[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT, 3);
        assert_eq!(27, r);
    }

    //#[test]
    pub fn test2() {
        let r = solve2(EXAMPLE2_INPUT, 3);
        assert_eq!(27, r);
    }

    //#[test]
    pub fn test3() {
        let r = solve3(EXAMPLE3_INPUT);
        assert_eq!(15, r);
    }

    //#[test]
    pub fn test4() {
        let r = solve3(EXAMPLE4_INPUT);
        assert_eq!(8, r);
    }
    pub fn test5() {
        let r = solve3(EXAMPLE5_INPUT);
        assert_eq!(44, r);
    }
    pub fn test6() {
        let r = solve3(EXAMPLE6_INPUT);
        assert_eq!(4406, r);
    }
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
    .filter(|(x, y)| *x >= 0 && *x < w as i32 && *y >= 0 && *y < h as i32)
    .collect()
}

fn solve(input_file: &str, max_steps: usize) -> usize {
    let board = input_file
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let w = board[0].len() as i32;
    let h = board.len() as i32;
    let initial_position: Vec2i = ((w / 2) as i32, (h / 2) as i32);
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
    for round in 0..max_steps {
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
    board: &Vec<Vec<char>>,
    w: i32,
    h: i32,
) -> (
    HashSet<(i32, i32)>,
    HashSet<(i32, i32)>,
    HashSet<(i32, i32)>,
) {
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

fn read_board(position: Vec2i, board: &Vec<Vec<char>>) -> char {
    let (x, y) = position;
    board[y as usize][x as usize]
}

fn make_move(sheep_turn: bool, pos: Vec2i) -> String {
    let x_char = (b'A' + pos.0 as u8) as char;
    let y_num = pos.1 + 1;
    if sheep_turn {
        format!("S->{}{}", x_char, y_num)
    } else {
        format!("D->{}{}", x_char, y_num)
    }
}

fn solve3(input_file: &str) -> usize {
    let (board, w, h) = parse_board(input_file);
    let (sheep_positions, dragon_positions, hideout_positions) = extract_positions(&board, w, h);

    // let sheep_positions = SortedSet::from(&sheep_positions);
    let mut possible_count = 0;

    let mut stack = Vec::new();
    stack.push((
        true,
        dragon_positions.iter().next().unwrap().clone(),
        sheep_positions.clone(),
        // Vec::new(),
    ));
    // while let Some((sheep_turn, dragon_position, ref sheep_positions, moves)) = stack.pop() {
    while let Some((sheep_turn, dragon_position, ref sheep_positions)) = stack.pop() {
        // println!(
        //     "Round: {} {:?} {:?}, {:?}",
        //     sheep_turn, &dragon_position, &sheep_positions, &moves
        // );
        if sheep_turn {
            let mut any_sheep_moved = false;
            for sheep_position in sheep_positions.clone().into_iter() {
                let new_sheep_position = (sheep_position.0, sheep_position.1 + 1);
                if new_sheep_position.1 == h {
                    // sheep got to bottom.
                    // println!("A sheep escaped!)");
                    any_sheep_moved = true;
                } else if new_sheep_position == dragon_position
                    && !hideout_positions.contains(&new_sheep_position)
                {
                    // dont move into dragon (unless it's the hideout)
                } else {
                    let mut new_sheep_positions = sheep_positions.clone();
                    new_sheep_positions.remove(&sheep_position);
                    new_sheep_positions.insert(new_sheep_position);
                    any_sheep_moved = true;
                    // let mut new_moves = moves.clone();
                    // new_moves.push(make_move(sheep_turn, new_sheep_position));
                    // stack.push((false, dragon_position, new_sheep_positions, new_moves));
                    stack.push((false, dragon_position, new_sheep_positions));
                }
            }
            if !any_sheep_moved {
                // will never happen
                println!("No sheep moved this turn");
                // stack.push((false, dragon_position, sheep_positions.clone(), moves));
                stack.push((false, dragon_position, sheep_positions.clone()));
            }
        } else {
            let new_dragon_positions = next(dragon_position, w, h);
            for dragon_position in new_dragon_positions.into_iter() {
                if sheep_positions.contains(&dragon_position)
                    && !hideout_positions.contains(&dragon_position)
                {
                    // eat a sheep
                    let mut new_sheep_positions = sheep_positions.clone();
                    new_sheep_positions.remove(&dragon_position);
                    if new_sheep_positions.is_empty() {
                        // println!("All sheep eaten!");
                        // issue a solution
                        possible_count += 1;
                    } else {
                        // let mut new_moves = moves.clone();
                        // new_moves.push(make_move(sheep_turn, dragon_position));
                        // stack.push((true, dragon_position, new_sheep_positions, new_moves));
                        stack.push((true, dragon_position, new_sheep_positions));
                    }
                } else {
                    // let mut new_moves = moves.clone();
                    // new_moves.push(make_move(sheep_turn, dragon_position));
                    // stack.push((true, dragon_position, sheep_positions.clone(), new_moves));
                    stack.push((true, dragon_position, sheep_positions.clone()));
                }
            }
        }
    }

    println!("Possible count: {}", &possible_count);
    possible_count
}

fn main() {
    // tests::test1();
    // tests::test2();
    println!("Test 3");
    tests::test3();
    println!("Test 4");
    tests::test4();

    println!("Test 5");
    tests::test5();
    println!("Test 6");
    tests::test6();
    println!("Test 7");
    tests::test7();
    //
    // let input_file = fs::read_to_string("everybody_codes_e2025_q10_p1.txt").unwrap();
    // let r = solve(&input_file, 4);
    // println!("Part 1: {}", r);
    // //
    // let input_file = fs::read_to_string("everybody_codes_e2025_q10_p2.txt").unwrap();
    // let r = solve2(&input_file, 20);
    // println!("Part 2: {}", r);
    //
    // let input_file = fs::read_to_string("everybody_codes_e2025_q10_p3.txt").unwrap();
    // let r = solve3(&input_file);
    // println!("Part 3: {}", r);
}
