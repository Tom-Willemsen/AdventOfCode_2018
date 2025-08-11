use advent_of_code_2018::{Cli, Parser};
use ahash::AHashSet;
use itertools::Itertools;
use std::fs;

fn parse(raw_inp: &str) -> (AHashSet<i64>, AHashSet<[bool; 5]>) {
    let (initial_state, rules) = raw_inp.split_once("\n\n").expect("bad format");

    let initial_state = initial_state
        .trim()
        .strip_prefix("initial state: ")
        .expect("no prefix")
        .bytes()
        .enumerate()
        .filter(|(_, itm)| *itm == b'#')
        .map(|(pos, _)| pos as i64)
        .collect::<AHashSet<_>>();

    let rules = rules
        .trim()
        .lines()
        .filter_map(|line| line.split_once(" => "))
        .filter(|(_, result)| result.bytes().next() == Some(b'#'))
        .filter_map(|(state, _)| {
            state
                .bytes()
                .map(|b| b == b'#')
                .collect::<Vec<_>>()
                .try_into()
                .ok()
        })
        .collect::<AHashSet<_>>();

    (initial_state, rules)
}

fn calculate<const GENERATIONS: i64>(data: &(AHashSet<i64>, AHashSet<[bool; 5]>)) -> i64 {
    let mut next_generation = AHashSet::default();
    let mut this_generation = data.0.clone();

    for g in 0..GENERATIONS {
        let (min, max) = this_generation
            .iter()
            .minmax()
            .into_option()
            .expect("should have some plants");

        next_generation.clear();

        for p in *min - 2..=*max + 2 {
            let (a, b, c, d, e) = (
                this_generation.contains(&(p - 2)),
                this_generation.contains(&(p - 1)),
                this_generation.contains(&p),
                this_generation.contains(&(p + 1)),
                this_generation.contains(&(p + 2)),
            );

            if data.1.contains(&[a, b, c, d, e]) {
                next_generation.insert(p);
            }
        }

        if this_generation.len() == next_generation.len()
            && next_generation
                .iter()
                .all(|n| this_generation.contains(&(n - 1)))
        {
            return this_generation.iter().sum::<i64>()
                + (GENERATIONS - g) * this_generation.len() as i64;
        }

        std::mem::swap(&mut next_generation, &mut this_generation);
    }

    this_generation.iter().sum()
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let p1 = calculate::<20>(&data);
    let p2 = calculate::<50000000000>(&data);
    println!("{p1}\n{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2018_12");
    const REAL_DATA: &str = include_str!("../../inputs/real/2018_12");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate::<20>(&parse(EXAMPLE_DATA)), 325);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate::<20>(&parse(REAL_DATA)), 3051);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate::<50000000000>(&parse(REAL_DATA)), 1300000000669);
    }
}
