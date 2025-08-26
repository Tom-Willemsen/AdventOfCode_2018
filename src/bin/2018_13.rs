use advent_of_code_2018::grid_util::make_byte_grid;
use advent_of_code_2018::{Cli, Parser};
use ndarray::Array2;
use std::cell::RefCell;
use std::fs;

#[derive(PartialEq, Eq, Debug, Clone, Copy, PartialOrd, Ord)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            b'v' => Direction::Down,
            b'^' => Direction::Up,
            b'>' => Direction::Right,
            b'<' => Direction::Left,
            _ => panic!("invalid direction"),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, PartialOrd, Ord)]
struct Cart {
    y: usize,
    x: usize,
    intersection_counter: usize,
    dir: Direction,
    is_removed: bool,
}

impl Cart {
    fn move_cart(&mut self, grid: &Array2<u8>) {
        match grid.get((self.y, self.x)) {
            Some(b'/') => match self.dir {
                Direction::Right => self.dir = Direction::Up,
                Direction::Down => self.dir = Direction::Left,
                Direction::Left => self.dir = Direction::Down,
                Direction::Up => self.dir = Direction::Right,
            },
            Some(b'\\') => match self.dir {
                Direction::Down => self.dir = Direction::Right,
                Direction::Left => self.dir = Direction::Up,
                Direction::Right => self.dir = Direction::Down,
                Direction::Up => self.dir = Direction::Left,
            },
            Some(b'+') => {
                if self.intersection_counter % 3 == 0 {
                    match self.dir {
                        Direction::Up => self.dir = Direction::Left,
                        Direction::Right => self.dir = Direction::Up,
                        Direction::Down => self.dir = Direction::Right,
                        Direction::Left => self.dir = Direction::Down,
                    }
                } else if self.intersection_counter % 3 == 2 {
                    match self.dir {
                        Direction::Up => self.dir = Direction::Right,
                        Direction::Right => self.dir = Direction::Down,
                        Direction::Down => self.dir = Direction::Left,
                        Direction::Left => self.dir = Direction::Up,
                    }
                }
                self.intersection_counter += 1;
            }
            None => panic!("Cart out of bounds"),
            _ => {}
        }
        match self.dir {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
        }
    }
}

fn parse(raw_inp: &str) -> (Array2<u8>, Vec<Cart>) {
    let mut grid = make_byte_grid(raw_inp);
    let mut carts: Vec<Cart> = vec![];

    grid.indexed_iter_mut().for_each(|((y, x), e)| match *e {
        b'v' | b'^' | b'>' | b'<' => {
            carts.push(Cart {
                y,
                x,
                intersection_counter: 0,
                dir: (*e).into(),
                is_removed: false,
            });
            *e = if *e == b'v' || *e == b'^' { b'|' } else { b'-' };
        }
        _ => {}
    });

    (grid, carts)
}

fn calculate(data: &(Array2<u8>, Vec<Cart>)) -> (String, String) {
    let mut p1 = None;
    let grid = &data.0;
    let mut carts = data.1.iter().map(|c| RefCell::new(*c)).collect::<Vec<_>>();
    while carts.iter().filter(|c| !c.borrow().is_removed).count() > 1 {
        carts.sort_unstable();

        for cart in carts.iter() {
            let (y, x) = {
                let mut c = cart.borrow_mut();
                if c.is_removed {
                    continue;
                }
                c.move_cart(grid);
                (c.y, c.x)
            };

            if carts
                .iter()
                .filter(|other| {
                    let other = other.borrow();
                    !other.is_removed && other.y == y && other.x == x
                })
                .count()
                != 1
            {
                carts.iter().for_each(|other| {
                    let mut other = other.borrow_mut();
                    if other.x == x && other.y == y {
                        other.is_removed = true;
                    }
                });

                if p1.is_none() {
                    p1 = Some(format!("{},{}", x, y))
                }
            }
        }
    }

    (
        p1.unwrap_or("no p1 answer".to_owned()),
        carts
            .iter()
            .find(|c| !c.borrow().is_removed)
            .map(|c| format!("{},{}", c.borrow().x, c.borrow().y))
            .unwrap_or("no p2 answer".to_owned()),
    )
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2018_13");
    const REAL_DATA: &str = include_str!("../../inputs/real/2018_13");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate(&parse(EXAMPLE_DATA)).0, "7,3");
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate(&parse(REAL_DATA)).0, "74,87");
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate(&parse(REAL_DATA)).1, "29,74");
    }
}
