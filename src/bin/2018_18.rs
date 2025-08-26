use advent_of_code_2018::grid_util::make_byte_grid;
use advent_of_code_2018::{Cli, Parser};
use ahash::AHashMap;
use ndarray::Array2;
use std::fs;

fn parse(raw_inp: &str) -> Array2<u8> {
    make_byte_grid(raw_inp)
}

const NEIGHBOURS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn grid_to_score(data: &Array2<u8>) -> usize {
    data.iter().filter(|&e| e == &b'|').count() * data.iter().filter(|&e| e == &b'#').count()
}

fn calculate<const ENABLE_P2: bool>(data: &Array2<u8>) -> (usize, usize) {
    let mut data = data.clone();
    let mut p1 = 0;
    let mut p2 = 0;

    let mut states = AHashMap::default();

    for s in 1.. {
        let mut next_data = data.clone();
        next_data.indexed_iter_mut().for_each(|((y, x), e)| {
            let mut trees = 0;
            let mut lumberyards = 0;
            for (ny, nx) in NEIGHBOURS.into_iter() {
                match data.get((y.wrapping_add_signed(ny), x.wrapping_add_signed(nx))) {
                    Some(b'|') => trees += 1,
                    Some(b'#') => lumberyards += 1,
                    _ => {}
                }
            }
            if *e == b'.' && trees >= 3 {
                *e = b'|';
            } else if *e == b'|' && lumberyards >= 3 {
                *e = b'#';
            } else if *e == b'#' && (lumberyards == 0 || trees == 0) {
                *e = b'.';
            } else {
                *e = data[(y, x)];
            }
        });
        std::mem::swap(&mut data, &mut next_data);
        std::mem::drop(next_data);

        if s == 10 {
            p1 = grid_to_score(&data);
            if !ENABLE_P2 {
                break;
            }
        }

        if let Some(old_value) = states.insert(data.clone(), s) {
            let cycle_len = s - old_value;
            let p2_iter = (1000000000 - s) % cycle_len;
            let p2_grid = states
                .iter()
                .filter(|&(_, v)| *v == old_value + p2_iter)
                .map(|(k, _)| k)
                .next()
                .unwrap();
            p2 = grid_to_score(p2_grid);
            break;
        }
    }

    (p1, p2)
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let (p1, p2) = calculate::<true>(&data);
    println!("{p1}\n{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2018_18");
    const REAL_DATA: &str = include_str!("../../inputs/real/2018_18");

    #[test]
    fn test_example() {
        assert_eq!(calculate::<false>(&parse(EXAMPLE_DATA)).0, 1147);
    }

    #[test]
    fn test_real() {
        assert_eq!(calculate::<true>(&parse(REAL_DATA)), (519478, 210824));
    }
}
