use everybodycodes_2025::parse_utils;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"1,2,3,5,9";
    const EXAMPLE2_INPUT: &str = r"1,2,2,2,2,3,1,2,3,3,1,3,1,2,3,2,1,4,1,3,2,2,1,3,2,2";

    #[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT, 90);
        assert_eq!(193, r);
    }

    #[test]
    pub fn test2() {
        let r = solve2(EXAMPLE2_INPUT);
        assert_eq!(270, r);
    }

    #[test]
    pub fn test3() {
        assert_eq!(1, solve3(EXAMPLE2_INPUT, 1));
        assert_eq!(5, solve3(EXAMPLE2_INPUT, 10));
        assert_eq!(47, solve3(EXAMPLE2_INPUT, 100));
        assert_eq!(467, solve3(EXAMPLE2_INPUT, 1000));
        assert_eq!(4664, solve3(EXAMPLE2_INPUT, 10000));
        assert_eq!(94439495762954, solve3(EXAMPLE2_INPUT, 202520252025000));
    }
}

fn solve(input_file: &str, column_count: usize) -> usize {
    let xs = parse_utils::parse_usize_numbers(input_file);
    let mut columns = vec![0; column_count];
    for x in xs {
        let mut i = x - 1;
        while i < column_count {
            columns[i] += 1;
            i += x;
        }
    }
    columns.iter().sum()
}

fn remove_factor(xs: &[usize], factor: usize) -> Option<Vec<usize>> {
    let ys = xs
        .iter()
        .enumerate()
        .map(|(i, &x)| {
            if (i + 1) % factor == 0 {
                if x > 0 { Some(x - 1) } else { None }
            } else {
                Some(x)
            }
        })
        .collect::<Vec<_>>();
    if ys.iter().all(|&x| x.is_some()) {
        Some(ys.into_iter().map(|x| x.unwrap()).collect::<Vec<_>>())
    } else {
        None
    }
}

fn solve2(input_file: &str) -> usize {
    let xs = parse_utils::parse_usize_numbers(input_file);
    let factors = find_factors(&xs);
    println!("factors: {:?}", factors);
    factors.iter().product()
}

fn find_factors(xs: &[usize]) -> Vec<usize> {
    let mut xs = xs.to_vec();
    let mut factors = Vec::new();
    for factor in 1..xs.len() {
        if let Some(ys) = remove_factor(&xs, factor) {
            factors.push(factor);
            xs = ys;
        }
    }
    factors
}

fn count_blocks(spell: &[usize], len: usize) -> usize {
    let mut count = 0;
    for x in spell {
        count += len / x;
    }
    count
}

fn binary_search(low: usize, high: usize, spell: &[usize], max_blocks: usize) -> usize {
    if low == high {
        return low;
    }
    if high - low == 1 {
        let blocks2 = count_blocks(spell, high);
        if blocks2 > max_blocks {
            return low;
        }
        return high;
    }

    let mid = (low + high) / 2;
    let blocks = count_blocks(spell, mid);
    if blocks < max_blocks {
        binary_search(mid, high, spell, max_blocks)
    } else if blocks == max_blocks {
        mid
    } else {
        binary_search(low, mid, spell, max_blocks)
    }
}

fn solve3(input_file: &str, available_blocks: usize) -> usize {
    let xs = parse_utils::parse_usize_numbers(input_file);
    let factors = find_factors(&xs);
    binary_search(1, available_blocks, &factors, available_blocks)
}

fn main() {
    let input_file = fs::read_to_string("everybody_codes_e2025_q16_p1.txt").unwrap();
    let r = solve(&input_file, 90);
    println!("Part 1: {}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q16_p2.txt").unwrap();
    let r = solve2(&input_file);
    println!("Part 2: {}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q16_p3.txt").unwrap();
    let r = solve3(&input_file, 202520252025000);
    println!("Part 3: {}", r);
}
