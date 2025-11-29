use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs,
};

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"189482189843433862719
279415473483436249988
432746714658787816631
428219317375373724944
938163982835287292238
627369424372196193484
539825864246487765271
517475755641128575965
685934212385479112825
815992793826881115341
1737798467@7983146242
867597735651751839244
868364647534879928345
519348954366296559425
134425275832833829382
764324337429656245499
654662236199275446914
317179356373398118618
542673939694417586329
987342622289291613318
971977649141188759131";
    #[allow(dead_code)]
    const EXAMPLE2_INPUT: &str = r"4547488458944
9786999467759
6969499575989
7775645848998
6659696497857
5569777444746
968586@767979
6476956899989
5659745697598
6874989897744
6479994574886
6694118785585
9568991647449";
    #[allow(dead_code)]
    const EXAMPLE3_INPUT: &str = r"2645233S5466644
634566343252465
353336645243246
233343552544555
225243326235365
536334634462246
666344656233244
6426432@2366453
364346442652235
253652463426433
426666225623563
555462553462364
346225464436334
643362324542432
463332353552464";
    #[allow(dead_code)]
    const EXAMPLE4_INPUT: &str = r"";

    #[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT);
        assert_eq!(1573, r);
    }

    #[test]
    pub fn test2() {
        let r = solve2(EXAMPLE2_INPUT);
        assert_eq!(1090, r);
    }

    #[test]
    pub fn test3() {
        let r = solve3(EXAMPLE3_INPUT);
        assert_eq!(592, r);
    }
}

type Vec2i = (i32, i32);

fn solve(input_file: &str) -> u32 {
    let map = input_file
        .lines()
        .map(|row| {
            row.chars()
                .map(|d| d.to_digit(10).unwrap_or(0))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let h = map.len();
    let w = map[0].len();
    let center = (h / 2, w / 2);
    let max_r2 = 10 * 10;
    let mut sum = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            let dx = center.0 as i32 - x as i32;
            let dy = center.1 as i32 - y as i32;
            let r2 = dx * dx + dy * dy;
            if r2 <= max_r2 {
                sum += value;
            }
        }
    }
    sum
}

fn solve2(input_file: &str) -> u32 {
    let map = input_file
        .lines()
        .map(|row| {
            row.chars()
                .map(|d| d.to_digit(10).unwrap_or(0))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let h = map.len();
    let w = map[0].len();
    let center = (h / 2, w / 2);
    let mut max_sum = 0;
    let mut max_r = 0;
    for r in 1..=(w - 1) / 2 {
        let mut sum = 0;
        {
            let r = r as isize;
            let r2min = (r - 1) * (r - 1);
            let r2max = (r) * (r);
            for dy in -r..=r {
                for dx in -r..=r {
                    let r2 = dx * dx + dy * dy;
                    if r2 > r2min && r2 <= r2max {
                        sum += map[(center.0 as isize + dy) as usize]
                            [(center.1 as isize + dx) as usize];
                    }
                }
            }
        }
        if sum > max_sum {
            max_sum = sum;
            max_r = r;
        }
    }
    max_sum * max_r as u32
}

fn dijkstra(start_pos: Vec2i, cost_map: &[Vec<u32>], min_r: usize) -> Option<u32> {
    let max_cost = 30 * min_r as u32;
    let min_r = min_r as i32;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start_pos, true)));
    let w = cost_map[0].len() as i32;
    let h = cost_map.len() as i32;
    let center = (w / 2, h / 2);
    let mut costs = HashMap::new();

    while let Some(Reverse((cost, pos, mut first_half))) = heap.pop() {
        if cost > max_cost {
            continue;
        }
        if first_half && pos.0 == center.0 && pos.1 >= center.1 {
            first_half = false;
        } else if !first_half && pos == start_pos {
            return Some(cost);
        }

        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_pos = (pos.0 + dx, pos.1 + dy);
            if new_pos.0 >= w || new_pos.1 >= h || new_pos.0 < 0 || new_pos.1 < 0 {
                continue;
            }
            if first_half {
                // not allowed to go to quadrant lower right
                if new_pos.0 > center.0 && new_pos.1 >= center.1 {
                    continue;
                }
            } else {
                // not allowed to go to quadrant lower left
                if new_pos.0 < center.0 && new_pos.1 >= center.1 {
                    continue;
                }
            }
            let r2 = (center.0 - new_pos.0) * (center.0 - new_pos.0)
                + (center.1 - new_pos.1) * (center.1 - new_pos.1);
            if r2 <= (min_r - 1) * (min_r - 1) {
                continue;
            }
            let new_cost = cost + cost_map[new_pos.1 as usize][new_pos.0 as usize];
            let old_cost = costs
                .get(&(new_pos, first_half))
                .copied()
                .unwrap_or(u32::MAX);
            if new_cost < old_cost {
                costs.insert((new_pos, first_half), new_cost);
                heap.push(Reverse((new_cost, new_pos, first_half)));
            }
        }
    }
    None
}

fn solve3(input_file: &str) -> u32 {
    let cost_map = input_file
        .lines()
        .map(|row| {
            row.chars()
                .map(|d| d.to_digit(10).unwrap_or(0))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut start_pos = (0, 0);
    for (row, line) in input_file.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start_pos = (col as i32, row as i32);
                break;
            }
        }
    }

    for r in 1..=(cost_map[0].len() / 2) {
        let cost = dijkstra(start_pos, &cost_map, r);
        let max_cost = 30 * r as u32;
        if let Some(cost) = cost
            && cost <= max_cost
        {
            return ((r - 1) as u32) * cost;
        }
    }
    panic!("No solution found");
}

fn main() {
    let input_file = fs::read_to_string("everybody_codes_e2025_q17_p1.txt").unwrap();
    let r = solve(&input_file);
    println!("Part 1: {}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q17_p2.txt").unwrap();
    let r = solve2(&input_file);
    println!("Part 2: {}", r);

    let input_file = fs::read_to_string("everybody_codes_e2025_q17_p3.txt").unwrap();
    let r = solve3(&input_file);
    println!("Part 3: {}", r);
}
