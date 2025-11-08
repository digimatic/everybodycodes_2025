use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"10,5,1,10,3,8,5,2,2";
    const EXAMPLE2_INPUT: &str = r"4,51,13,64,57,51,82,57,16,88,89,48,32,49,49,2,84,65,49,43,9,13,2,3,75,72,63,48,61,14,40,77";

    #[test]
    fn test1() {
        let r = solve(EXAMPLE1_INPUT);
        assert_eq!(29, r);
    }

    #[test]
    fn test2() {
        let r = solve2(EXAMPLE2_INPUT);
        assert_eq!(781, r);
    }

    #[test]
    fn test3() {
        let r = solve3(EXAMPLE2_INPUT);
        assert_eq!(3, r);
    }
}

fn solve(input_file: &str) -> usize {
    let lines = input_file.lines().collect::<Vec<_>>();
    let mut numbers = lines[0]
        .split(",")
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    numbers.sort();
    numbers.dedup();
    let sum: isize = numbers.iter().sum();
    sum as usize
}

fn solve2(input_file: &str) -> usize {
    let lines = input_file.lines().collect::<Vec<_>>();
    let mut numbers = lines[0]
        .split(",")
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    numbers.sort();
    numbers.dedup();
    let xs = numbers[0..20].to_vec();

    xs.iter().sum::<isize>() as usize
}

fn solve3(input_file: &str) -> usize {
    let lines = input_file.lines().collect::<Vec<_>>();
    let mut numbers = lines[0]
        .split(",")
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    numbers.sort();

    let mut count = 0;
    while !numbers.is_empty() {
        let mut ns = numbers.clone();
        ns.dedup();
        for n in ns {
            let i = numbers.iter().position(|&x| x == n).unwrap();
            numbers.remove(i);
        }
        count += 1;
    }
    count
}

fn main() {
    let input_file = fs::read_to_string("everybody_codes_e2025_q03_p1.txt").unwrap();
    let r = solve(&input_file);
    println!("{}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q03_p2.txt").unwrap();
    let r = solve2(&input_file);

    println!("{}", r);
    let input_file = fs::read_to_string("everybody_codes_e2025_q03_p3.txt").unwrap();
    let r = solve3(&input_file);
    println!("{}", r);
}
