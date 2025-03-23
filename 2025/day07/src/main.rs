use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();

    let numbers = lines[0..100]
        .iter()
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let instructions = lines.iter().skip(101).take(250).collect::<Vec<_>>();

    // part 1
    let mut part1_numbers = numbers.clone();
    for swap in &instructions {
        let (l, r) = swap.split_once("-").unwrap();
        let l = l.parse::<usize>().unwrap();
        let r = r.parse::<usize>().unwrap();
        part1_numbers.swap(l - 1, r - 1);
    }
    println!(
        "{}",
        part1_numbers[lines[lines.len() - 1].parse::<usize>().unwrap() - 1]
    );

    // part 2
    let mut part2_numbers = numbers.clone();
    for (i, swap) in instructions.iter().enumerate() {
        let (x, y) = swap.split_once("-").unwrap();
        let (z, _) = instructions[(i + 1) % instructions.len()]
            .split_once("-")
            .unwrap();
        let x = x.parse::<usize>().unwrap();
        let y = y.parse::<usize>().unwrap();
        let z = z.parse::<usize>().unwrap();
        let vz = part2_numbers[z - 1];
        part2_numbers[z - 1] = part2_numbers[y - 1];
        part2_numbers[y - 1] = part2_numbers[x - 1];
        part2_numbers[x - 1] = vz;
    }
    println!(
        "{}",
        part2_numbers[lines[lines.len() - 1].parse::<usize>().unwrap() - 1]
    );

    // part 3
    let mut part3_numbers = numbers.clone();
    for swap in &instructions {
        let (x, y) = swap.split_once("-").unwrap();
        let mut x = x.parse::<usize>().unwrap();
        let mut y = y.parse::<usize>().unwrap();
        if x > y {
            std::mem::swap(&mut x, &mut y);
        }
        let x_len = y - x;
        let y_len = part3_numbers.len() - y + 1;
        let len = x_len.min(y_len);
        for l in 0..len {
            part3_numbers.swap(x - 1 + l, y - 1 + l);
        }
    }
    println!(
        "{}",
        part3_numbers[lines[lines.len() - 1].parse::<usize>().unwrap() - 1]
    );
}
