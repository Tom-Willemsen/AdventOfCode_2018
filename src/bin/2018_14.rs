use advent_of_code_2018::{Cli, Parser};
use num::Integer;
use std::fs;

fn parse(raw_inp: &str) -> &str {
    raw_inp.trim()
}

fn simulate<F, T>(callback: F) -> T
where
    F: Fn(&[usize]) -> Option<T>,
{
    let mut recipes: Vec<usize> = vec![3, 7];

    let mut e1 = 0;
    let mut e2 = 1;

    loop {
        if e1 >= recipes.len() {
            e1 %= recipes.len();
        }
        if e2 >= recipes.len() {
            e2 %= recipes.len();
        }
        let c1 = recipes[e1];
        let c2 = recipes[e2];

        let (n1, n2) = (c1 + c2).div_rem(&10);
        if n1 != 0 {
            recipes.push(n1);
        }
        recipes.push(n2);

        e1 += c1 + 1;
        e2 += c2 + 1;

        if let Some(result) = callback(&recipes) {
            return result;
        }
    }
}

fn calculate_p1(data: &str) -> String {
    let data: usize = data.parse().expect("not a number");
    simulate(|recipes| {
        (recipes.len() >= data + 10).then_some(
            String::from_utf8(
                recipes
                    .iter()
                    .skip(data as usize)
                    .take(10)
                    .map(|&n| b'0' + n as u8)
                    .collect::<Vec<_>>(),
            )
            .expect("encoding"),
        )
    })
}

fn calculate_p2(data: &str) -> usize {
    let needle = data
        .bytes()
        .map(|b| (b - b'0') as usize)
        .collect::<Vec<_>>();

    simulate(|recipes| {
        (recipes.len() > needle.len() + 1)
            .then(|| {
                if recipes[recipes.len() - needle.len()..recipes.len()] == needle {
                    Some(recipes.len() - needle.len())
                } else if recipes[recipes.len() - needle.len() - 1..recipes.len() - 1] == needle {
                    Some(recipes.len() - needle.len() - 1)
                } else {
                    None
                }
            })
            .flatten()
    })
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let p1 = calculate_p1(data);
    let p2 = calculate_p2(data);
    println!("{p1}\n{p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_DATA: &str = include_str!("../../inputs/real/2018_14");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(parse("9")), "5158916779");
        assert_eq!(calculate_p1(parse("5")), "0124515891");
        assert_eq!(calculate_p1(parse("18")), "9251071085");
        assert_eq!(calculate_p1(parse("2018")), "5941429882");
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(parse(REAL_DATA)), "5715102879");
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(parse("51589")), 9);
        assert_eq!(calculate_p2(parse("01245")), 5);
        assert_eq!(calculate_p2(parse("92510")), 18);
        assert_eq!(calculate_p2(parse("59414")), 2018);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(parse(REAL_DATA)), 20225706);
    }
}
