use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();

    // part 1
    let mut part1 = 0;
    for &l in &lines {
        part1 += l.chars().filter(|c| c.is_ascii_alphabetic()).count();
    }
    println!("{}", part1);

    // part 2 and 3
    for part in [2, 3] {
        let mut total = 0;

        for &l in &lines {
            let mut cs = l.chars().collect::<Vec<_>>();
            'l: loop {
                for i in 0..cs.len() {
                    if cs[i].is_ascii_digit() {
                        if i > 0 && (cs[i - 1].is_alphabetic() || (part == 2 && cs[i - 1] == '-')) {
                            cs.remove(i - 1);
                            cs.remove(i - 1);
                            continue 'l;
                        } else if i < cs.len() - 1
                            && (cs[i + 1].is_alphabetic() || (part == 2 && cs[i + 1] == '-'))
                        {
                            cs.remove(i);
                            cs.remove(i);
                            continue 'l;
                        }
                    }
                }
                break;
            }
            total += cs.len();
        }

        println!("{}", total);
    }
}
