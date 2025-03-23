use std::fs;

fn dist(island: (i64, i64), pos: (i64, i64)) -> i64 {
    (island.0 - pos.0).abs() + (island.1 - pos.1).abs()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();
    let mut islands = lines
        .iter()
        .map(|l| {
            let (x, y) = l[1..l.len() - 1].split_once(", ").unwrap();
            let x = x.parse::<i64>().unwrap();
            let y = y.parse::<i64>().unwrap();
            (x, y)
        })
        .collect::<Vec<_>>();

    // part 1
    let mut pos = (0, 0);
    islands.sort_by_key(|a| (dist(*a, pos), a.0, a.1));
    let part1_min = dist(islands[0], pos);
    let part1_max = dist(islands[islands.len() - 1], pos);

    // part 2
    pos = islands.swap_remove(0);
    islands.sort_by_key(|a| (dist(*a, pos), a.0, a.1));
    let part2 = dist(islands[0], pos);

    let mut part3 = part1_min + part2;
    pos = islands.swap_remove(0);
    while !islands.is_empty() {
        islands.sort_by_key(|a| (dist(*a, pos), a.0, a.1));
        part3 += dist(islands[0], pos);
        pos = islands.swap_remove(0);
    }

    println!("{}", part1_max - part1_min);
    println!("{}", part2);
    println!("{}", part3);
}
