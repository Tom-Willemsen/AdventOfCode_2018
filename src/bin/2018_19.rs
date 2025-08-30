use advent_of_code_2018::{Cli, Parser};
use itertools::Itertools;
use num::integer::Roots;
use std::fs;
use std::str::FromStr;

type RegisterState = [usize; 6];

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
enum InstructionType {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl FromStr for InstructionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "addr" => Self::Addr,
            "addi" => Self::Addi,
            "mulr" => Self::Mulr,
            "muli" => Self::Muli,
            "banr" => Self::Banr,
            "bani" => Self::Bani,
            "borr" => Self::Borr,
            "bori" => Self::Bori,
            "setr" => Self::Setr,
            "seti" => Self::Seti,
            "gtir" => Self::Gtir,
            "gtri" => Self::Gtri,
            "gtrr" => Self::Gtrr,
            "eqir" => Self::Eqir,
            "eqri" => Self::Eqri,
            "eqrr" => Self::Eqrr,
            _ => Err(())?,
        })
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
struct Instruction {
    typ: InstructionType,
    a: usize,
    b: usize,
    out: usize,
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (typ, rest) = s.split_once(" ").ok_or(())?;
        let typ = typ.parse()?;

        let (a, b, out) = rest
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect_tuple()
            .ok_or(())?;

        Ok(Instruction { typ, a, b, out })
    }
}

fn apply_instruction(state: &mut RegisterState, instruction: &Instruction) {
    let val_a = match instruction.typ {
        InstructionType::Seti | InstructionType::Gtir | InstructionType::Eqir => instruction.a,
        _ => state[instruction.a],
    };

    let val_b = match instruction.typ {
        InstructionType::Addi
        | InstructionType::Muli
        | InstructionType::Bani
        | InstructionType::Bori
        | InstructionType::Gtri
        | InstructionType::Eqri => instruction.b,
        InstructionType::Seti => 0,
        _ => state[instruction.b],
    };

    let result =
        match instruction.typ {
            InstructionType::Addi | InstructionType::Addr => val_a + val_b,
            InstructionType::Mulr | InstructionType::Muli => val_a * val_b,
            InstructionType::Banr | InstructionType::Bani => val_a & val_b,
            InstructionType::Borr | InstructionType::Bori => val_a | val_b,
            InstructionType::Setr | InstructionType::Seti => val_a,
            InstructionType::Gtir | InstructionType::Gtri | InstructionType::Gtrr => {
                if val_a > val_b { 1 } else { 0 }
            }
            InstructionType::Eqir | InstructionType::Eqri | InstructionType::Eqrr => {
                if val_a == val_b { 1 } else { 0 }
            }
        };

    state[instruction.out] = result;
}

fn parse(raw_inp: &str) -> (usize, Vec<Instruction>) {
    let ip = raw_inp
        .lines()
        .next()
        .and_then(|line| line.split_once(" "))
        .and_then(|line| line.1.parse().ok())
        .expect("no instruction pointer");

    let instructions = raw_inp
        .lines()
        .skip(1)
        .filter_map(|line| line.parse().ok())
        .collect();

    (ip, instructions)
}

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
