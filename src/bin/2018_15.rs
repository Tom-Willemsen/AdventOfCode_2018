use advent_of_code_2018::{Cli, Parser, grid_util::make_byte_grid};
use ahash::AHashSet;
use itertools::Itertools;
use ndarray::Array2;
use std::cell::{RefCell, RefMut};
use std::collections::VecDeque;
use std::fs;

#[derive(PartialEq, Eq, Debug, Clone, PartialOrd, Ord)]
enum UnitClass {
    Goblin,
    Elf,
}

impl From<u8> for UnitClass {
    fn from(value: u8) -> Self {
        match value {
            b'G' => UnitClass::Goblin,
            b'E' => UnitClass::Elf,
            _ => panic!("Unhandled unit class"),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, PartialOrd, Ord)]
struct Unit {
    y: usize,
    x: usize,
    class: UnitClass,
    hp: i64,
}

impl Unit {
    fn is_alive(&self) -> bool {
        self.hp > 0
    }

    fn is_adjacent(&self, other: &Unit) -> bool {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) == 1
    }

    fn valid_movement_targets(&self, state: &GameState) -> AHashSet<(usize, usize)> {
        state
            .units
            .iter()
            .filter_map(|u| u.try_borrow().ok())
            .filter(|unit| unit.is_alive() && unit.class != self.class)
            .flat_map(|unit| {
                [
                    (unit.y - 1, unit.x),
                    (unit.y, unit.x - 1),
                    (unit.y, unit.x + 1),
                    (unit.y + 1, unit.x),
                ]
            })
            .filter(|&(y, x)| state.is_empty_square(y, x))
            .collect()
    }

    fn pathfind(
        &self,
        sy: usize,
        sx: usize,
        targets: &AHashSet<(usize, usize)>,
        state: &GameState,
    ) -> Vec<(i64, usize, usize)> {
        debug_assert!(self.y.abs_diff(sy) + self.x.abs_diff(sx) == 1);

        if targets.is_empty() {
            return vec![];
        }

        let mut costs = Array2::from_shape_simple_fn(state.board.dim(), || i64::MAX);
        let mut q = VecDeque::default();

        costs[(sy, sx)] = 1;
        q.push_back((1, sy, sx));

        while let Some((cost, y, x)) = q.pop_front() {
            if costs[(y, x)] < cost || targets.contains(&(y, x)) {
                continue;
            }
            state.valid_neighbours_of(y, x).for_each(|(ny, nx)| {
                if costs[(ny, nx)] > cost + 1 {
                    costs[(ny, nx)] = cost + 1;
                    q.push_back((cost + 1, ny, nx));
                }
            });
        }

        targets
            .iter()
            .filter_map(|&(ty, tx)| {
                let cost = costs[(ty, tx)];
                (cost != i64::MAX).then_some((cost, ty, tx))
            })
            .collect()
    }

    fn movement(&mut self, state: &GameState) {
        let valid_targets = self.valid_movement_targets(state);

        let target = state
            .valid_neighbours_of(self.y, self.x)
            .flat_map(|(sy, sx)| {
                self.pathfind(sy, sx, &valid_targets, state)
                    .into_iter()
                    .map(move |(c, ty, tx)| (c, ty, tx, sy, sx))
            })
            .sorted_unstable()
            .next()
            .map(|(_, _, _, y, x)| (y, x));

        if let Some((y, x)) = target {
            let mut occupied = state.occupied_squares.borrow_mut();
            occupied.remove(&(self.y, self.x));
            occupied.insert((y, x));
            self.y = y;
            self.x = x;
        }
    }

    fn take_turn(&mut self, state: &GameState) {
        if let Some(mut opponent) = self.opponent_in_range(state) {
            self.attack(&mut opponent, state);
        } else {
            self.movement(state);

            if let Some(mut opponent) = self.opponent_in_range(state) {
                self.attack(&mut opponent, state);
            }
        }
    }

    fn attack(&self, opp: &mut Unit, state: &GameState) {
        opp.hp -= if opp.class == UnitClass::Goblin {
            state.elf_attack_power
        } else {
            3
        };

        if !opp.is_alive() {
            state.occupied_squares.borrow_mut().remove(&(opp.y, opp.x));
        }
    }

    fn opponent_in_range<'a>(&self, state: &'a GameState) -> Option<RefMut<'a, Unit>> {
        state
            .units
            .iter()
            .filter_map(|u| u.try_borrow_mut().ok())
            .filter(|unit| unit.class != self.class && unit.is_alive() && self.is_adjacent(unit))
            .sorted_unstable_by_key(|unit| (unit.hp, unit.y, unit.x))
            .next()
    }
}

#[derive(Clone)]
struct GameState {
    board: Array2<u8>,
    units: Vec<RefCell<Unit>>,
    round: i64,
    elf_attack_power: i64,
    occupied_squares: RefCell<AHashSet<(usize, usize)>>,
}

impl GameState {
    fn is_empty_square(&self, y: usize, x: usize) -> bool {
        self.board.get((y, x)) == Some(&b'.') && !self.occupied_squares.borrow().contains(&(y, x))
    }

    fn valid_neighbours_of(&self, y: usize, x: usize) -> impl Iterator<Item = (usize, usize)> {
        [(y - 1, x), (y, x - 1), (y, x + 1), (y + 1, x)]
            .into_iter()
            .filter(|&(y, x)| self.is_empty_square(y, x))
    }

    fn left(&self) -> (usize, usize) {
        self.units
            .iter()
            .map(|u| u.borrow())
            .filter(|u| u.is_alive())
            .fold((0, 0), |(g, e), u| match u.class {
                UnitClass::Goblin => (g + 1, e),
                UnitClass::Elf => (g, e + 1),
            })
    }

    fn take_turns(&mut self) -> bool {
        for unit in self.units.iter() {
            if !unit.borrow().is_alive() {
                continue;
            }

            let (g, e) = self.left();
            if g == 0 || e == 0 {
                return false;
            }

            unit.borrow_mut().take_turn(self);
        }
        true
    }

    fn play_single_round(&mut self) -> Option<i64> {
        self.units.sort_unstable();

        let was_full_round = self.take_turns();

        if !was_full_round {
            return Some(self.outcome());
        }

        self.round += 1;
        None
    }

    fn play(&mut self) -> i64 {
        loop {
            if let Some(outcome) = self.play_single_round() {
                return outcome;
            }
        }
    }

    fn outcome(&self) -> i64 {
        self.round
            * self
                .units
                .iter()
                .filter(|&unit| unit.borrow().is_alive())
                .map(|unit| unit.borrow().hp)
                .sum::<i64>()
    }
}

fn parse(raw_inp: &str) -> GameState {
    let mut board = make_byte_grid(raw_inp);

    let mut units = vec![];
    let mut occupied_squares = AHashSet::default();

    board
        .indexed_iter_mut()
        .filter(|(_, elem)| *elem == &b'G' || *elem == &b'E')
        .for_each(|((y, x), elem)| {
            units.push(RefCell::new(Unit {
                y,
                x,
                class: (*elem).into(),
                hp: 200,
            }));

            occupied_squares.insert((y, x));

            *elem = b'.';
        });

    GameState {
        board,
        units,
        round: 0,
        elf_attack_power: 3,
        occupied_squares: RefCell::new(occupied_squares),
    }
}

fn calculate_p1(data: &GameState) -> i64 {
    let mut state = data.clone();
    state.play()
}

fn calculate_p2(data: &GameState) -> i64 {
    let (_, initial_elves) = data.left();

    for elf_attack in 4..200 {
        let mut state = data.clone();
        state.elf_attack_power = elf_attack;

        let outcome = state.play();
        let (_, elves) = state.left();
        if elves == initial_elves {
            return outcome;
        }
    }
    panic!("no p2 answer");
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2018_15");
    const EXAMPLE_DATA_2: &str = include_str!("../../inputs/examples/2018_15_2");
    const EXAMPLE_DATA_3: &str = include_str!("../../inputs/examples/2018_15_3");
    const EXAMPLE_DATA_4: &str = include_str!("../../inputs/examples/2018_15_4");
    const EXAMPLE_DATA_5: &str = include_str!("../../inputs/examples/2018_15_5");
    const EXAMPLE_DATA_6: &str = include_str!("../../inputs/examples/2018_15_6");
    const REAL_DATA: &str = include_str!("../../inputs/real/2018_15");
    const REAL_DATA_2: &str = include_str!("../../inputs/real/2018_15_2");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(EXAMPLE_DATA)), 27730);
    }

    #[test]
    fn test_p1_example_2() {
        assert_eq!(calculate_p1(&parse(EXAMPLE_DATA_2)), 36334);
    }

    #[test]
    fn test_p1_example_3() {
        assert_eq!(calculate_p1(&parse(EXAMPLE_DATA_3)), 39514);
    }

    #[test]
    fn test_p1_example_4() {
        assert_eq!(calculate_p1(&parse(EXAMPLE_DATA_4)), 27755);
    }

    #[test]
    fn test_p1_example_5() {
        assert_eq!(calculate_p1(&parse(EXAMPLE_DATA_5)), 28944);
    }

    #[test]
    fn test_p1_example_6() {
        assert_eq!(calculate_p1(&parse(EXAMPLE_DATA_6)), 18740);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(REAL_DATA)), 257954);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(&parse(EXAMPLE_DATA)), 4988);
    }

    #[test]
    fn test_p2_example_3() {
        assert_eq!(calculate_p2(&parse(EXAMPLE_DATA_3)), 31284);
    }

    #[test]
    fn test_p2_example_4() {
        assert_eq!(calculate_p2(&parse(EXAMPLE_DATA_4)), 3478);
    }

    #[test]
    fn test_p2_example_5() {
        assert_eq!(calculate_p2(&parse(EXAMPLE_DATA_5)), 6474);
    }

    #[test]
    fn test_p2_example_6() {
        assert_eq!(calculate_p2(&parse(EXAMPLE_DATA_6)), 1140);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(REAL_DATA)), 51041);
    }

    #[test]
    fn test_p2_real_2() {
        assert_eq!(calculate_p2(&parse(REAL_DATA_2)), 62958);
    }
}
