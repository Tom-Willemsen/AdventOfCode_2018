use advent_of_code_2018::{Cli, Parser};
use ahash::{AHashMap, AHashSet};
use std::fs;

fn parse(raw_inp: &str) -> Vec<&str> {
    raw_inp.trim().lines().collect()
}

fn calculate_p1(lines: &[&str]) -> usize {
    let mut frequencies = AHashMap::default();
    let (mut twos, mut threes) = (0, 0);

    for &line in lines {
        line.chars()
            .for_each(|c| *frequencies.entry(c).or_insert(0) += 1);

        if frequencies.values().any(|&v| v == 2) {
            twos += 1;
        }
        if frequencies.values().any(|&v| v == 3) {
            threes += 1;
        }

        frequencies.clear();
    }

    twos * threes
}

fn calculate_p2(lines: &[&str]) -> String {
    let mut seen = AHashSet::default();
    for to_remove in 0..lines[0].len() {
        for line in lines {
            let mut test = line[0..to_remove].to_owned();
            test.push_str(&line[to_remove + 1..]);

            if !seen.insert(test.clone()) {
                return test;
            }
        }
        seen.clear();
    }
    panic!("no p2 solution found");
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let lines = parse(&inp);
    let p1 = calculate_p1(&lines);
    let p2 = calculate_p2(&lines);
    println!("{p1}\n{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA_P1: &str = include_str!("../../inputs/examples/2018_02_p1");
    const EXAMPLE_DATA_P2: &str = include_str!("../../inputs/examples/2018_02_p2");
    const REAL_DATA: &str = include_str!("../../inputs/real/2018_02");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(&EXAMPLE_DATA_P1)), 12);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(&parse(&EXAMPLE_DATA_P2)), "fgij");
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(&REAL_DATA)), 8715);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(
            calculate_p2(&parse(&REAL_DATA)),
            "fvstwblgqkhpuixdrnevmaycd"
        );
    }
}
