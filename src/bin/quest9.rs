use std::{collections::HashSet, fs};

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"1:CAAGCGCTAAGTTCGCTGGATGTGTGCCCGCG
2:CTTGAATTGGGCCGTTTACCTGGTTTAACCAT
3:CTAGCGCTGAGCTGGCTGCCTGGTTGACCGCG";
    #[allow(dead_code)]
    const EXAMPLE2_INPUT: &str = r"1:GCAGGCGAGTATGATACCCGGCTAGCCACCCC
2:TCTCGCGAGGATATTACTGGGCCAGACCCCCC
3:GGTGGAACATTCGAAAGTTGCATAGGGTGGTG
4:GCTCGCGAGTATATTACCGAACCAGCCCCTCA
5:GCAGCTTAGTATGACCGCCAAATCGCGACTCA
6:AGTGGAACCTTGGATAGTCTCATATAGCGGCA
7:GGCGTAATAATCGGATGCTGCAGAGGCTGCTG";
    #[allow(dead_code)]
    const EXAMPLE3_INPUT: &str = r"1:GCAGGCGAGTATGATACCCGGCTAGCCACCCC
2:TCTCGCGAGGATATTACTGGGCCAGACCCCCC
3:GGTGGAACATTCGAAAGTTGCATAGGGTGGTG
4:GCTCGCGAGTATATTACCGAACCAGCCCCTCA
5:GCAGCTTAGTATGACCGCCAAATCGCGACTCA
6:AGTGGAACCTTGGATAGTCTCATATAGCGGCA
7:GGCGTAATAATCGGATGCTGCAGAGGCTGCTG";
    const EXAMPLE4_INPUT: &str = r"1:GCAGGCGAGTATGATACCCGGCTAGCCACCCC
2:TCTCGCGAGGATATTACTGGGCCAGACCCCCC
3:GGTGGAACATTCGAAAGTTGCATAGGGTGGTG
4:GCTCGCGAGTATATTACCGAACCAGCCCCTCA
5:GCAGCTTAGTATGACCGCCAAATCGCGACTCA
6:AGTGGAACCTTGGATAGTCTCATATAGCGGCA
7:GGCGTAATAATCGGATGCTGCAGAGGCTGCTG
8:GGCGTAAAGTATGGATGCTGGCTAGGCACCCG";

    #[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT);
        assert_eq!(414, r);
    }

    #[test]
    pub fn test2() {
        let r = solve2(EXAMPLE2_INPUT);
        assert_eq!(1245, r);
    }

    #[test]
    pub fn test3() {
        let r = solve3(EXAMPLE3_INPUT);
        assert_eq!(12, r);
    }

    #[test]
    pub fn test4() {
        let r = solve3(EXAMPLE4_INPUT);
        assert_eq!(36, r);
    }
}

fn get_parent_indices(child_index: usize) -> (usize, usize) {
    let mut parents = vec![0, 1, 2];
    parents.remove(child_index);
    let p1 = parents[0];
    let p2 = parents[1];
    (p1, p2)
}

fn is_child(index: usize, lines: &[Vec<char>]) -> bool {
    let (p1, p2) = get_parent_indices(index);
    for (i, &c) in lines[index].iter().enumerate() {
        let pc1 = lines[p1][i];
        let pc2 = lines[p2][i];

        if c != pc1 && c != pc2 {
            return false;
        }
    }
    true
}

fn compute_similarity(a: &[char], b: &[char]) -> usize {
    let mut similarity = 0;
    for i in 0..a.len() {
        if a[i] == b[i] {
            similarity += 1;
        }
    }
    similarity
}

fn solve(input_file: &str) -> usize {
    let lines = input_file
        .lines()
        .map(|x| x[2..].chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let child_index = vec![0, 1, 2]
        .into_iter()
        .find(|&x| is_child(x, &lines))
        .unwrap();

    let (p1, p2) = get_parent_indices(child_index);
    let similarity1 = compute_similarity(&lines[p1], &lines[child_index]);
    let similarity2 = compute_similarity(&lines[p2], &lines[child_index]);

    similarity1 * similarity2
}

fn is_child_v2(
    child_index: usize,
    parent1_index: usize,
    parent2_index: usize,
    lines: &[Vec<char>],
) -> bool {
    for (i, &c) in lines[child_index].iter().enumerate() {
        let pc1 = lines[parent1_index][i];
        let pc2 = lines[parent2_index][i];

        if c != pc1 && c != pc2 {
            return false;
        }
    }
    true
}

fn solve2(input_file: &str) -> usize {
    let lines = input_file
        .lines()
        .map(|x| {
            x.split(":").collect::<Vec<_>>()[1]
                .chars()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut total_similarity = 0;
    for child_index in 0..lines.len() {
        for parent1_index in 0..lines.len() {
            for parent2_index in (parent1_index + 1)..lines.len() {
                if child_index == parent1_index
                    || child_index == parent2_index
                    || parent1_index == parent2_index
                {
                    continue;
                }
                if is_child_v2(child_index, parent1_index, parent2_index, &lines) {
                    let sim1 = compute_similarity(&lines[parent1_index], &lines[child_index]);
                    let sim2 = compute_similarity(&lines[parent2_index], &lines[child_index]);
                    total_similarity += sim1 * sim2;
                }
            }
        }
    }
    total_similarity
}

fn solve3(input_file: &str) -> usize {
    let lines = input_file
        .lines()
        .map(|x| {
            x.split(":").collect::<Vec<_>>()[1]
                .chars()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut families: Vec<HashSet<usize>> = Vec::new();
    let add_family = |child_index,
                      parent1_index,
                      parent2_index,
                      families: &mut Vec<HashSet<usize>>| {
        let matching_families = families
            .iter()
            .enumerate()
            .filter(|(_i, f)| {
                f.contains(&child_index) || f.contains(&parent1_index) || f.contains(&parent2_index)
            })
            .collect::<Vec<_>>();
        if matching_families.is_empty() {
            families.push(HashSet::from([child_index, parent1_index, parent2_index]));
        } else {
            let mut new_family = HashSet::from([child_index, parent1_index, parent2_index]);
            let mut indices_to_remove = Vec::new();

            for (i, family) in matching_families {
                new_family.extend(family);
                indices_to_remove.push(i);
            }

            indices_to_remove.sort_by(|a, b| b.cmp(a));
            for i in indices_to_remove {
                families.remove(i);
            }

            families.push(new_family);
        }
    };

    for child_index in 0..lines.len() {
        for parent1_index in 0..lines.len() {
            for parent2_index in (parent1_index + 1)..lines.len() {
                if child_index == parent1_index
                    || child_index == parent2_index
                    || parent1_index == parent2_index
                {
                    continue;
                }
                if is_child_v2(child_index, parent1_index, parent2_index, &lines) {
                    add_family(child_index, parent1_index, parent2_index, &mut families);
                }
            }
        }
    }

    let f = families.iter().max_by_key(|f| f.len()).unwrap();
    f.iter().map(|v| v + 1).sum::<usize>()
}

fn main() {
    let input_file = fs::read_to_string("everybody_codes_e2025_q09_p1.txt").unwrap();
    let r = solve(&input_file);
    println!("Part 1: {}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q09_p2.txt").unwrap();
    let r = solve2(&input_file);
    println!("Part 2: {}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q09_p3.txt").unwrap();
    let r = solve3(&input_file);
    println!("Part 3: {}", r);
}
