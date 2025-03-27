use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();

    // part 1
    let mut max = 0;
    let mut sum = 0;
    for l in lines {
        let (n, base) = l.split_once(" ").unwrap();
        let base = base.parse::<u64>().unwrap();
        let mut m = 0;
        for c in n.chars() {
            m *= base;
            if c.is_ascii_digit() {
                m += c.to_digit(10).unwrap() as u64;
            } else if c.is_ascii_uppercase() {
                m += ((c as u8) - b'A' + 10) as u64;
            } else if c.is_ascii_lowercase() {
                m += ((c as u8) - b'a' + 36) as u64;
            }
        }
        sum += m;
        max = max.max(m);
    }
    println!("{}", max);

    // part 2
    let mut s = sum;
    let mut base68sum = String::new();
    while s > 0 {
        let d = (s % 68) as u8;
        let c = if d < 10 {
            b'0' + d
        } else if (10..=35).contains(&d) {
            b'A' + (d - 10)
        } else if (36..=61).contains(&d) {
            b'a' + (d - 36)
        } else if d == 62 {
            b'!'
        } else if d == 63 {
            b'@'
        } else if d == 64 {
            b'#'
        } else if d == 65 {
            b'$'
        } else if d == 66 {
            b'%'
        } else if d == 67 {
            b'^'
        } else {
            panic!()
        };
        base68sum.insert(0, c as char);
        s /= 68;
    }
    println!("{}", base68sum);

    // part 3
    for base in 2.. {
        let mut count = 0;
        let mut s = sum;
        while s > 0 {
            s /= base;
            count += 1;
        }
        if count <= 4 {
            println!("{}", base);
            break;
        }
    }
}
