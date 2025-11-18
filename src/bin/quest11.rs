use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"9
1
1
4
9
6";
    const EXAMPLE2_INPUT: &str = r"805
706
179
48
158
150
232
885
598
524
423";

    #[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT);
        assert_eq!(109, r);
    }

    #[test]
    pub fn test2() {
        let r = solve2(EXAMPLE1_INPUT);
        assert_eq!(11, r);
    }

    #[test]
    pub fn test3() {
        let r = solve2(EXAMPLE2_INPUT);
        assert_eq!(1579, r);
    }
}

fn parse(input_file: &str) -> Vec<u64> {
    input_file
        .lines()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
}

fn phase1(xs: &mut [u64]) -> u64 {
    let mut round = 0;
    loop {
        let mut moved_this_round = false;
        // println!("Checksum phase 1 - round {}, {}", round, checksum(&xs));
        for i in 0..xs.len() - 1 {
            if xs[i] > xs[i + 1] {
                xs[i] -= 1;
                xs[i + 1] += 1;
                moved_this_round = true;
            }
        }
        if !moved_this_round {
            break;
        }
        round += 1;
    }
    round
}

fn phase2(xs: &mut [u64], start_round: u64, max_rounds: Option<u64>) -> u64 {
    let mut round = start_round;
    loop {
        // println!("Checksum phase 2 - round {}, {}", round, checksum(&xs));
        let mut moved_this_round = false;
        for i in 0..xs.len() - 1 {
            if xs[i] < xs[i + 1] {
                xs[i] += 1;
                xs[i + 1] -= 1;
                moved_this_round = true;
            }
        }
        round += 1;
        if !moved_this_round || (max_rounds.is_some() && round == max_rounds.unwrap()) {
            round -= 1;
            break;
        }
    }
    // println!("Checksum phase 2 - round {}, {}", round, checksum(&xs));
    round
}

fn checksum(xs: &[u64]) -> u64 {
    xs.iter()
        .enumerate()
        .map(|(i, x)| *x * (1 + i) as u64)
        .sum()
}

fn solve(input_file: &str) -> u64 {
    let mut xs = parse(input_file);
    let round = phase1(&mut xs);
    phase2(&mut xs, round, Some(10));
    checksum(&xs)
}

fn solve2(input_file: &str) -> u64 {
    let mut xs = parse(input_file);
    let round = phase1(&mut xs);
    phase2(&mut xs, round, None)
}

fn solve3(input_file: &str) -> u64 {
    let xs = parse(input_file);
    // xs is sorted, so phase1 is not needed
    let avg = xs.iter().sum::<u64>() / (xs.len() as u64);
    xs.iter().map(|&v| v.saturating_sub(avg)).sum()
}

fn main() {
    let input_file = fs::read_to_string("everybody_codes_e2025_q11_p1.txt").unwrap();
    let r = solve(&input_file);
    println!("Part 1: {}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q11_p2.txt").unwrap();
    let r = solve2(&input_file);
    println!("Part 2: {}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q11_p3.txt").unwrap();
    let r = solve3(&input_file);
    println!("Part 3: {}", r);
}
