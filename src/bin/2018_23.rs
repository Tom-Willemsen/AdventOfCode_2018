use advent_of_code_2018::{Cli, Parser};
use itertools::Itertools;
use std::{collections::BinaryHeap, fs, str::FromStr};

#[derive(PartialEq, Eq)]
struct Nanobot {
    x: i64,
    y: i64,
    z: i64,
    r: u64,
}

impl Nanobot {
    fn in_range_coord(&self, (x, y, z): (i64, i64, i64)) -> bool {
        let diff = self.x.abs_diff(x) + self.y.abs_diff(y) + self.z.abs_diff(z);
        diff <= self.r
    }

    fn in_range(&self, other: &Nanobot) -> bool {
        self.in_range_coord((other.x, other.y, other.z))
    }
}

impl FromStr for Nanobot {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, tail) = s.split_once(">, ").ok_or(())?;

        let (x, y, z) = head[5..]
            .split(",")
            .filter_map(|x| x.parse().ok())
            .collect_tuple()
            .ok_or(())?;

        let r = tail[2..].parse().map_err(|_| ())?;

        Ok(Nanobot { x, y, z, r })
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Voxel {
    start_x: i64,
    start_y: i64,
    start_z: i64,
    end_x: i64,
    end_y: i64,
    end_z: i64,
}

impl PartialOrd for Voxel {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Voxel {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.manhattan().cmp(&self.manhattan())
    }
}

impl Voxel {
    fn manhattan(&self) -> i64 {
        self.start_x.abs() + self.start_y.abs() + self.start_z.abs()
    }

    fn count_maybe_in_range(&self, bots: &[Nanobot]) -> usize {
        bots.iter().filter(|b| self.maybe_in_range(b)).count()
    }

    fn maybe_in_range(&self, bot: &Nanobot) -> bool {
        let xd = bot.x.abs_diff(self.start_x);
        let yd = bot.y.abs_diff(self.start_y);
        let zd = bot.z.abs_diff(self.start_z);

        let xv = self.start_x.abs_diff(self.end_x);
        let yv = self.start_y.abs_diff(self.end_y);
        let zv = self.start_z.abs_diff(self.end_z);

        xd + yd + zd <= bot.r + xv + yv + zv
    }

    fn volume(&self) -> u64 {
        (self.end_x.abs_diff(self.start_x) + 1)
            .saturating_mul(self.end_y.abs_diff(self.start_y) + 1)
            .saturating_mul(self.end_z.abs_diff(self.start_z) + 1)
    }

    fn voxelate(&self) -> [Voxel; 8] {
        let xm = (self.start_x + self.end_x) / 2;
        let ym = (self.start_y + self.end_y) / 2;
        let zm = (self.start_z + self.end_z) / 2;
        [
            Voxel {
                start_x: self.start_x,
                end_x: xm,
                start_y: self.start_y,
                end_y: ym,
                start_z: self.start_z,
                end_z: zm,
            },
            Voxel {
                start_x: self.start_x,
                end_x: xm,
                start_y: self.start_y,
                end_y: ym,
                start_z: zm + 1,
                end_z: self.end_z,
            },
            Voxel {
                start_x: self.start_x,
                end_x: xm,
                start_y: ym + 1,
                end_y: self.end_y,
                start_z: self.start_z,
                end_z: zm,
            },
            Voxel {
                start_x: self.start_x,
                end_x: xm,
                start_y: ym + 1,
                end_y: self.end_y,
                start_z: zm + 1,
                end_z: self.end_z,
            },
            Voxel {
                start_x: xm + 1,
                end_x: self.end_x,
                start_y: self.start_y,
                end_y: ym,
                start_z: self.start_z,
                end_z: zm,
            },
            Voxel {
                start_x: xm + 1,
                end_x: self.end_x,
                start_y: self.start_y,
                end_y: ym,
                start_z: zm + 1,
                end_z: self.end_z,
            },
            Voxel {
                start_x: xm + 1,
                end_x: self.end_x,
                start_y: ym + 1,
                end_y: self.end_y,
                start_z: self.start_z,
                end_z: zm,
            },
            Voxel {
                start_x: xm + 1,
                end_x: self.end_x,
                start_y: ym + 1,
                end_y: self.end_y,
                start_z: zm + 1,
                end_z: self.end_z,
            },
        ]
    }
}

fn parse(raw_inp: &str) -> Vec<Nanobot> {
    raw_inp
        .trim()
        .lines()
        .filter_map(|x| x.parse().ok())
        .collect()
}

fn calculate_p1(data: &[Nanobot]) -> usize {
    let biggest_range_bot = data.iter().max_by_key(|b| b.r).expect("no best bot?");

    data.iter()
        .filter(|&other| biggest_range_bot.in_range(other))
        .count()
}

fn calculate_p2(data: &[Nanobot]) -> i64 {
    let (min_x, max_x) = data
        .iter()
        .map(|b| b.x)
        .minmax()
        .into_option()
        .expect("nonempty");
    let (min_y, max_y) = data
        .iter()
        .map(|b| b.y)
        .minmax()
        .into_option()
        .expect("nonempty");
    let (min_z, max_z) = data
        .iter()
        .map(|b| b.z)
        .minmax()
        .into_option()
        .expect("nonempty");

    let mut q = BinaryHeap::default();
    q.push((
        data.len(),
        Voxel {
            start_x: min_x,
            start_y: min_y,
            start_z: min_z,
            end_x: max_x,
            end_y: max_y,
            end_z: max_z,
        },
    ));

    while let Some((_, voxel)) = q.pop() {
        match voxel.volume() {
            1 => {
                return voxel.manhattan();
            }
            _ => {
                voxel
                    .voxelate()
                    .iter()
                    .for_each(|v| q.push((v.count_maybe_in_range(data), *v)));
            }
        }
    }

    panic!("no p2 answer")
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let p1 = calculate_p1(&data);
    let p2 = calculate_p2(&data);
    println!("{p1}\n{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2018_23");
    const EXAMPLE_DATA_2: &str = include_str!("../../inputs/examples/2018_23_2");
    const REAL_DATA: &str = include_str!("../../inputs/real/2018_23");

    #[test]
    fn test_example_p1() {
        assert_eq!(calculate_p1(&parse(EXAMPLE_DATA)), 7);
    }

    #[test]
    fn test_real_p1() {
        assert_eq!(calculate_p1(&parse(REAL_DATA)), 640);
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(calculate_p2(&parse(EXAMPLE_DATA_2)), 36);
    }

    #[test]
    fn test_real_p2() {
        assert_eq!(calculate_p2(&parse(REAL_DATA)), 113066145)
    }
}
