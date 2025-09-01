use advent_of_code_2018::{Cli, Parser};
use itertools::Itertools;
use ndarray::{Array2, Array3};
use std::{collections::BinaryHeap, fs};

struct Data {
    depth: usize,
    tx: usize,
    ty: usize,
}

fn parse(raw_inp: &str) -> Data {
    let (depth_line, target_line) = raw_inp.trim().split_once("\n").expect("invalid format");

    let depth = depth_line
        .split_once(": ")
        .expect("invalid format")
        .1
        .parse()
        .expect("bad parse");

    let (tx, ty) = target_line
        .split_once(": ")
        .expect("invalid format")
        .1
        .split(",")
        .filter_map(|s| s.parse().ok())
        .collect_tuple()
        .expect("invalid format");

    Data { depth, tx, ty }
}

fn geologic_index(x: usize, y: usize, geologic_indices: &mut Array2<usize>, data: &Data) -> usize {
    let mut g = if (x == 0 && y == 0) || (x == data.tx && y == data.ty) {
        0
    } else if y == 0 {
        x * 16807
    } else if x == 0 {
        y * 48271
    } else {
        let up = geologic_indices
            .get((y - 1, x))
            .copied()
            .unwrap_or_else(|| geologic_index(x, y - 1, geologic_indices, data));
        let left = geologic_indices
            .get((y, x - 1))
            .copied()
            .unwrap_or_else(|| geologic_index(x - 1, y, geologic_indices, data));
        up * left
    };
    g += data.depth;
    g %= 20183;
    geologic_indices[(y, x)] = g;
    g
}

fn erosion_level(x: usize, y: usize, geologic_indices: &mut Array2<usize>, data: &Data) -> usize {
    geologic_index(x, y, geologic_indices, data) % 3
}

const TORCH: usize = 0;
const CLIMB: usize = 1;
const NEITHER: usize = 2;

#[inline]
fn valid_gear(erosion_level: usize) -> [usize; 2] {
    match erosion_level {
        0 => [CLIMB, TORCH],
        1 => [CLIMB, NEITHER],
        2 => [TORCH, NEITHER],
        _ => panic!("bad erosion level"),
    }
}

#[inline]
fn change_gear(erosion_level: usize, current_gear: usize) -> usize {
    valid_gear(erosion_level)
        .into_iter()
        .find(|&g| g != current_gear)
        .expect("no valid gear to change to")
}

const DIRS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const PADDING: usize = 20;

fn calculate(data: &Data) -> (usize, i64) {
    let mut p1 = 0;
    let mut p2: i64 = -1;
    let mut geologic_indices = Array2::from_elem((data.ty + PADDING, data.tx + PADDING), 0);
    let mut costs = Array3::from_elem((data.ty + PADDING, data.tx + PADDING, 3), i64::MIN);

    for y in 0..data.ty + PADDING {
        for x in 0..data.tx + PADDING {
            let e = erosion_level(x, y, &mut geologic_indices, data);
            if y <= data.ty && x <= data.tx {
                p1 += e;
            }
        }
    }

    let mut b_heap = BinaryHeap::default();
    b_heap.push((0, (0, 0, TORCH)));

    while let Some((cost, (x, y, gear))) = b_heap.pop() {
        if cost <= costs[(y, x, gear)] {
            continue;
        }
        costs[(y, x, gear)] = cost;
        if x == data.tx && y == data.ty && gear == TORCH {
            p2 = -cost;
            break;
        }
        let e = erosion_level(x, y, &mut geologic_indices, data);
        let cg = change_gear(e, gear);
        if cost - 7 > costs[(y, x, cg)] {
            b_heap.push((cost - 7, (x, y, cg)));
        }

        for dir in DIRS {
            if let Some(nx) = x.checked_add_signed(dir.0)
                && let Some(ny) = y.checked_add_signed(dir.1)
            {
                if ny >= data.ty + PADDING || nx >= data.tx + PADDING {
                    continue;
                }
                let ne = erosion_level(nx, ny, &mut geologic_indices, data);
                if valid_gear(ne).contains(&gear) && cost - 1 > costs[(ny, nx, gear)] {
                    b_heap.push((cost - 1, (nx, ny, gear)));
                }
            }
        }
    }

    (p1, p2)
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let (p1, p2) = calculate(&data);
    println!("{p1}\n{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2018_22");
    const REAL_DATA: &str = include_str!("../../inputs/real/2018_22");

    #[test]
    fn test_example() {
        assert_eq!(calculate(&parse(EXAMPLE_DATA)), (114, 45));
    }

    #[test]
    fn test_real() {
        assert_eq!(calculate(&parse(REAL_DATA)), (7915, 980));
    }
}
