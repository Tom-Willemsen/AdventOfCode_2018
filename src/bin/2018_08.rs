use advent_of_code_2018::{Cli, Parser};
use std::{collections::VecDeque, fs};

struct Node {
    children: Vec<Node>,
    meta: Vec<usize>,
}

impl Node {
    fn parse_node(data: &mut VecDeque<usize>) -> Node {
        let n_children = data.pop_front().expect("no children");
        let n_meta = data.pop_front().expect("no meta");

        let children = (0..n_children).map(|_| Node::parse_node(data)).collect();
        let meta = (0..n_meta).filter_map(|_| data.pop_front()).collect();

        Node { children, meta }
    }

    fn sum(&self) -> usize {
        self.meta.iter().sum::<usize>() + self.children.iter().map(|c| c.sum()).sum::<usize>()
    }

    fn value(&self) -> usize {
        if self.children.is_empty() {
            self.meta.iter().sum()
        } else {
            self.meta
                .iter()
                .filter_map(|m| self.children.get(m - 1).map(|n| n.value()))
                .sum()
        }
    }
}

fn parse(raw_inp: &str) -> Node {
    let mut dq = raw_inp
        .trim()
        .split(" ")
        .filter_map(|elem| elem.parse().ok())
        .collect::<VecDeque<usize>>();

    Node::parse_node(&mut dq)
}

fn calculate_p1(data: &Node) -> usize {
    data.sum()
}

fn calculate_p2(data: &Node) -> usize {
    data.value()
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2018_08");
    const REAL_DATA: &str = include_str!("../../inputs/real/2018_08");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(EXAMPLE_DATA)), 138);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(&parse(EXAMPLE_DATA)), 66);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(REAL_DATA)), 40977);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(REAL_DATA)), 27490);
    }
}
