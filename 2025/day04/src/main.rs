use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();

    // part 1
    let mut part1 = 0u64;
    for &l in &lines {
        for c in l.trim().bytes() {
            part1 += (c - b'A' + 1) as u64;
        }
    }
    println!("{}", part1);

    // part 2
    let mut part2 = 0u64;
    for &l in &lines {
        let keep = l.len() / 10;
        let s = format!(
            "{}{}{}",
            &l[0..keep],
            l.len() - keep * 2,
            &l[l.len() - keep..]
        );
        for c in s.bytes() {
            if c.is_ascii_digit() {
                part2 += (c - b'0') as u64;
            } else {
                part2 += (c - b'A' + 1) as u64;
            }
        }
    }
    println!("{}", part2);

    // part 3
    let mut part3 = 0u64;
    for l in lines {
        let b = l.as_bytes();
        let mut i = 0;
        let mut s = String::new();
        while i < b.len() {
            let c = b[i];
            let mut len = 0;
            while i < b.len() && b[i] == c {
                i += 1;
                len += 1;
            }
            s.push_str(&format!("{}{}", len, c as char));
        }
        for c in s.bytes() {
            if c.is_ascii_digit() {
                part3 += (c - b'0') as u64;
            } else {
                part3 += (c - b'A' + 1) as u64;
            }
        }
    }
    println!("{}", part3);
}
