use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"Vyrdax,Drakzyph,Fyrryn,Elarzris

R3,L2,R3,L1";

    const EXAMPLE3_INPUT: &str = r"Vyrdax,Drakzyph,Fyrryn,Elarzris

R3,L2,R3,L3";

    #[test]
    fn test1() {
        let r = solve(EXAMPLE1_INPUT);
        assert_eq!("Fyrryn", r);
    }

    #[test]
    fn test2() {
        let r = solve2(EXAMPLE1_INPUT);
        assert_eq!("Elarzris", r);
    }

    #[test]
    fn test3() {
        let r = solve3(EXAMPLE3_INPUT);
        assert_eq!("Drakzyph", r);
    }
}

fn solve(input_file: &str) -> String {
    let input = input_file.lines().collect::<Vec<_>>();
    let list_str = input[0];
    let instr_str = input[2];
    let names = list_str.split(",").collect::<Vec<_>>();
    let instructions = instr_str.split(",").collect::<Vec<_>>();

    let mut current: isize = 0;
    for instruction in instructions {
        let dir: char = instruction.chars().collect::<Vec<_>>()[0];
        let mut num: isize = instruction[1..].parse::<isize>().unwrap();
        if dir == 'L' {
            num = -num
        }
        current += num;
        if current < 0 {
            current = 0
        } else if current >= names.len() as isize {
            current = names.len() as isize - 1
        }
    }
    names[current as usize].to_string()
}

fn solve2(input_file: &str) -> String {
    let input = input_file.lines().collect::<Vec<_>>();
    let list_str = input[0];
    let instr_str = input[2];
    let names = list_str.split(",").collect::<Vec<_>>();
    let instructions = instr_str.split(",").collect::<Vec<_>>();

    let mut current: isize = 0;
    for instruction in instructions {
        let dir: char = instruction.chars().collect::<Vec<_>>()[0];
        let mut num: isize = instruction[1..].parse::<isize>().unwrap();
        if dir == 'L' {
            num = -num
        }
        current += num;
        current += names.len() as isize;
        current %= names.len() as isize;
    }
    names[current as usize].to_string()
}

fn solve3(input_file: &str) -> String {
    let input = input_file.lines().collect::<Vec<_>>();
    let list_str = input[0];
    let instr_str = input[2];
    let names = list_str.split(",").collect::<Vec<_>>();
    let instructions = instr_str.split(",").collect::<Vec<_>>();

    let mut name_indices = (0..names.len()).collect::<Vec<_>>();
    assert_eq!(names.len(), name_indices.len());
    for instruction in instructions {
        let dir: char = instruction.chars().collect::<Vec<_>>()[0];
        let mut current: isize = instruction[1..].parse::<isize>().unwrap();
        if dir == 'L' {
            current = names.len() as isize - current
        }

        current = ((current % names.len() as isize) + names.len() as isize) % names.len() as isize;
        current %= names.len() as isize;

        name_indices.swap(0, current as usize);
    }
    names[name_indices[0]].to_string()
}

fn main() {
    let input_file = fs::read_to_string("everybody_codes_e2025_q01_p1.txt").unwrap();
    let r = solve(&input_file);
    println!("{}", r);

    let input2_file = fs::read_to_string("everybody_codes_e2025_q01_p2.txt").unwrap();
    let r2 = solve2(&input2_file);
    println!("{}", r2);

    let input3_file = fs::read_to_string("everybody_codes_e2025_q01_p3.txt").unwrap();
    let r3 = solve3(&input3_file);
    println!("{}", r3);
}
