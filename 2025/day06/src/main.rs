use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut part1 = 0i64;
    let mut part2 = 0i64;
    let mut part3 = 0i64;
    let mut prev = 0;
    for c in input.trim().bytes() {
        if c.is_ascii_alphabetic() {
            part1 += 1;
        }
        if c.is_ascii_lowercase() {
            prev = ((c - b'a') + 1) as i64;
            part2 += prev;
            part3 += prev;
        } else if c.is_ascii_uppercase() {
            prev = ((c - b'A') + 27) as i64;
            part2 += prev;
            part3 += prev;
        } else {
            prev = prev * 2 - 5;
            while prev < 1 {
                prev += 52;
            }
            while prev > 52 {
                prev -= 52;
            }
            part3 += prev;
        }
    }

    println!("{}", part1);
    println!("{}", part2);
    println!("{}", part3);
}
