use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();

    let numbers = lines[..lines.len() - 1]
        .iter()
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let instructions = lines[lines.len() - 1];

    // part 1
    let mut part1 = numbers[0];
    for (i, c) in instructions.chars().enumerate() {
        let n = numbers[i + 1];
        match c {
            '-' => part1 -= n,
            '+' => part1 += n,
            _ => unreachable!(),
        }
    }
    println!("{}", part1);

    // part 2
    let mut part2 = numbers[0];
    for (i, c) in instructions.chars().rev().enumerate() {
        let n = numbers[i + 1];
        match c {
            '-' => part2 -= n,
            '+' => part2 += n,
            _ => unreachable!(),
        }
    }
    println!("{}", part2);

    // part 3
    let mut part3 = numbers[0] * 10 + numbers[1];
    for (i, c) in instructions.chars().rev().enumerate() {
        if (i + 1) * 2 >= numbers.len() {
            break;
        }
        let n = numbers[(i + 1) * 2] * 10 + numbers[(i + 1) * 2 + 1];
        match c {
            '-' => part3 -= n,
            '+' => part3 += n,
            _ => unreachable!(),
        }
    }
    println!("{}", part3);
}
