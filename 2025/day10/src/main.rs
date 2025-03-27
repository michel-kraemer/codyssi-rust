use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut grid = Vec::new();
    let mut height = 0;
    for l in input.lines() {
        for n in l.split_whitespace() {
            grid.push(n.parse::<u64>().unwrap());
        }
        height += 1;
    }
    let width = grid.len() / height;

    // part 1
    let mut min = u64::MAX;
    for y in 0..height {
        let mut sum = 0;
        for x in 0..width {
            sum += grid[y * width + x];
        }
        min = min.min(sum);
    }
    for x in 0..width {
        let mut sum = 0;
        for y in 0..height {
            sum += grid[y * width + x];
        }
        min = min.min(sum);
    }
    println!("{}", min);

    // parts 2 and 3
    for part in [2, 3] {
        let mut queue = BinaryHeap::new();
        queue.push(Reverse((grid[0], 0, 0)));
        let mut seen = vec![u64::MAX; width * height];
        seen[0] = grid[0];
        while let Some(Reverse((score, x, y))) = queue.pop() {
            if (part == 2 && x == 14 && y == 14) || (part == 3 && x == width - 1 && y == height - 1)
            {
                println!("{}", score);
                break;
            }
            if x < width - 1 {
                let new_score = score + grid[y * width + x + 1];
                if seen[y * width + x + 1] > new_score {
                    seen[y * width + x + 1] = new_score;
                    queue.push(Reverse((new_score, x + 1, y)));
                }
            }
            if y < height - 1 {
                let new_score = score + grid[(y + 1) * width + x];
                if seen[(y + 1) * width + x] > new_score {
                    seen[(y + 1) * width + x] = new_score;
                    queue.push(Reverse((new_score, x, y + 1)));
                }
            }
        }
    }
}
