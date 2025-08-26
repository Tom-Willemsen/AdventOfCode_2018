use advent_of_code_2018::{Cli, Parser};
use std::{collections::VecDeque, fs};

fn parse(raw_inp: &str) -> Vec<u8> {
    raw_inp.trim().bytes().collect()
}

fn reduced(data: &[u8], ignore: Option<u8>) -> Vec<u8> {
    let mut chain: Vec<u8> = vec![];
    let mut to_add: VecDeque<u8> = data.iter().copied().collect();

    while let Some(new_elem) = to_add.pop_front() {
        if let Some(ignore) = ignore
            && (new_elem == ignore || new_elem == ignore + 32)
        {
            continue;
        }
        if let Some(&last_elem) = chain.last()
            && (new_elem == last_elem + 32 || new_elem == last_elem - 32)
        {
            chain.pop();
            continue;
        }
        chain.push(new_elem);
    }

    chain
}

fn calculate(data: &[u8]) -> (usize, usize) {
    let d = reduced(data, None);
    let p1 = d.len();
    let p2 = (b'A'..=b'Z')
        .map(|r| reduced(&d, Some(r)).len())
        .min()
        .expect("non-empty");
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2018_05");
    const REAL_DATA: &str = include_str!("../../inputs/real/2018_05");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate(&parse(&EXAMPLE_DATA)).0, 10);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate(&parse(&EXAMPLE_DATA)).1, 4);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate(&parse(&REAL_DATA)).0, 9078);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate(&parse(&REAL_DATA)).1, 5698);
    }
}
