use advent_of_code_2018::{Cli, Parser};
use ahash::AHashSet;
use std::fs;

fn parse(raw_inp: &str) -> Vec<i32> {
    raw_inp.trim().lines().map(|s| s.parse().unwrap()).collect()
}

fn calculate_p1(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

fn calculate_p2(nums: &[i32]) -> i32 {
    let mut seen = AHashSet::with_capacity(150_000);
    seen.insert(0);
    let mut curr: i32 = 0;
    loop {
        for n in nums {
            curr += n;
            if !seen.insert(curr) {
                return curr;
            }
        }
    }
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let nums: Vec<i32> = parse(&inp);
    let p1 = calculate_p1(&nums);
    let p2 = calculate_p2(&nums);
    println!("{p1}\n{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_DATA: &str = include_str!("../../inputs/real/2018_01");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&[1, 1, 1]), 3);
        assert_eq!(calculate_p1(&[1, 1, -2]), 0);
        assert_eq!(calculate_p1(&[-1, -2, -3]), -6);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(&[1, -1]), 0);
        assert_eq!(calculate_p2(&[3, 3, 4, -2, -4]), 10);
        assert_eq!(calculate_p2(&[-6, 3, 8, 5, -6]), 5);
        assert_eq!(calculate_p2(&[7, 7, -2, -7, -4]), 14);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(&REAL_DATA)), 576);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(&REAL_DATA)), 77674);
    }
}
