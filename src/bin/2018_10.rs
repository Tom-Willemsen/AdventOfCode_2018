use advent_of_code_2018::{Cli, Parser};
use itertools::Itertools;
use std::{fs, str::FromStr};

struct Point {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos, vel) = s.split_once("> velocity=<").ok_or(())?;

        let (x, y) = pos
            .strip_prefix("position=<")
            .ok_or(())?
            .split_once(",")
            .map(|(x, y)| (x.trim().parse::<i32>().ok(), y.trim().parse::<i32>().ok()))
            .unzip();

        let (vx, vy) = vel
            .strip_suffix(">")
            .ok_or(())?
            .split_once(",")
            .map(|(vx, vy)| (vx.trim().parse::<i32>().ok(), vy.trim().parse::<i32>().ok()))
            .unzip();

        Ok(Point {
            x: x.flatten().ok_or(())?,
            y: y.flatten().ok_or(())?,
            vx: vx.flatten().ok_or(())?,
            vy: vy.flatten().ok_or(())?,
        })
    }
}

impl Point {
    fn pos_after(&self, time: i32) -> (i32, i32) {
        (self.x + time * self.vx, self.y + time * self.vy)
    }
}

fn parse(raw_inp: &str) -> Vec<Point> {
    raw_inp
        .trim()
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect()
}

fn calculate(data: &[Point]) -> (String, i32) {
    for second in 1.. {
        let (min_y, max_y) = data
            .iter()
            .map(|p| p.pos_after(second).1)
            .minmax()
            .into_option()
            .expect("non-empty");

        if (max_y - min_y) <= 10 {
            let mut ans = vec![];

            let (min_x, max_x) = data
                .iter()
                .map(|p| p.pos_after(second).0)
                .minmax()
                .into_option()
                .expect("non-empty");

            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    let yes = data
                        .iter()
                        .map(|p| p.pos_after(second))
                        .any(|p| p == (x, y));
                    if yes {
                        ans.push(b'#');
                    } else {
                        ans.push(b' ');
                    }
                }
                ans.push(b'\n');
            }
            return (String::from_utf8(ans).expect("encoding"), second);
        }
    }
    panic!("no answer");
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

    const REAL_DATA: &str = include_str!("../../inputs/real/2018_10");

    #[test]
    fn test_p1_real() {
        assert_eq!(
            calculate(&parse(REAL_DATA)).0.trim(),
            "
 ####   #####      ###  #       #       #       #       #    #
#    #  #    #      #   #       #       #       #       #    #
#       #    #      #   #       #       #       #       #    #
#       #    #      #   #       #       #       #       #    #
#       #####       #   #       #       #       #       ######
#  ###  #           #   #       #       #       #       #    #
#    #  #           #   #       #       #       #       #    #
#    #  #       #   #   #       #       #       #       #    #
#   ##  #       #   #   #       #       #       #       #    #
 ### #  #        ###    ######  ######  ######  ######  #    #
"
            .trim()
        );
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate(&parse(REAL_DATA)).1, 10515);
    }
}
