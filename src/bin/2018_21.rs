use advent_of_code_2018::vm::{Instruction, apply_instruction, parse};
use advent_of_code_2018::{Cli, Parser};
use std::fs;

fn calculate(ip_register: usize, data: &[Instruction]) -> (usize, usize) {
    let mut ip = 0;
    let mut register_state = [0; 6];

    let mut possible_answers = vec![];

    let result_register = data[28].a;
    let div_result_register = data[26].out;

    while ip < data.len() {
        if ip == 17 {
            // 'live-patch' this series of instructions to just be a division and move on.
            register_state[div_result_register] /= 256;
            ip = 27;
            continue;
        }
        if ip == 28 {
            let r = register_state[result_register];
            if possible_answers.contains(&r) {
                break;
            }
            possible_answers.push(r);
        }
        register_state[ip_register] = ip;
        apply_instruction(&mut register_state, &data[ip]);
        ip = register_state[ip_register] + 1;
    }

    let p1 = *possible_answers.first().expect("nonempty");
    let p2 = *possible_answers.last().expect("nonempty");
    (p1, p2)
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let (ip_register, data) = parse(&inp);
    let (p1, p2) = calculate(ip_register, &data);
    println!("{p1}\n{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_DATA: &str = include_str!("../../inputs/real/2018_21");

    #[test]
    fn test_real() {
        let (ipr, data) = parse(REAL_DATA);
        assert_eq!(calculate(ipr, &data), (9959629, 12691260));
    }
}
