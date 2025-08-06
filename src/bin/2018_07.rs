use advent_of_code_2018::{Cli, Parser};
use ahash::AHashMap;
use itertools::Itertools;
use std::fs;

fn parse(raw_inp: &str) -> AHashMap<u8, Vec<u8>> {
    let mut result = AHashMap::default();
    raw_inp
        .trim()
        .lines()
        .filter_map(|line| line.split_once(" must be finished before step "))
        .filter_map(|(a, b)| Some((a.bytes().last()?, b.bytes().next()?)))
        .for_each(|(a, b)| {
            result.entry(b).or_insert(vec![]).push(a);
            result.entry(a).or_insert(vec![]);
        });

    result
}

fn calculate_p1(data: &AHashMap<u8, Vec<u8>>) -> String {
    let mut completed: Vec<u8> = vec![];

    loop {
        let next_complete = data
            .iter()
            .filter(|(k, _)| !completed.contains(k))
            .filter(|(_, v)| v.iter().all(|r| completed.contains(r)))
            .map(|(k, _)| k)
            .sorted()
            .next();

        if let Some(next) = next_complete {
            completed.push(*next);
        } else {
            break;
        }
    }

    String::from_utf8(completed).expect("encoding")
}

fn calculate_p2<const WORKERS: usize, const OFFSET: u32>(data: &AHashMap<u8, Vec<u8>>) -> u32 {
    let mut completed: AHashMap<u8, u32> = AHashMap::default();
    let mut workers = vec![0; WORKERS];

    for second in 0.. {
        for worker in workers.iter_mut() {
            if *worker > second {
                // No free worker
                continue;
            }

            let next_complete = data
                .iter()
                .filter(|(k, _)| !completed.keys().contains(k))
                .filter(|(_, v)| {
                    v.iter()
                        .all(|r| completed.get(r).unwrap_or(&u32::MAX) <= &second)
                })
                .map(|(k, _)| k)
                .sorted()
                .next();

            if let Some(&next) = next_complete {
                let complete_time = second + OFFSET + (next - b'A') as u32 + 1;
                completed.insert(next, complete_time);
                *worker = complete_time;
            }
        }

        if completed.len() == data.len() {
            break;
        }
    }

    *completed.values().max().expect("non-empty")
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let p1 = calculate_p1(&data);
    let p2 = calculate_p2::<5, 60>(&data);
    println!("{p1}\n{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2018_07");
    const REAL_DATA: &str = include_str!("../../inputs/real/2018_07");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(EXAMPLE_DATA)), "CABDFE");
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2::<2, 0>(&parse(EXAMPLE_DATA)), 15);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(
            calculate_p1(&parse(REAL_DATA)),
            "DFOQPTELAYRVUMXHKWSGZBCJIN"
        );
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2::<5, 60>(&parse(REAL_DATA)), 1036);
    }
}
