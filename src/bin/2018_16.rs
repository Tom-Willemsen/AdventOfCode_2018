use advent_of_code_2018::vm::{InstructionType, RegisterState};
use advent_of_code_2018::{Cli, Parser};
use ahash::{AHashMap, AHashSet};
use itertools::Itertools;
use std::fs;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
struct Instruction {
    raw_typ: usize,
    a: usize,
    b: usize,
    out: usize,
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_typ, a, b, out) = s
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect_tuple()
            .ok_or(())?;

        Ok(Instruction { raw_typ, a, b, out })
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
struct P1InputData {
    before: RegisterState<4>,
    instruction: Instruction,
    after: RegisterState<4>,
}

impl FromStr for P1InputData {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.split("\n").collect::<Vec<_>>();
        let before = lines[0]
            .strip_prefix("Before: [")
            .expect("bad format")
            .strip_suffix("]")
            .expect("bad format")
            .split(", ")
            .filter_map(|n| n.parse().ok())
            .collect::<Vec<_>>();

        let instruction = lines[1].parse()?;

        let after = lines[2]
            .strip_prefix("After:  [")
            .expect("bad format")
            .strip_suffix("]")
            .expect("bad format")
            .split(", ")
            .filter_map(|n| n.parse().ok())
            .collect::<Vec<_>>();

        Ok(P1InputData {
            before: before.try_into().expect("bad"),
            instruction,
            after: after.try_into().expect("bad"),
        })
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct InputData {
    p1: Vec<P1InputData>,
    p2: Vec<Instruction>,
}

fn apply_instruction(
    state: &RegisterState<4>,
    instruction: &Instruction,
    typ: InstructionType,
) -> RegisterState<4> {
    let val_a = match typ {
        InstructionType::Seti | InstructionType::Gtir | InstructionType::Eqir => instruction.a,
        _ => state[instruction.a],
    };

    let val_b = match typ {
        InstructionType::Addi
        | InstructionType::Muli
        | InstructionType::Bani
        | InstructionType::Bori
        | InstructionType::Gtri
        | InstructionType::Eqri => instruction.b,
        _ => state[instruction.b],
    };

    let result =
        match typ {
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

    let mut next_state = *state;
    next_state[instruction.out] = result;
    next_state
}

fn parse(raw_inp: &str) -> InputData {
    let (p1_inp, p2_inp) = raw_inp.split_once("\n\n\n\n").expect("invalid format");

    let p1 = p1_inp
        .split("\n\n")
        .filter_map(|group| group.parse().ok())
        .collect::<Vec<_>>();

    let p2 = p2_inp
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect::<Vec<_>>();

    InputData { p1, p2 }
}

fn calculate_p1(data: &InputData) -> usize {
    data.p1
        .iter()
        .filter(|group| {
            InstructionType::VALUES
                .into_iter()
                .filter(|&inst_type| {
                    apply_instruction(&group.before, &group.instruction, inst_type) == group.after
                })
                .take(3)
                .count()
                >= 3
        })
        .count()
}

fn calculate_p2(data: &InputData) -> usize {
    let mut possible_ops: AHashMap<usize, AHashSet<InstructionType>> = AHashMap::default();
    for i in 0..16 {
        for typ in InstructionType::VALUES.iter() {
            possible_ops.entry(i).or_default().insert(*typ);
        }
    }

    while possible_ops.iter().any(|(_, v)| v.len() != 1) {
        data.p1.iter().for_each(|group| {
            let possible = possible_ops
                .get(&group.instruction.raw_typ)
                .expect("invalid instruction")
                .clone();

            for &p in possible.iter() {
                let matches =
                    apply_instruction(&group.before, &group.instruction, p) == group.after;
                if !matches {
                    possible_ops
                        .get_mut(&group.instruction.raw_typ)
                        .expect("invalid instruction")
                        .remove(&p);
                }
            }
        });

        possible_ops
            .clone()
            .into_iter()
            .filter(|(_, v)| v.len() == 1)
            .for_each(|(k, v)| {
                possible_ops
                    .iter_mut()
                    .filter(|(k2, _)| *k2 != &k)
                    .for_each(|(_, v2)| {
                        v2.remove(v.iter().next().expect("should have at least 1"));
                    });
            });
    }

    let mut state = [0; 4];

    for inst in data.p2.iter() {
        state = apply_instruction(
            &state,
            inst,
            *possible_ops
                .get(&inst.raw_typ)
                .expect("should exist")
                .iter()
                .next()
                .expect("should exist"),
        );
    }

    state[0]
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2018_16");
    const REAL_DATA: &str = include_str!("../../inputs/real/2018_16");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(EXAMPLE_DATA)), 1);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(REAL_DATA)), 560);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(REAL_DATA)), 622);
    }
}
