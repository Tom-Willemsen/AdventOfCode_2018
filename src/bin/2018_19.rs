use advent_of_code_2018::vm::{Instruction, apply_instruction, parse};
use advent_of_code_2018::{Cli, Parser};
use num::integer::Roots;
use std::fs;

fn divisors(n: usize) -> impl Iterator<Item = usize> {
    (1..=n.sqrt())
        .filter(move |d| n % d == 0)
        .flat_map(move |d| [d, num::Integer::div_floor(&n, &d)])
}

fn calculate<const R0: usize>(ip_register: usize, data: &[Instruction]) -> usize {
    let mut ip = 0;
    let mut register_state = [0; 6];
    register_state[0] = R0;

    let mut n_instructions = 0;
    while ip < data.len() && n_instructions < data.len() {
        n_instructions += 1;
        register_state[ip_register] = ip;
        apply_instruction(&mut register_state, &data[ip]);
        ip = register_state[ip_register] + 1;
    }

    let target_number = register_state.into_iter().max().expect("nonempty");
    divisors(target_number).sum()
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let (ip_register, data) = parse(&inp);
    let p1 = calculate::<0>(ip_register, &data);
    let p2 = calculate::<1>(ip_register, &data);
    println!("{p1}\n{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_DATA: &str = include_str!("../../inputs/real/2018_19");

    #[test]
    fn test_real_p1() {
        let (ipr, data) = parse(REAL_DATA);
        assert_eq!(calculate::<0>(ipr, &data), 888);
    }

    #[test]
    fn test_real_p2() {
        let (ipr, data) = parse(REAL_DATA);
        assert_eq!(calculate::<1>(ipr, &data), 10708992);
    }
}
