use advent_of_code_2018::bitvec_set::BitVecSet2D;
use advent_of_code_2018::{Cli, Parser};
use itertools::Itertools;
use std::collections::VecDeque;
use std::fs;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
struct Vein {
    start_x: usize,
    stop_x: usize,
    start_y: usize,
    stop_y: usize,
}

impl FromStr for Vein {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, tail) = s.split_once(", ").ok_or(())?;

        let lhs = &head[2..];
        let rhs = &tail[2..];
        let lhs_value: usize = lhs.parse().map_err(|_| ())?;
        let rhs_values: (usize, usize) = rhs
            .split("..")
            .filter_map(|x| x.parse().ok())
            .collect_tuple()
            .ok_or(())?;

        let (start_x, stop_x, start_y, stop_y) = match head.bytes().next() {
            Some(b'x') => Ok((lhs_value, lhs_value, rhs_values.0, rhs_values.1)),
            Some(b'y') => Ok((rhs_values.0, rhs_values.1, lhs_value, lhs_value)),
            _ => Err(()),
        }?;

        Ok(Vein {
            start_x,
            stop_x,
            start_y,
            stop_y,
        })
    }
}

impl Vein {
    fn points(&self) -> Vec<(usize, usize)> {
        let mut ans = vec![];
        for y in self.start_y..=self.stop_y {
            for x in self.start_x..=self.stop_x {
                ans.push((y, x));
            }
        }
        ans
    }
}

fn parse(raw_inp: &str) -> BitVecSet2D {
    let points: Vec<_> = raw_inp
        .lines()
        .filter_map(|line| line.parse::<Vein>().ok())
        .flat_map(|v| v.points())
        .collect();

    let max_x = points.iter().map(|(_, x)| x).max().expect("non-empty");
    let max_y = points.iter().map(|(y, _)| y).max().expect("non-empty");

    let mut result = BitVecSet2D::new((max_y + 2, max_x + 2));
    points.iter().for_each(|&pt| {
        result.insert(pt);
    });
    result
}

enum NextState {
    Wall((usize, usize)),
    Drop((usize, usize)),
}

fn next<const DIR: isize>(
    walls: &BitVecSet2D,
    settled_water: &BitVecSet2D,
    w: (usize, usize),
) -> NextState {
    let mut x = w.1;
    loop {
        x = x.checked_add_signed(DIR).expect("adding dir overflowed");
        if walls.contains(&(w.0, x)) {
            return NextState::Wall((w.0, x));
        }
        let down = (w.0 + 1, x);
        if !walls.contains(&down) && !settled_water.contains(&down) {
            return NextState::Drop(down);
        }
    }
}

fn calculate(walls: &BitVecSet2D) -> (usize, usize) {
    let (min_y, max_y) = walls
        .iter()
        .map(|(y, _)| y)
        .minmax()
        .into_option()
        .expect("nonempty");

    let mut wet = BitVecSet2D::new(walls.bounds);
    let mut settled_water = BitVecSet2D::new(walls.bounds);

    let mut sources = VecDeque::default();
    sources.push_back((min_y, 500));

    while let Some(source) = sources.pop_front() {
        let (sy, sx) = source;
        if sy > max_y || settled_water.contains(&source) {
            continue;
        }
        wet.insert(source);
        let down = (sy + 1, sx);
        if !walls.contains(&down) && !settled_water.contains(&down) {
            sources.push_back(down);
            continue;
        }

        let left = next::<-1>(walls, &settled_water, (sy, sx));
        let right = next::<1>(walls, &settled_water, (sy, sx));

        match (left, right) {
            (NextState::Wall(l), NextState::Wall(r)) => {
                for x in l.1 + 1..=r.1 - 1 {
                    wet.insert((sy, x));
                    settled_water.insert((sy, x));
                }
                sources.push_back((sy - 1, sx));
            }
            (NextState::Wall(l), NextState::Drop(r)) => {
                for x in l.1 + 1..=r.1 {
                    wet.insert((sy, x));
                }
                sources.push_back(r);
            }
            (NextState::Drop(l), NextState::Wall(r)) => {
                for x in l.1..=r.1 - 1 {
                    wet.insert((sy, x));
                }
                sources.push_back(l);
            }
            (NextState::Drop(l), NextState::Drop(r)) => {
                for x in l.1..=r.1 {
                    wet.insert((sy, x));
                }
                sources.push_back(l);
                sources.push_back(r);
            }
        }
    }

    let p1 = wet.len();
    let p2 = settled_water.len();
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2018_17");
    const REAL_DATA: &str = include_str!("../../inputs/real/2018_17");

    #[test]
    fn test_example() {
        assert_eq!(calculate(&parse(EXAMPLE_DATA)), (57, 29));
    }

    #[test]
    fn test_real() {
        assert_eq!(calculate(&parse(REAL_DATA)), (27736, 22474));
    }
}
