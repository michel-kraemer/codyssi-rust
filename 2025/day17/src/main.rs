use std::collections::{HashMap, HashSet};
use std::fs;

/// A position on a staircase
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Position {
    staircase: usize,
    step: usize,
}

impl Position {
    fn from(staircase: usize, step: usize) -> Self {
        Self { staircase, step }
    }
}

/// Find all reachable target positions from the given current position `pos`
/// using exactly `n_steps` without overshooting `final_target`. Consider all
/// `branches`. Store the possible unique target positions in the given
/// `result` set.
fn find_possible_targets(
    pos: Position,
    final_target: Position,
    n_steps: usize,
    branches: &[HashMap<usize, Vec<usize>>],
    steps_per_staircase: &[usize],
    result: &mut HashSet<Position>,
) {
    if n_steps == 0 {
        result.insert(pos);
        return;
    }

    if pos == final_target {
        // don't overshoot
        return;
    }

    // try to take a branch if there is one at the current step
    if let Some(branches_for_current_step) = branches[pos.staircase].get(&pos.step) {
        for &b in branches_for_current_step {
            find_possible_targets(
                Position::from(b, pos.step),
                final_target,
                n_steps - 1, // changing staircases takes one step
                branches,
                steps_per_staircase,
                result,
            );
        }
    }

    // follow this staircase if there are steps left
    if pos.step < steps_per_staircase[pos.staircase] {
        find_possible_targets(
            Position::from(pos.staircase, pos.step + 1),
            final_target,
            n_steps - 1,
            branches,
            steps_per_staircase,
            result,
        );
    }
}

/// Count all possible ways we can take from the given `pos` to the
/// `final_target` using the given `possible_moves`. Consider all branches.
fn count_ways(
    pos: Position,
    final_target: Position,
    possible_moves: &[usize],
    branches: &[HashMap<usize, Vec<usize>>],
    steps_per_staircase: &[usize],
    cache: &mut HashMap<Position, u128>,
) -> u128 {
    if pos == final_target {
        return 1;
    }

    if let Some(c) = cache.get(&pos) {
        return *c;
    }

    let mut result = 0;
    let mut possible_targets = HashSet::new();
    for &m in possible_moves {
        find_possible_targets(
            pos,
            final_target,
            m,
            branches,
            steps_per_staircase,
            &mut possible_targets,
        );
    }

    for p in possible_targets {
        result += count_ways(
            p,
            final_target,
            possible_moves,
            branches,
            steps_per_staircase,
            cache,
        );
    }

    cache.insert(pos, result);

    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let (stairs, possible_moves) = input.trim().split_once("\n\n").unwrap();

    // parse stairs and collect possible branch points
    let stairs = stairs.lines().collect::<Vec<_>>();
    let mut steps_per_staircase = vec![0; stairs.len() + 1];
    let mut branches: Vec<HashMap<usize, Vec<usize>>> = vec![HashMap::new(); stairs.len() + 1];
    for s in stairs {
        let p = s.split_whitespace().collect::<Vec<_>>();
        let id = p[0][1..].parse::<usize>().unwrap();
        let n1 = p[2].parse::<usize>().unwrap();
        let n2 = p[4].parse::<usize>().unwrap();
        if p[7] == "START" {
            steps_per_staircase[1] = n2;
        } else {
            // IMPORTANT: parse staircase IDs to integers so we can sort
            // them correctly
            let a = p[7][1..].parse::<usize>().unwrap();
            let b = p[9][1..].parse::<usize>().unwrap();
            branches[a].entry(n1).or_default().push(id);
            branches[id].entry(n2).or_default().push(b);
            steps_per_staircase.insert(id, n2);
        }
    }

    // parse possible moves
    let (_, possible_moves) = possible_moves.split_once(": ").unwrap();
    let possible_moves = possible_moves
        .split(", ")
        .map(|m| m.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let starting_position = Position::from(1, 0);
    let final_target = Position::from(1, steps_per_staircase[1]);

    // part 1
    println!(
        "{}",
        count_ways(
            starting_position,
            final_target,
            &possible_moves,
            &vec![HashMap::new(); branches.len()],
            &steps_per_staircase,
            &mut HashMap::new()
        )
    );

    // part 2
    let mut cache = HashMap::new();
    println!(
        "{}",
        count_ways(
            starting_position,
            final_target,
            &possible_moves,
            &branches,
            &steps_per_staircase,
            &mut cache
        )
    );

    // part 3
    let mut current_rank = 1;
    let target_rank = 100000000000000000000000000000;
    let mut pos = starting_position;
    while pos != final_target {
        // find all possible targets we could go to from the current position
        let mut possible_targets = HashSet::new();
        for &m in &possible_moves {
            find_possible_targets(
                pos,
                final_target,
                m,
                &branches,
                &steps_per_staircase,
                &mut possible_targets,
            );
        }

        // calculate number of ways from each target
        let mut ranks = Vec::new();
        for p in possible_targets {
            let r = count_ways(
                p,
                final_target,
                &possible_moves,
                &branches,
                &steps_per_staircase,
                &mut cache,
            );
            ranks.push((p, r));
        }

        // sort target positions by number of ways we could take from them to the end
        ranks.sort_unstable();

        // Find out which path we need to take. Skip anything that would give
        // us a rank that's too low. Since the target positions are sorted, the
        // skipped paths will have a lower rank than our target rank. If all
        // target positions have a lower rank, take that path with the highest
        // rank. If one of the target positions has too many possible ways and
        // we would get a rank that's too high, take this path. In summary,
        // this basically works like a n-ary search in the tree of possible
        // paths.
        let mut sum = current_rank;
        let mut i = 0;
        while i < ranks.len() - 1 && sum + ranks[i].1 <= target_rank {
            sum += ranks[i].1;
            i += 1;
        }
        print!("S{}_{}-", pos.staircase, pos.step);

        pos = ranks[i].0;
        current_rank = sum;
    }
    println!("S{}_{}", pos.staircase, pos.step);
}
