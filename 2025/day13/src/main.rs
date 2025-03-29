use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();

    // parse input
    let mut all_nodes = HashSet::new();
    let mut edges: HashMap<&str, Vec<(&str, u64)>> = HashMap::new();
    for l in lines {
        let (nodes, cost) = l.split_once(" | ").unwrap();
        let (a, b) = nodes.split_once(" -> ").unwrap();
        let cost = cost.parse::<u64>().unwrap();
        all_nodes.insert(a);
        all_nodes.insert(b);
        edges.entry(a).or_default().push((b, cost));
    }

    // parts 1 and 2
    for part in [1, 2] {
        let mut all_costs = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back(("STT", 0));
        while let Some((n, cost)) = queue.pop_front() {
            if all_costs.contains_key(&n) {
                continue;
            }
            all_costs.insert(n, cost);
            if let Some(es) = edges.get(n) {
                for e in es {
                    if part == 1 {
                        queue.push_back((e.0, cost + 1));
                    } else {
                        queue.push_back((e.0, cost + e.1));
                    }
                }
            }
        }

        let mut all_costs = all_costs.into_iter().collect::<Vec<_>>();
        all_costs.sort_unstable_by_key(|a| a.1);
        println!(
            "{}",
            all_costs[all_costs.len() - 3..]
                .iter()
                .map(|a| a.1)
                .product::<u64>()
        );
    }

    // part 3
    let mut max = 0;
    for start in all_nodes {
        let mut queue = VecDeque::new();
        queue.push_back((start, 0, HashSet::new()));
        while let Some((n, cost, seen)) = queue.pop_front() {
            if n == start && cost > 0 {
                max = max.max(cost);
                continue;
            }
            if seen.contains(&n) {
                continue;
            }
            if let Some(es) = edges.get(n) {
                for e in es {
                    let mut new_seen = seen.clone();
                    new_seen.insert(n);
                    queue.push_back((e.0, cost + e.1, new_seen));
                }
            }
        }
    }
    println!("{}", max);
}
