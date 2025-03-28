use std::{collections::VecDeque, fs};

fn apply_arithmetic<F>(p: Vec<&str>, f: F, grid: &mut [Vec<i64>])
where
    F: Fn(&mut i64, i64),
{
    let amount = p[1].parse::<i64>().unwrap();
    if p[2] == "ALL" {
        for row in &mut *grid {
            for cell in row {
                f(cell, amount);
            }
        }
    } else if p[2] == "ROW" {
        let y = p[3].parse::<usize>().unwrap() - 1;
        for cell in &mut grid[y] {
            f(cell, amount);
        }
    } else {
        let x = p[3].parse::<usize>().unwrap() - 1;
        for row in &mut *grid {
            f(&mut row[x], amount);
        }
    }
}
fn apply_instruction(i: &str, grid: &mut [Vec<i64>]) {
    let p = i.split_ascii_whitespace().collect::<Vec<_>>();
    if i.starts_with("SHIFT") {
        let j = p[2].parse::<usize>().unwrap() - 1;
        let by = p[4].parse::<usize>().unwrap();
        if p[1] == "COL" {
            for _ in 0..by {
                let tmp = grid[grid.len() - 1][j];
                for y in (1..grid.len()).rev() {
                    grid[y][j] = grid[y - 1][j];
                }
                grid[0][j] = tmp;
            }
        } else {
            grid[j].rotate_right(by);
        }
    } else if i.starts_with("ADD") {
        apply_arithmetic(p, |a, b| *a += b, grid);
    } else if i.starts_with("SUB") {
        apply_arithmetic(p, |a, b| *a -= b, grid);
    } else if i.starts_with("MULTIPLY") {
        apply_arithmetic(p, |a, b| *a *= b, grid);
    }

    for row in &mut *grid {
        for cell in row {
            while *cell < 0 {
                *cell += 1073741824;
            }
            while *cell > 1073741823 {
                *cell -= 1073741824;
            }
        }
    }
}

fn apply_flow<'a>(
    f: &'a str,
    stash: Option<&'a str>,
    instructions: &mut VecDeque<&'a str>,
    grid: &mut [Vec<i64>],
) -> Option<&'a str> {
    if f == "TAKE" {
        instructions.pop_front()
    } else if f == "CYCLE" {
        instructions.push_back(stash.unwrap());
        None
    } else {
        apply_instruction(stash.unwrap(), grid);
        None
    }
}

fn max_sum(grid: &[Vec<i64>]) -> i64 {
    let mut max = 0;
    for row in grid {
        let mut sum = 0;
        for cell in row {
            sum += *cell;
        }
        max = max.max(sum);
    }
    for x in 0..grid[0].len() {
        let mut sum = 0;
        for row in grid {
            sum += row[x];
        }
        max = max.max(sum);
    }
    max
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let blocks = input.split("\n\n").collect::<Vec<_>>();
    let grid_block = blocks[0];
    let instructions = blocks[1].lines().collect::<VecDeque<_>>();
    let flow = blocks[2].lines().collect::<Vec<_>>();

    // parse grid
    let mut grid = Vec::new();
    for l in grid_block.lines() {
        let mut row = Vec::new();
        for n in l
            .split_ascii_whitespace()
            .map(|i| i.parse::<i64>().unwrap())
        {
            row.push(n);
        }
        grid.push(row);
    }

    // part 1
    let mut grid_part1 = grid.clone();
    for &i in &instructions {
        apply_instruction(i, &mut grid_part1);
    }
    println!("{}", max_sum(&grid_part1));

    // part 2
    let mut instructions_part2 = instructions.clone();
    let mut grid_part2 = grid.clone();
    let mut stash = None;
    for f in &flow {
        stash = apply_flow(f, stash, &mut instructions_part2, &mut grid_part2);
    }
    println!("{}", max_sum(&grid_part2));

    // part 3
    let mut instructions_part3 = instructions.clone();
    let mut grid_part3 = grid.clone();
    let mut stash = None;
    for f in flow.iter().cycle() {
        stash = apply_flow(f, stash, &mut instructions_part3, &mut grid_part3);
        if stash.is_none() && instructions_part3.is_empty() {
            break;
        }
    }
    println!("{}", max_sum(&grid_part3));
}
