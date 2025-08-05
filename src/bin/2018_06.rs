use advent_of_code_2018::{Cli, Parser};
use itertools::Itertools;
use rayon::prelude::*;
use std::fs;

fn parse(raw_inp: &str) -> Vec<(i32, i32)> {
    raw_inp
        .trim()
        .lines()
        .filter_map(|line| line.split_once(", "))
        .map(|(a, b)| (a.parse().expect("invalid x"), b.parse().expect("invalid y")))
        .collect()
}

fn closest(coord: (i32, i32), data: &[(i32, i32)]) -> Option<&(i32, i32)> {
    let minset = data
        .iter()
        .min_set_by_key(|pt| (coord.0 - pt.0).abs() + (coord.1 - pt.1).abs());

    if minset.len() == 1 {
        minset.first().copied()
    } else {
        None
    }
}

fn is_finite(coord: &(i32, i32), data: &[(i32, i32)]) -> bool {
    let mut other_data = data.to_vec();
    other_data.retain(|c| c != coord);

    other_data
        .iter()
        .any(|other| other.0 >= coord.0 + (coord.1 - other.1).abs())
        && other_data
            .iter()
            .any(|other| other.0 <= coord.0 - (coord.1 - other.1).abs())
        && other_data
            .iter()
            .any(|other| other.1 >= coord.1 + (coord.0 - other.0).abs())
        && other_data
            .iter()
            .any(|other| other.1 <= coord.1 - (coord.0 - other.0).abs())
}

fn region_size(coord: &(i32, i32), data: &[(i32, i32)]) -> i32 {
    // Maximum distance away in each direction we can find a point that's closest to us
    let min_xd = (1..)
        .find_or_first(|&d| closest((coord.0 - d, coord.1), data) != Some(coord))
        .expect("infinite")
        - 1;
    let max_xd = (1..)
        .find_or_first(|&d| closest((coord.0 + d, coord.1), data) != Some(coord))
        .expect("infinite")
        - 1;
    let min_yd = (1..)
        .find_or_first(|&d| closest((coord.0, coord.1 - d), data) != Some(coord))
        .expect("infinite")
        - 1;
    let max_yd = (1..)
        .find_or_first(|&d| closest((coord.0, coord.1 + d), data) != Some(coord))
        .expect("infinite")
        - 1;

    let mut ans = 0;
    for y in coord.1 - min_yd..=coord.1 + max_yd {
        for x in coord.0 - min_xd..=coord.0 + max_xd {
            if closest((x, y), data) == Some(coord) {
                ans += 1;
            }
        }
    }
    ans
}

fn calculate_p1(data: &[(i32, i32)]) -> i32 {
    data.par_iter()
        .filter(|&coord| is_finite(coord, data))
        .map(|coord| region_size(coord, data))
        .max()
        .expect("non-empty data")
}

fn calculate_p2<const CUTOFF: i32>(data: &[(i32, i32)]) -> i32 {
    let (min_x, max_x) = data
        .iter()
        .map(|c| c.0)
        .minmax()
        .into_option()
        .expect("non-empty");
    let (min_y, max_y) = data
        .iter()
        .map(|c| c.0)
        .minmax()
        .into_option()
        .expect("non-empty");

    let mut ans = 0;

    for y in max_y - CUTOFF..=min_y + CUTOFF {
        let mut x = max_x - CUTOFF;
        loop {
            let dsum = data
                .iter()
                .map(|c| (c.0 - x).abs() + (c.1 - y).abs())
                .sum::<i32>();

            let dsum_diff =
                num::Integer::div_floor(&(dsum - CUTOFF).abs(), &(data.len() as i32)).max(1);

            if dsum < CUTOFF {
                ans += dsum_diff;
            }
            x += dsum_diff;
            if x > min_x + CUTOFF {
                break;
            }
        }
    }
    ans
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let (p1, p2) = rayon::join(|| calculate_p1(&data), || calculate_p2::<10000>(&data));
    println!("{p1}\n{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2018_06");
    const REAL_DATA: &str = include_str!("../../inputs/real/2018_06");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(EXAMPLE_DATA)), 17);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2::<32>(&parse(EXAMPLE_DATA)), 16);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(REAL_DATA)), 3890);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2::<10000>(&parse(REAL_DATA)), 40284);
    }
}
