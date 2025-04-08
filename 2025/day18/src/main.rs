use regex::Regex;
use std::collections::{HashSet, VecDeque};
use std::fs;

fn main() {
    let range_x = 0..=9;
    let range_y = 0..=14;
    let range_z = 0..=59;
    let range_a = -1..=1;

    let width = range_x.end() - range_x.start() + 1;
    let height = range_y.end() - range_y.start() + 1;
    let depth = range_z.end() - range_z.start() + 1;

    // parse input
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();
    let re =
        Regex::new(r"RULE \d+: (\d+)x\+(\d+)y\+(\d+)z\+(\d+)a DIVIDE (\d+) HAS REMAINDER (\d+) \| DEBRIS VELOCITY \((-?\d+), (-?\d+), (-?\d+), (-?\d+)\)")
            .unwrap();

    let mut debris = Vec::new();
    for l in lines {
        let captures = re.captures(l).unwrap();
        let fx = captures[1].parse::<i64>().unwrap();
        let fy = captures[2].parse::<i64>().unwrap();
        let fz = captures[3].parse::<i64>().unwrap();
        let fa = captures[4].parse::<i64>().unwrap();
        let divide = captures[5].parse::<i64>().unwrap();
        let remainder = captures[6].parse::<i64>().unwrap();
        let vx = captures[7].parse::<i64>().unwrap();
        let vy = captures[8].parse::<i64>().unwrap();
        let vz = captures[9].parse::<i64>().unwrap();
        let va = captures[10].parse::<i64>().unwrap();

        for x in range_x.clone() {
            for y in range_y.clone() {
                for z in range_z.clone() {
                    for a in range_a.clone() {
                        if (x * fx + y * fy + z * fz + a * fa).rem_euclid(divide) == remainder {
                            debris.push((x, y, z, a, vx, vy, vz, va));
                        }
                    }
                }
            }
        }
    }

    // part 1
    println!("{}", debris.len());

    // IMPORTANT performance optimization for parts 2 and 3: Simulate the first
    // 1000 time steps and store the amount of debris per cell and time step.
    // 1000 steps are more than enough for my puzzle input. If it doesn't work
    // for you, increase that number.
    let mut blocked =
        vec![vec![vec![vec![0; 1000]; depth as usize]; height as usize]; width as usize];

    for t in 0..1000 {
        for d in &debris {
            let a = (d.3 + d.7 * t + 1).rem_euclid(range_a.end() - range_a.start() + 1) - 1;
            if a == 0 {
                let x = (d.0 + d.4 * t).rem_euclid(width);
                let y = (d.1 + d.5 * t).rem_euclid(height);
                let z = (d.2 + d.6 * t).rem_euclid(depth);
                blocked[x as usize][y as usize][z as usize][t as usize] += 1;
            }
        }
    }

    // parts 2 and 3 - simple BFS
    for part in [2, 3] {
        let mut queue = VecDeque::new();
        queue.push_back((0, if part == 2 { 0 } else { 3 }, 0, 0, 0));
        let mut seen = HashSet::new();
        while let Some((seconds, hits_left, x, y, z)) = queue.pop_front() {
            if x == *range_x.end() && y == *range_y.end() && z == *range_z.end() {
                println!("{}", seconds);
                break;
            }

            for new_pos in [
                (x + 1, y, z),
                (x - 1, y, z),
                (x, y + 1, z),
                (x, y - 1, z),
                (x, y, z + 1),
                (x, y, z - 1),
                (x, y, z),
            ] {
                if range_x.contains(&new_pos.0)
                    && range_y.contains(&new_pos.1)
                    && range_z.contains(&new_pos.2)
                {
                    let new_seconds = seconds + 1;

                    let hits = if new_pos != (0, 0, 0) {
                        blocked[new_pos.0 as usize][new_pos.1 as usize][new_pos.2 as usize]
                            [new_seconds as usize]
                    } else {
                        0
                    };

                    if hits <= hits_left {
                        let n = (
                            new_seconds,
                            hits_left - hits,
                            new_pos.0,
                            new_pos.1,
                            new_pos.2,
                        );
                        if seen.insert(n) {
                            queue.push_back(n);
                        }
                    }
                }
            }
        }
    }
}
