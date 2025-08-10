use advent_of_code_2018::{Cli, Parser};
use ndarray::Array2;
use num::Integer;
use rayon::prelude::*;
use std::fs;

const GRID_SIZE: usize = 300;

fn power_level(x: i32, y: i32, serial_number: i32) -> i32 {
    let rack_id = x + 10;
    (((rack_id * y + serial_number) * rack_id) % 1000) / 100 - 5
}

fn parse(raw_inp: &str) -> Array2<i32> {
    let serial_number = raw_inp.trim().parse().expect("not a number");

    let mut arr = Array2::from_shape_fn((GRID_SIZE, GRID_SIZE), |(y, x)| {
        power_level(x as i32 + 1, y as i32 + 1, serial_number)
    });

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let upper = arr
                .get((y.wrapping_sub(1), x.wrapping_sub(1)))
                .unwrap_or(&0);
            let left = arr.get((y, x.wrapping_sub(1))).unwrap_or(&0);
            let up = arr.get((y.wrapping_sub(1), x)).unwrap_or(&0);

            arr[(y, x)] += up + left - upper;
        }
    }

    arr
}

fn region_area(data: &Array2<i32>, y: usize, x: usize, size: usize) -> i32 {
    if x + size >= GRID_SIZE || y + size >= GRID_SIZE {
        return i32::MIN;
    }
    data[(y + size, x + size)] - data[(y + size, x)] - data[(y, x + size)] + data[(y, x)]
}

fn calculate_p1(data: &Array2<i32>) -> String {
    (0..GRID_SIZE * GRID_SIZE)
        .into_par_iter()
        .map(|n| n.div_mod_floor(&GRID_SIZE))
        .max_by_key(|(y, x)| region_area(data, *y, *x, 3))
        .map(|(y, x)| format!("{},{}", x + 2, y + 2))
        .expect("non-empty")
}

fn calculate_p2(data: &Array2<i32>) -> String {
    (0..GRID_SIZE * GRID_SIZE)
        .into_par_iter()
        .map(|n| n.div_mod_floor(&GRID_SIZE))
        .flat_map_iter(|(y, x)| (1..GRID_SIZE).map(move |s| (y, x, s)))
        .max_by_key(|(y, x, size)| region_area(data, *y, *x, *size))
        .map(|(y, x, size)| format!("{},{},{}", x + 2, y + 2, size))
        .expect("non-empty")
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

    const REAL_DATA: &str = include_str!("../../inputs/real/2018_11");

    #[test]
    fn test_p1_cells() {
        assert_eq!(power_level(3, 5, 8), 4);
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
    }

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse("18")), "33,45");
        assert_eq!(calculate_p1(&parse("42")), "21,61");
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(REAL_DATA)), "20,34");
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(&parse("18")), "90,269,16");
        assert_eq!(calculate_p2(&parse("42")), "232,251,12");
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(REAL_DATA)), "90,57,15");
    }
}
