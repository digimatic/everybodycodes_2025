use std::fmt;
use std::fs;
use std::ops::{Add, Div, Mul};

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"A=[25,9]";
    const EXAMPLE2_INPUT: &str = r"A=[35300,-64910]";

    #[test]
    fn test1() {
        let r = solve(EXAMPLE1_INPUT);
        assert_eq!("[357,862]", r);
    }

    #[test]
    fn test2() {
        assert_eq!(true, is_engraved(&Complex::new(35630, -64880)));

        assert_eq!(false, is_engraved(&Complex::new(35480, -64910)));

        let r = solve2(EXAMPLE2_INPUT);
        assert_eq!(4076, r);
    }

    #[test]
    fn test3() {
        let r = solve3(EXAMPLE2_INPUT);
        assert_eq!(406954, r);
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Complex {
    x: isize,
    y: isize,
}

impl Complex {
    pub fn new(x: isize, y: isize) -> Complex {
        Complex { x, y }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add for &Complex {
    type Output = Complex;

    fn add(self, other: &Complex) -> Complex {
        Complex {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul for &Complex {
    type Output = Complex;

    fn mul(self, other: &Complex) -> Complex {
        Complex {
            x: self.x * other.x - self.y * other.y,
            y: self.x * other.y + self.y * other.x,
        }
    }
}

impl Div for &Complex {
    type Output = Complex;

    fn div(self, other: &Complex) -> Complex {
        Complex {
            x: (self.x / other.x),
            y: (self.y / other.y),
        }
    }
}

impl Div for Complex {
    type Output = Complex;

    fn div(self, other: Complex) -> Complex {
        Complex {
            x: (self.x / other.x),
            y: (self.y / other.y),
        }
    }
}

fn solve(input_file: &str) -> String {
    let a = parse_input(input_file);

    let mut r = Complex { x: 0, y: 0 };
    for _i in 0..3 {
        let r1 = &r * &r;
        let r2 = r1 / Complex { x: 10, y: 10 };
        let r3 = &r2 + &a;
        r = r3;
    }
    format!("{}", &r)
}

fn parse_input(input_file: &str) -> Complex {
    let xs = input_file[3..input_file.len() - 1]
        .split(",")
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    let x = xs[0];
    let y = xs[1];
    Complex { x, y }
}

fn is_engraved(p: &Complex) -> bool {
    let mut r = Complex { x: 0, y: 0 };
    for _i in 0..100 {
        let r1 = &r * &r;
        let r2 = r1
            / Complex {
                x: 100000,
                y: 100000,
            };
        let r3 = &r2 + p;
        r = r3;
        if r.x > 1000000 || r.y > 1000000 || r.x < -1000000 || r.y < -1000000 {
            return false;
        }
    }
    true
}

fn solve2(input_file: &str) -> usize {
    let a = parse_input(input_file);

    // println!("A={}", &a);

    let p1 = &a + &Complex { x: 1000, y: 1000 };

    let mut count = 0;
    for x in (a.x..=p1.x).step_by(10) {
        for y in (a.y..=p1.y).step_by(10) {
            if is_engraved(&Complex { x, y }) {
                count += 1;
            }
        }
    }

    count
}

fn solve3(input_file: &str) -> usize {
    let a = parse_input(input_file);

    let p1 = &a + &Complex::new(1000, 1000);

    let mut count = 0;
    for x in a.x..=p1.x {
        for y in a.y..=p1.y {
            if is_engraved(&Complex { x, y }) {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let input_file = fs::read_to_string("everybody_codes_e2025_q02_p1.txt").unwrap();
    let r = solve(&input_file);
    println!("{}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q02_p2.txt").unwrap();
    let r = solve2(&input_file);
    println!("{}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q02_p3.txt").unwrap();
    let r = solve3(&input_file);
    println!("{}", r);
}
