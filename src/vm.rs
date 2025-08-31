use itertools::Itertools;
use std::str::FromStr;

pub type RegisterState<const RS: usize> = [usize; RS];

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
pub enum InstructionType {
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

impl InstructionType {
    pub const VALUES: [Self; 16] = [
        Self::Addr,
        Self::Addi,
        Self::Mulr,
        Self::Muli,
        Self::Banr,
        Self::Bani,
        Self::Borr,
        Self::Bori,
        Self::Setr,
        Self::Seti,
        Self::Gtir,
        Self::Gtri,
        Self::Gtrr,
        Self::Eqir,
        Self::Eqri,
        Self::Eqrr,
    ];
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
pub struct Instruction {
    pub typ: InstructionType,
    pub a: usize,
    pub b: usize,
    pub out: usize,
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

pub fn apply_instruction<const RS: usize>(
    state: &mut RegisterState<RS>,
    instruction: &Instruction,
) {
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

pub fn parse(raw_inp: &str) -> (usize, Vec<Instruction>) {
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
