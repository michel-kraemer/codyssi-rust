use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();
    let add = lines[0][16..].parse::<u64>().unwrap();
    let multiply = lines[1][21..].parse::<u64>().unwrap();
    let power = lines[2][34..].parse::<u64>().unwrap();

    let mut numbers = lines[4..]
        .iter()
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    numbers.sort_unstable();

    // part 1
    let median = numbers[numbers.len() / 2];
    println!("{}", median.pow(power as u32) * multiply + add);

    // part 2
    let mut sum = 0;
    for &n in numbers.iter().filter(|n| *n % 2 == 0) {
        sum += n;
    }
    println!("{}", sum.pow(power as u32) * multiply + add);

    // part 3
    let mut max_price = 0;
    let mut max_room = 0;
    for n in numbers {
        let price = n.pow(power as u32) * multiply + add;
        if price <= 15000000000000 && price > max_price {
            max_price = price;
            max_room = n;
        }
    }
    println!("{}", max_room);
}
