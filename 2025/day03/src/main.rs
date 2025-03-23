use std::fs;
use std::ops::Range;

fn merge(ranges: &mut Vec<Range<i64>>, mut m: Range<i64>) {
    let mut i = 0;
    while i < ranges.len() {
        if ranges[i].contains(&m.start) && ranges[i].contains(&(m.end - 1)) {
            return;
        }
        if m.contains(&ranges[i].start) && m.contains(&(ranges[i].end - 1)) {
            ranges.remove(i);
            continue;
        }
        if ranges[i].contains(&m.start) {
            m.start = ranges[i].end;
        } else if ranges[i].contains(&(m.end - 1)) {
            m.end = ranges[i].start;
        }
        i += 1;
    }
    ranges.push(m);
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let boxes = input
        .lines()
        .map(|l| {
            let (b1, b2) = l.split_once(" ").unwrap();
            let (b1a, b1b) = b1.split_once("-").unwrap();
            let (b2a, b2b) = b2.split_once("-").unwrap();
            let b1a = b1a.parse::<i64>().unwrap();
            let b1b = b1b.parse::<i64>().unwrap();
            let b2a = b2a.parse::<i64>().unwrap();
            let b2b = b2b.parse::<i64>().unwrap();
            ((b1a..b1b + 1), (b2a..b2b + 1))
        })
        .collect::<Vec<_>>();

    // part 1
    let mut part1 = 0;
    for b in &boxes {
        part1 += b.0.end - b.0.start + b.1.end - b.1.start;
    }
    println!("{}", part1);

    // part 2
    let mut part2 = 0;
    for b in &boxes {
        let mut ranges = vec![b.0.clone()];
        merge(&mut ranges, b.1.clone());
        for r in ranges {
            part2 += r.end - r.start;
        }
    }
    println!("{}", part2);

    // part 3
    let mut part3 = 0;
    for b in boxes.windows(2) {
        let mut ranges = vec![b[0].0.clone()];
        merge(&mut ranges, b[0].1.clone());
        merge(&mut ranges, b[1].0.clone());
        merge(&mut ranges, b[1].1.clone());
        let s = ranges.iter().map(|r| r.end - r.start).sum::<i64>();
        part3 = part3.max(s);
    }
    println!("{}", part3);
}
