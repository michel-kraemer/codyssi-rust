use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Node<'a> {
    code: &'a str,
    id: u64,
    left: Option<usize>,
    right: Option<usize>,
    layer: usize,
}

fn layer_sums(i: usize, nodes: &[Node], sums: &mut HashMap<usize, u64>) {
    *sums.entry(nodes[i].layer).or_default() += nodes[i].id;
    if let Some(j) = nodes[i].left {
        layer_sums(j, nodes, sums);
    }
    if let Some(j) = nodes[i].right {
        layer_sums(j, nodes, sums);
    }
}

fn find_path<'a>(nodes: &'a [Node], id: u64) -> Vec<&'a str> {
    let mut path = Vec::new();
    let mut n = Some(0);
    while let Some(m) = n {
        path.push(nodes[m].code);
        if id < nodes[m].id {
            n = nodes[m].left;
        } else {
            n = nodes[m].right;
        }
    }
    path
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let (artifacts, common) = input.split_once("\n\n").unwrap();

    // build tree
    let mut nodes = Vec::new();
    for a in artifacts.lines() {
        let (code, id) = a.split_once(" | ").unwrap();
        let id = id.parse::<u64>().unwrap();

        if nodes.is_empty() {
            nodes.push(Node {
                code,
                id,
                left: None,
                right: None,
                layer: 1,
            });
        } else {
            let new_m = nodes.len();

            let mut layer = 0;
            let mut n = Some(0);
            while let Some(m) = n {
                layer = nodes[m].layer + 1;
                if id < nodes[m].id {
                    n = nodes[m].left;
                    if n.is_none() {
                        nodes[m].left = Some(new_m);
                    }
                } else {
                    n = nodes[m].right;
                    if n.is_none() {
                        nodes[m].right = Some(new_m);
                    }
                }
            }

            nodes.push(Node {
                code,
                id,
                left: None,
                right: None,
                layer,
            });
        }
    }

    // part 1
    let mut sums = HashMap::new();
    layer_sums(0, &nodes, &mut sums);
    let max_sum = sums.values().max().unwrap();
    println!("{}", sums.len() as u64 * max_sum);

    // part 2
    println!("{}", find_path(&nodes, 500000).join("-"));

    // part 3
    let mut all_paths = Vec::new();
    for c in common.lines() {
        let (_, id) = c.split_once(" | ").unwrap();
        let id = id.parse::<u64>().unwrap();
        all_paths.push(find_path(&nodes, id));
    }
    let mut i = 0;
    while all_paths.iter().all(|p| p[i] == all_paths[0][i]) {
        i += 1;
    }
    println!("{}", all_paths[0][i - 1]);
}
