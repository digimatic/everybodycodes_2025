use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"Oronris,Urakris,Oroneth,Uraketh

r > a,i,o
i > p,w
n > e,r
o > n,m
k > f,r
a > k
U > r
e > t
O > r
t > h";
    #[allow(dead_code)]
    const EXAMPLE2_INPUT: &str = r"Xanverax,Khargyth,Nexzeth,Helther,Braerex,Tirgryph,Kharverax

r > v,e,a,g,y
a > e,v,x,r
e > r,x,v,t
h > a,e,v
g > r,y
y > p,t
i > v,r
K > h
v > e
B > r
t > h
N > e
p > h
H > e
l > t
z > e
X > a
n > v
x > z
T > i";
    #[allow(dead_code)]
    const EXAMPLE3_INPUT: &str = r"Xaryt

X > a,o
a > r,t
r > y,e,a
h > a,e,v
t > h
v > e
y > p,t";

    #[allow(dead_code)]
    const EXAMPLE4_INPUT: &str = r"Khara,Xaryt,Noxer,Kharax

r > v,e,a,g,y
a > e,v,x,r,g
e > r,x,v,t
h > a,e,v
g > r,y
y > p,t
i > v,r
K > h
v > e
B > r
t > h
N > e
p > h
H > e
l > t
z > e
X > a
n > v
x > z
T > i";

    #[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT);
        assert_eq!("Oroneth", r);
    }

    #[test]
    pub fn test2() {
        let r = solve2(EXAMPLE2_INPUT);
        assert_eq!(23, r);
    }

    #[test]
    pub fn test3() {
        let r = solve3(EXAMPLE3_INPUT);
        assert_eq!(25, r);
    }

    #[test]
    pub fn test4() {
        let r = solve3(EXAMPLE4_INPUT);
        assert_eq!(1154, r);
    }
}

fn parse(input: &str) -> (Vec<Vec<char>>, HashMap<char, HashSet<char>>) {
    let lines = input.lines().collect::<Vec<_>>();
    let names = lines[0]
        .split(",")
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut rules = HashMap::new();
    for rule in lines[2..].iter() {
        let c = rule.chars().collect::<Vec<_>>()[0];
        let letters = rule[4..]
            .split(",")
            .map(|s| s.chars().collect::<Vec<_>>()[0])
            .collect::<HashSet<_>>();
        rules.insert(c, letters);
    }
    (names, rules)
}

fn check_name(name: &[char], rules: &HashMap<char, HashSet<char>>) -> bool {
    for (c1, c2) in name.iter().tuple_windows() {
        let r = rules.get(c1);
        if let Some(letters) = r {
            if !letters.contains(c2) {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn solve(input_file: &str) -> String {
    let (names, rules) = parse(input_file);
    names
        .iter()
        .find(|name| check_name(name, &rules))
        .unwrap()
        .iter().copied()
        .collect::<String>()
}

fn solve2(input_file: &str) -> usize {
    let (names, rules) = parse(input_file);
    names
        .iter()
        .enumerate()
        .map(|(index, name)| {
            if check_name(name, &rules) {
                1 + index
            } else {
                0
            }
        })
        .sum()
}

fn generate_words(name: &[char], rules: &HashMap<char, HashSet<char>>) -> HashSet<String> {
    let mut q: VecDeque<Vec<char>> = VecDeque::new();
    q.push_back(name.to_vec());
    let mut generated_words = HashSet::new();
    while !q.is_empty() {
        let name = q.pop_front().unwrap();
        if name.len() >= 7 && name.len() <= 11 {
            let word = name.iter().collect::<String>();
            generated_words.insert(word);
        }
        if name.len() == 11 {
            continue;
        }
        let c1 = name.last().unwrap();
        let next_list = rules.get(c1);
        if let Some(next_list) = next_list {
            for c2 in next_list {
                let mut name2 = name.clone();
                name2.push(*c2);
                q.push_back(name2);
            }
        }
    }
    generated_words
}

fn solve3(input_file: &str) -> usize {
    let (names, rules) = parse(input_file);
    let mut generated_words = HashSet::new();

    for name in &names {
        if check_name(name, &rules) {
            let new_generated_words = generate_words(name, &rules);
            generated_words.extend(new_generated_words);
        }
    }
    generated_words.len()
}

fn main() {
    let input_file = fs::read_to_string("everybody_codes_e2025_q07_p1.txt").unwrap();
    let r = solve(&input_file);
    println!("{}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q07_p2.txt").unwrap();
    let r = solve2(&input_file);
    println!("{}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q07_p3.txt").unwrap();
    let r = solve3(&input_file);
    println!("{}", r);
}
