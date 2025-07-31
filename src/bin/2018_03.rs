use advent_of_code_2018::{Cli, Parser};
use ndarray::Array2;
use std::{fs, str::FromStr};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Claim {
    id: usize,
    left: usize,
    top: usize,
    right: usize,
    bottom: usize,
}

impl FromStr for Claim {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, tail) = s.split_once(" @ ").ok_or(())?;

        let id = head[1..].parse::<usize>().map_err(|_| ())?;
        let (pos, size) = tail.split_once(": ").ok_or(())?;
        let (left, top) = pos.split_once(",").ok_or(())?;
        let (width, height) = size.split_once("x").ok_or(())?;

        let left = left.parse().map_err(|_| ())?;
        let top = top.parse().map_err(|_| ())?;
        let right = left + width.parse::<usize>().map_err(|_| ())?;
        let bottom = top + height.parse::<usize>().map_err(|_| ())?;

        Ok(Claim {
            id,
            left,
            top,
            right,
            bottom,
        })
    }
}

fn overlaps(start1: usize, stop1: usize, start2: usize, stop2: usize) -> bool {
    (start2..stop2).contains(&start1)
        || (start2..stop2).contains(&stop1)
        || (start1..stop1).contains(&start2)
        || (start1..stop1).contains(&stop2)
}

impl Claim {
    fn conflicts_with(&self, other: &Claim) -> bool {
        overlaps(self.left, self.right - 1, other.left, other.right - 1)
            && overlaps(self.top, self.bottom - 1, other.top, other.bottom - 1)
    }
}

fn parse(raw_inp: &str) -> Vec<Claim> {
    raw_inp
        .trim()
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect()
}

fn calculate_p1(claims: &[Claim]) -> usize {
    let mut map = Array2::from_elem((1000, 1000), 0_u8);

    claims.iter().for_each(|c| {
        for y in c.top..c.bottom {
            for x in c.left..c.right {
                map[(y, x)] += 1;
            }
        }
    });

    map.iter().filter(|&&c| c >= 2).count()
}

fn calculate_p2(claims: &[Claim]) -> usize {
    claims
        .iter()
        .rev()
        .find(|claim| {
            claims
                .iter()
                .filter(|c| c.id != claim.id)
                .all(|c| !claim.conflicts_with(c))
        })
        .map(|claim| claim.id)
        .expect("no solution")
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let claims = parse(&inp);
    let p1 = calculate_p1(&claims);
    let p2 = calculate_p2(&claims);
    println!("{p1}\n{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2018_03");
    const REAL_DATA: &str = include_str!("../../inputs/real/2018_03");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(&EXAMPLE_DATA)), 4);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(&parse(&EXAMPLE_DATA)), 3);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(&REAL_DATA)), 117505);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(&REAL_DATA)), 1254);
    }
}
