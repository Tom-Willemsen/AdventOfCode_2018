use advent_of_code_2018::{Cli, Parser};
use mimalloc::MiMalloc;
use std::{collections::VecDeque, fs};

#[global_allocator]
static GLOBAL_ALLOC: MiMalloc = MiMalloc;

fn parse(raw_inp: &str) -> (usize, usize) {
    raw_inp
        .trim()
        .split_once(" players; last marble is worth ")
        .and_then(|(a, b)| Some((a.parse().ok()?, b.split_once(" ")?.0.parse().ok()?)))
        .expect("bad format")
}

fn calculate<const M: usize>(data: &(usize, usize)) -> usize {
    // Always with "current marble" at end of deque
    let mut marbles = VecDeque::with_capacity(data.1 * M);
    marbles.push_back(0);
    let n_players = data.0;
    let n_marbles = data.1 * M;
    let mut player_scores = vec![0; n_players];

    for marble in 1..=n_marbles {
        if marble % 23 == 0 {
            marbles.rotate_right(6);

            let a = marbles.pop_back().expect("non-empty");
            let b = marbles.pop_back().expect("non-empty");
            marbles.push_back(a);
            player_scores[marble % n_players] += marble + b;
        } else {
            let a = marbles.pop_front().expect("non-empty");
            marbles.push_back(a);
            marbles.push_back(marble);
        }
    }

    player_scores
        .into_iter()
        .max()
        .expect("more than 0 players")
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let p1 = calculate::<1>(&data);
    let p2 = calculate::<100>(&data);
    println!("{p1}\n{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2018_09");
    const REAL_DATA: &str = include_str!("../../inputs/real/2018_09");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate::<1>(&parse(EXAMPLE_DATA)), 32);
        assert_eq!(calculate::<1>(&(10, 1618)), 8317);
        assert_eq!(calculate::<1>(&(13, 7999)), 146373);
        assert_eq!(calculate::<1>(&(17, 1104)), 2764);
        assert_eq!(calculate::<1>(&(21, 6111)), 54718);
        assert_eq!(calculate::<1>(&(30, 5807)), 37305);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate::<1>(&parse(REAL_DATA)), 408679);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate::<100>(&parse(REAL_DATA)), 3443939356);
    }
}
