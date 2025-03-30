use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

struct Item {
    quality: i64,
    cost: i64,
    unique_materials: i64,
}

#[derive(Copy, Clone, Default)]
struct Optimal {
    quality: i64,
    unique_materials: i64,
}

fn dfs(
    remaining_units: i64,
    items: &[Item],
    i: usize,
    cache: &mut HashMap<(i64, usize), Optimal>,
) -> Optimal {
    if remaining_units == 0 {
        return Optimal::default();
    }

    if let Some(c) = cache.get(&(remaining_units, i)) {
        return *c;
    }

    let mut quality = 0;
    let mut unique_materials = i64::MAX;
    for j in i..items.len() {
        if items[j].cost <= remaining_units {
            let r = dfs(remaining_units - items[j].cost, items, j + 1, cache);
            let new_quality = r.quality + items[j].quality;
            match new_quality.cmp(&quality) {
                Ordering::Greater => {
                    quality = new_quality;
                    unique_materials = r.unique_materials + items[j].unique_materials;
                }
                Ordering::Equal => {
                    unique_materials =
                        unique_materials.min(r.unique_materials + items[j].unique_materials);
                }
                Ordering::Less => {}
            }
        }
    }

    cache.insert(
        (remaining_units, i),
        Optimal {
            quality,
            unique_materials,
        },
    );

    Optimal {
        quality,
        unique_materials,
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();

    let mut items = Vec::new();
    for l in lines {
        let p = l.split_whitespace().collect::<Vec<_>>();
        let quality = p[5][0..p[5].len() - 1].parse::<i64>().unwrap();
        let cost = p[8][0..p[8].len() - 1].parse::<i64>().unwrap();
        let unique_materials = p[12].parse::<i64>().unwrap();
        items.push(Item {
            quality,
            cost,
            unique_materials,
        });
    }

    // part 1
    items.sort_unstable_by_key(|i| i.quality);
    items.reverse();
    println!(
        "{}",
        items[0..5].iter().map(|i| i.unique_materials).sum::<i64>()
    );

    // part 2
    let part2 = dfs(30, &items, 0, &mut HashMap::new());
    println!("{:?}", part2.quality * part2.unique_materials);

    // part 3
    let part3 = dfs(300, &items, 0, &mut HashMap::new());
    println!("{:?}", part3.quality * part3.unique_materials);
}
