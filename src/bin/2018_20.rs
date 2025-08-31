use advent_of_code_2018::{Cli, Parser};
use ahash::{AHashMap, AHashSet};
use std::{collections::VecDeque, fs};

#[derive(Debug)]
enum PathElement {
    Route(u8),
    Branch(Vec<Vec<PathElement>>),
}

fn travelled_squares(data: &[PathElement], mut y: i32, mut x: i32) -> Vec<(i32, i32)> {
    let mut ans = Vec::default();
    ans.push((0, 0));

    data.iter().for_each(|elem| match elem {
        PathElement::Route(dir) => {
            let (dy, dx) = match dir {
                b'N' => (-1, 0),
                b'E' => (0, 1),
                b'S' => (1, 0),
                b'W' => (0, -1),
                x => panic!("Unhandled direction {x}"),
            };
            ans.push((y + dy, x + dx));
            ans.push((y + 2 * dy, x + 2 * dx));
            y += 2 * dy;
            x += 2 * dx;
        }
        PathElement::Branch(branches) => {
            branches.iter().for_each(|branch| {
                ans.extend(travelled_squares(branch, y, x));
            });
        }
    });

    ans
}

fn make_path(regex: &mut VecDeque<u8>) -> Vec<PathElement> {
    let mut path = vec![];

    while let Some(item) = regex.pop_front() {
        match item {
            b'(' => {
                let mut subpaths = vec![];
                subpaths.push(make_path(regex));
                while let Some(b'|') = regex.pop_front() {
                    subpaths.push(make_path(regex));
                }
                path.push(PathElement::Branch(subpaths));
            }
            b'N' | b'E' | b'W' | b'S' => path.push(PathElement::Route(item)),
            other => {
                regex.push_front(other);
                break;
            }
        }
    }
    path
}

fn flood_fill(grid: &AHashSet<(i32, i32)>) -> AHashMap<(i32, i32), i32> {
    let mut costs = AHashMap::default();
    let mut q = VecDeque::default();
    q.push_back((0, 0, 0)); // y, x, cost

    while let Some((y, x, cost)) = q.pop_front() {
        if let Some(&prev_cost) = costs.get(&(y, x))
            && prev_cost <= cost
        {
            continue;
        }

        costs.insert((y, x), cost);

        if grid.contains(&(y + 1, x)) {
            q.push_back((y + 2, x, cost + 1));
        }
        if grid.contains(&(y - 1, x)) {
            q.push_back((y - 2, x, cost + 1));
        }
        if grid.contains(&(y, x + 1)) {
            q.push_back((y, x + 2, cost + 1));
        }
        if grid.contains(&(y, x - 1)) {
            q.push_back((y, x - 2, cost + 1));
        }
    }

    costs
}

fn parse(raw_inp: &str) -> AHashMap<(i32, i32), i32> {
    let mut inp: VecDeque<u8> = raw_inp.trim().bytes().collect();
    inp.pop_front(); // ^
    inp.pop_back(); // $
    let grid = travelled_squares(&make_path(&mut inp), 0, 0);
    flood_fill(&grid.into_iter().collect::<AHashSet<_>>())
}

fn calculate_p1(costs: &AHashMap<(i32, i32), i32>) -> i32 {
    *costs.values().max().unwrap_or(&0)
}

fn calculate_p2(costs: &AHashMap<(i32, i32), i32>) -> usize {
    costs.values().filter(|&n| *n >= 1000).count()
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let nums = parse(&inp);
    let p1 = calculate_p1(&nums);
    let p2 = calculate_p2(&nums);
    println!("{p1}\n{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2018_20");
    const REAL_DATA: &str = include_str!("../../inputs/real/2018_20");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(EXAMPLE_DATA)), 31);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(&REAL_DATA)), 3839);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(&REAL_DATA)), 8407);
    }
}
