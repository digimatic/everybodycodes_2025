use std::{collections::VecDeque, fs};

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"58:5,3,7,8,9,10,4,5,7,8,8";
    const EXAMPLE2_INPUT: &str = r"1:2,4,1,1,8,2,7,9,8,6
2:7,9,9,3,8,3,8,8,6,8
3:4,7,6,9,1,8,3,7,2,2
4:6,4,2,1,7,4,5,5,5,8
5:2,9,3,8,3,9,5,2,1,4
6:2,4,9,6,7,4,1,7,6,8
7:2,3,7,6,2,2,4,1,4,2
8:5,1,5,6,8,3,1,8,3,9
9:5,7,7,3,7,2,3,8,6,7
10:4,1,9,3,8,5,4,3,5,5";
    const EXAMPLE3_INPUT: &str = r"1:7,1,9,1,6,9,8,3,7,2
2:6,1,9,2,9,8,8,4,3,1
3:7,1,9,1,6,9,8,3,8,3
4:6,1,9,2,8,8,8,4,3,1
5:7,1,9,1,6,9,8,3,7,3
6:6,1,9,2,8,8,8,4,3,5
7:3,7,2,2,7,4,4,6,3,1
8:3,7,2,2,7,4,4,6,3,7
9:3,7,2,2,7,4,1,6,3,7";
    const EXAMPLE4_INPUT: &str = r"1:7,1,9,1,6,9,8,3,7,2
2:7,1,9,1,6,9,8,3,7,2";

    #[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT);
        assert_eq!("581078", r);
    }

    #[test]
    pub fn test2() {
        let r = solve2(EXAMPLE2_INPUT);
        assert_eq!(77053, r);
    }

    #[test]
    pub fn test3() {
        let r = solve3(EXAMPLE3_INPUT);
        assert_eq!(260, r);
    }

    #[test]
    pub fn test4() {
        let r = solve3(EXAMPLE4_INPUT);
        assert_eq!(4, r);
    }
}

fn solve(input_file: &str) -> String {
    let lines = input_file.lines().collect::<Vec<_>>();
    let line = lines[0].split(":").collect::<Vec<_>>();
    let numbers_str = line[1];
    let mut xs = numbers_str
        .split(",")
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<VecDeque<_>>();
    let mut ys = Vec::new();
    let c = xs.pop_front().unwrap();
    ys.push((None, c, None));
    for x in xs {
        let mut done = false;
        for curr in ys.iter_mut() {
            if x < curr.1 && curr.0.is_none() {
                curr.0 = Some(x);
                done = true;
                break;
            } else if x > curr.1 && curr.2.is_none() {
                curr.2 = Some(x);
                done = true;
                break;
            }
        }
        if !done {
            ys.push((None, x, None));
        }
    }
    let mut rs = "".to_string();
    for (_, y, _) in ys {
        rs += &y.to_string();
    }

    rs
}

fn solve2(input_file: &str) -> isize {
    let lines = input_file.lines().collect::<Vec<_>>();
    let mut swords = Vec::new();

    for line in lines {
        let line = line.split(":").collect::<Vec<_>>();
        let numbers_str = line[1];
        let mut xs = numbers_str
            .split(",")
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<VecDeque<_>>();
        let mut ys = Vec::new();
        let c = xs.pop_front().unwrap();
        ys.push((None, c, None));
        for x in xs {
            let mut done = false;
            for curr in ys.iter_mut() {
                if x < curr.1 && curr.0.is_none() {
                    curr.0 = Some(x);
                    done = true;
                    break;
                } else if x > curr.1 && curr.2.is_none() {
                    curr.2 = Some(x);
                    done = true;
                    break;
                }
            }
            if !done {
                ys.push((None, x, None));
            }
        }
        let mut rs = "".to_string();
        for (_, y, _) in ys {
            rs += &y.to_string();
        }

        swords.push(rs.parse::<isize>().unwrap());
    }
    let x = swords.iter().min().unwrap();
    let y = swords.iter().max().unwrap();
    y - x
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Sword {
    quality: u64,
    fishbone: Vec<u64>,
    id: u64,
}

fn parse_sword(line: &str) -> Sword {
    let line = line.split(":").collect::<Vec<_>>();
    let numbers_str = line[1];
    let mut xs = numbers_str
        .split(",")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<VecDeque<_>>();
    let mut ys = Vec::new();
    let c = xs.pop_front().unwrap();
    ys.push((None, c, None));
    for x in xs {
        let mut done = false;
        for curr in ys.iter_mut() {
            if x < curr.1 && curr.0.is_none() {
                curr.0 = Some(x);
                done = true;
                break;
            } else if x > curr.1 && curr.2.is_none() {
                curr.2 = Some(x);
                done = true;
                break;
            }
        }
        if !done {
            ys.push((None, x, None));
        }
    }
    let mut rs = "".to_string();
    let mut fishbone = Vec::new();
    for (x, y, z) in ys {
        rs += &y.to_string();

        let mut v = 0;
        if let Some(xv) = x {
            v = xv;
        }
        v = 10 * v + y;
        if let Some(zv) = z {
            v = 10 * v + zv;
        }

        fishbone.push(v);
    }

    Sword {
        quality: rs.parse::<u64>().unwrap(),
        fishbone,
        id: line[0].parse::<u64>().unwrap(),
    }
}

fn solve3(input_file: &str) -> u64 {
    let lines = input_file.lines().collect::<Vec<_>>();

    let mut swords = lines.into_iter().map(parse_sword).collect::<Vec<_>>();
    swords.sort();

    swords
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, sword)| (i as u64 + 1) * sword.id)
        .sum()
}

fn main() {
    let input_file = fs::read_to_string("everybody_codes_e2025_q05_p1.txt").unwrap();
    let r = solve(&input_file);
    println!("{}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q05_p2.txt").unwrap();
    let r = solve2(&input_file);
    println!("{}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q05_p3.txt").unwrap();
    let r = solve3(&input_file);
    println!("{}", r);
}
