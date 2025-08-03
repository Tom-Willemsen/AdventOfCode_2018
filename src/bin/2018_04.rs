use advent_of_code_2018::{Cli, Parser};
use ahash::AHashMap;
use std::fs;

fn parse(raw_inp: &str) -> AHashMap<i32, AHashMap<i32, i32>> {
    let mut lines: Vec<&str> = raw_inp.trim().lines().collect();
    lines.sort_unstable();

    let mut guard_id = 0;
    let mut asleep_since = 0;
    let mut mins_asleep = AHashMap::default();

    for line in lines {
        if let Some((_, tail)) = line.split_once(" Guard #") {
            guard_id = tail
                .split_once(" ")
                .expect("invalid format")
                .0
                .parse::<i32>()
                .expect("invalid guard id");
        } else if line.contains("falls asleep") {
            asleep_since = line.split_once(":").expect("invalid format").1[..2]
                .parse()
                .expect("invalid minute");
        } else if line.contains("wakes up") {
            let wakeup_time: i32 = line.split_once(":").expect("invalid format").1[..2]
                .parse()
                .expect("invalid minute");

            let minutes_slept = mins_asleep.entry(guard_id).or_insert(AHashMap::default());

            for m in asleep_since..wakeup_time {
                *minutes_slept.entry(m).or_insert(0) += 1;
            }
        } else {
            panic!("unhandled line '{line}'");
        }
    }

    mins_asleep
}

fn calculate_p1(mins_asleep: &AHashMap<i32, AHashMap<i32, i32>>) -> i32 {
    let best_guard = mins_asleep
        .iter()
        .max_by_key(|elem| elem.1.values().sum::<i32>())
        .expect("must have at least one guard")
        .0;

    let best_minute = mins_asleep
        .get(best_guard)
        .expect("best guard must exist")
        .iter()
        .max_by_key(|elem| elem.1)
        .expect("best guard must have slept")
        .0;

    best_guard * best_minute
}

fn calculate_p2(mins_asleep: &AHashMap<i32, AHashMap<i32, i32>>) -> i32 {
    mins_asleep
        .iter()
        .map(|g| (g.0, g.1.iter().max_by_key(|m| m.1).unwrap_or((&0, &0))))
        .max_by_key(|(_, (_, f))| *f)
        .map(|(g, (m, _))| g * m)
        .expect("best guard must exist")
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2018_04");
    const REAL_DATA: &str = include_str!("../../inputs/real/2018_04");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(&EXAMPLE_DATA)), 240);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(&parse(&EXAMPLE_DATA)), 4455);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(&REAL_DATA)), 71748);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(&REAL_DATA)), 106850);
    }
}
