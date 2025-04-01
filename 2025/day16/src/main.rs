use ordered_float::OrderedFloat;
use rustc_hash::FxHashMap;
use std::fs;

type Coord = (OrderedFloat<f32>, OrderedFloat<f32>, OrderedFloat<f32>);

/// Rotate the cube
fn rotate(cells: FxHashMap<Coord, i64>, direction: char) -> FxHashMap<Coord, i64> {
    let mut new_cells = cells.clone();
    for ((x, y, z), v) in cells {
        match direction {
            'R' => new_cells.insert((z, y, -x), v),
            'L' => new_cells.insert((-z, y, x), v),
            'U' => new_cells.insert((x, z, -y), v),
            'D' => new_cells.insert((x, -z, y), v),
            _ => panic!(),
        };
    }
    new_cells
}

/// Calculate "dominant sum" (the maximum sum of all row and column sums) for
/// the current face and update the magic cell
fn update_dominant_sum(
    size: i64,
    min: f32,
    cells: &mut FxHashMap<Coord, i64>,
    magic: OrderedFloat<f32>,
) {
    let mut max_sum = 0;
    for y in 0..size {
        let mut row_sum = 0;
        for x in 0..size {
            row_sum += (cells
                .get(&(
                    (x as f32 + min).into(),
                    (y as f32 + min).into(),
                    (min - 1.0).into(),
                ))
                .unwrap()
                - 1)
                % 100
                + 1;
        }
        max_sum = max_sum.max(row_sum);
    }

    for x in 0..size {
        let mut col_sum = 0;
        for y in 0..size {
            col_sum += (cells
                .get(&(
                    (x as f32 + min).into(),
                    (y as f32 + min).into(),
                    (min - 1.0).into(),
                ))
                .unwrap()
                - 1)
                % 100
                + 1;
        }
        max_sum = max_sum.max(col_sum);
    }

    cells.insert((0.0.into(), 0.0.into(), -magic), max_sum);
}

fn main() {
    for part in [1, 2, 3] {
        let input = fs::read_to_string("input.txt").expect("Could not read file");
        let (instructions, turns) = input.split_once("\n\n").unwrap();
        let instructions = instructions.lines().collect::<Vec<_>>();
        let turns = turns.lines().collect::<Vec<_>>();

        let size = 80i64;

        // center cube around origin
        let min = if size % 2 == 0 {
            -size as f32 / 2.0 + 0.5
        } else {
            -(size - 1) as f32 / 2.0
        };
        let max = if size % 2 == 0 {
            size as f32 / 2.0 - 0.5
        } else {
            (size - 1) as f32 / 2.0
        };

        // add cells for each side of the cube (explode cube by one unit to
        // avoid collisions at borders)
        let mut cells = FxHashMap::default();
        for y in 0..size {
            let y = (y as f32 + min).into();
            for x in 0..size {
                let x = (x as f32 + min).into();
                cells.insert((x, y, (min - 1.0).into()), 1); // front
                cells.insert((x, y, (max + 1.0).into()), 1); // back
            }
        }
        for z in 0..size {
            let z = (z as f32 + min).into();
            for x in 0..size {
                let x = (x as f32 + min).into();
                cells.insert((x, (max + 1.0).into(), z), 1); // top
                cells.insert((x, (min - 1.0).into(), z), 1); // bottom
            }
        }
        for z in 0..size {
            let z = (z as f32 + min).into();
            for y in 0..size {
                let y = (y as f32 + min).into();
                cells.insert(((min - 1.0).into(), y, z), 1); // left
                cells.insert(((max + 1.0).into(), y, z), 1); // right
            }
        }

        // add six magic cells (size * 100 away from each side) to store results
        let magic = OrderedFloat::from(size as f32 * 100.0);
        let default_val = if part == 1 { 0 } else { size };
        cells.insert((0.0.into(), 0.0.into(), -magic), default_val);
        cells.insert((0.0.into(), 0.0.into(), magic), default_val);
        cells.insert((-magic, 0.0.into(), 0.0.into()), default_val);
        cells.insert((magic, 0.0.into(), 0.0.into()), default_val);
        cells.insert((0.0.into(), -magic, 0.0.into()), default_val);
        cells.insert((0.0.into(), magic, 0.0.into()), default_val);

        // process instructions
        let mut i = 0;
        let mut t = 0;
        while i < instructions.len() {
            let p = instructions[i].split_whitespace().collect::<Vec<_>>();
            if p[0] == "FACE" {
                // add value to all cell on the current face
                let v = p[3].parse::<i64>().unwrap();
                if part == 1 {
                    *cells.entry((0.0.into(), 0.0.into(), -magic)).or_default() += size * size * v;
                } else {
                    for y in 0..size {
                        for x in 0..size {
                            *cells
                                .entry((
                                    (x as f32 + min).into(),
                                    (y as f32 + min).into(),
                                    (min - 1.0).into(),
                                ))
                                .or_default() += v;
                        }
                    }
                    update_dominant_sum(size, min, &mut cells, magic);
                }
            } else if p[0] == "COL" {
                // add value to a given column on the current face
                let col = p[1].parse::<i32>().unwrap();
                let v = p[4].parse::<i64>().unwrap();
                if part == 1 {
                    *cells.entry((0.0.into(), 0.0.into(), -magic)).or_default() += size * v;
                } else {
                    // apply to all faces in the same direction (if part == 3)
                    for _ in 0..if part == 3 { 4 } else { 1 } {
                        for y in 0..size {
                            *cells
                                .entry((
                                    (min + col as f32 - 1.0).into(),
                                    (y as f32 + min).into(),
                                    (min - 1.0).into(),
                                ))
                                .or_default() += v;
                        }
                        update_dominant_sum(size, min, &mut cells, magic);
                        if part == 3 {
                            cells = rotate(cells, 'D');
                        }
                    }
                }
            } else if p[0] == "ROW" {
                // add value to a given row on the current face
                let row = p[1].parse::<i32>().unwrap();
                let v = p[4].parse::<i64>().unwrap();
                if part == 1 {
                    *cells.entry((0.0.into(), 0.0.into(), -magic)).or_default() += size * v;
                } else {
                    // apply to all faces in the same direction (if part == 3)
                    for _ in 0..if part == 3 { 4 } else { 1 } {
                        for x in 0..size {
                            *cells
                                .entry((
                                    (x as f32 + min).into(),
                                    (max - row as f32 + 1.0).into(),
                                    (min - 1.0).into(),
                                ))
                                .or_default() += v;
                        }
                        update_dominant_sum(size, min, &mut cells, magic);
                        if part == 3 {
                            cells = rotate(cells, 'R');
                        }
                    }
                }
            }

            if t < turns[0].len() {
                cells = rotate(cells, turns[0].chars().nth(t).unwrap());
            }

            i += 1;
            t += 1;
        }

        // get values of magic cells
        let mut results = Vec::new();
        for (x, y, z) in [
            (0.0.into(), 0.0.into(), -magic),
            (0.0.into(), 0.0.into(), magic),
            (-magic, 0.0.into(), 0.0.into()),
            (magic, 0.0.into(), 0.0.into()),
            (0.0.into(), -magic, 0.0.into()),
            (0.0.into(), magic, 0.0.into()),
        ] {
            results.push(*cells.get(&(x, y, z)).unwrap() as u128);
        }

        // print results
        if part == 1 {
            results.sort_unstable();
            println!("{}", results[results.len() - 2..].iter().product::<u128>());
        } else {
            println!("{}", results.iter().product::<u128>());
        }
    }
}
