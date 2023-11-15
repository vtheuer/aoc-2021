use Direction::*;
use Instruction::*;

use crate::day::Day;

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Move(usize),
    TurnLeft,
    TurnRight,
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn apply(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Up => (x, y - 1),
            Right => (x + 1, y),
            Down => (x, y + 1),
            Left => (x - 1, y),
        }
    }

    fn left(&self) -> Self {
        match self {
            Up => Left,
            Right => Up,
            Down => Right,
            Left => Down,
        }
    }

    fn right(&self) -> Self {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn value(&self) -> usize {
        match self {
            Up => 3,
            Right => 0,
            Down => 1,
            Left => 2,
        }
    }
}

#[derive(Debug)]
pub struct Day22 {
    map: Vec<Vec<Option<bool>>>,
    instructions: Vec<Instruction>,
}

type WrapFn = fn(&Day22, (usize, usize), Direction) -> ((usize, usize), Direction);

impl Day22 {
    fn next(&self, (x, y): (usize, usize), direction: Direction, wrap: WrapFn) -> ((usize, usize), Direction) {
        let map = &self.map;
        match direction {
            Up => {
                if y == 0 || x >= map[y - 1].len() || map[y - 1][x].is_none() {
                    wrap(self, (x, y), direction)
                } else {
                    ((x, y - 1), direction)
                }
            }
            Right => {
                if x == map[y].len() - 1 || y >= map.len() || map[y][x + 1].is_none() {
                    wrap(self, (x, y), direction)
                } else {
                    ((x + 1, y), direction)
                }
            }
            Down => {
                if y == map.len() - 1 || x >= map[y + 1].len() || map[y + 1][x].is_none() {
                    wrap(self, (x, y), direction)
                } else {
                    ((x, y + 1), direction)
                }
            }
            Left => {
                if x == 0 || y >= map.len() || map[y][x - 1].is_none() {
                    wrap(self, (x, y), direction)
                } else {
                    ((x - 1, y), direction)
                }
            }
        }
    }

    fn navigate(&self, wrap: WrapFn) -> usize {
        let mut position = (
            self.map[0]
                .iter()
                .enumerate()
                .find(|&(_, &c)| c == Some(false))
                .map(|(i, _)| i)
                .unwrap(),
            0,
        );
        let mut direction = Right;

        for instruction in &self.instructions {
            match instruction {
                &Move(n) => {
                    for _ in 0..n {
                        let (next_position, next_direction) = self.next(position, direction, wrap);
                        match self.map[next_position.1][next_position.0] {
                            Some(false) => {
                                position = next_position;
                                direction = next_direction;
                            }
                            Some(true) => break,
                            None => panic!("map[{}][{}] is None", next_position.1, next_position.0),
                        }
                    }
                }
                TurnLeft => direction = direction.left(),
                TurnRight => direction = direction.right(),
            }
        }

        1000 * (position.1 + 1) + 4 * (position.0 + 1) + direction.value()
    }

    fn wrap_flat(&self, (x, y): (usize, usize), direction: Direction) -> ((usize, usize), Direction) {
        (
            match direction {
                Up => (
                    x,
                    match x {
                        0..=49 => 199,
                        50..=99 => 149,
                        _ => 49,
                    },
                ),
                Right => (if y < 100 { 50 } else { 0 }, y),
                Down => (x, if x < 50 { 100 } else { 0 }),
                Left => (
                    match y {
                        0..=49 => 149,
                        50..=149 => 99,
                        _ => 49,
                    },
                    y,
                ),
            },
            direction,
        )
    }

    fn wrap_cube(&self, (x, y): (usize, usize), direction: Direction) -> ((usize, usize), Direction) {
        match direction {
            Up => match x {
                0..=49 => ((50, 50 + x), Right),
                50..=99 => ((0, 100 + x), Right),
                _ => ((x - 100, 199), Up),
            },
            Right => match y {
                0..=49 => ((99, 149 - y), Left),
                50..=99 => ((50 + y, 49), Up),
                100..=149 => ((149, 149 - y), Left),
                _ => ((y - 100, 149), Up),
            },
            Down => match x {
                0..=49 => ((100 + x, 0), Down),
                50..=99 => ((49, 100 + x), Left),
                _ => ((99, x - 50), Left),
            },
            Left => match y {
                0..=49 => ((0, 149 - y), Right),
                50..=99 => ((y - 50, 100), Down),
                100..=149 => ((50, 149 - y), Right),
                _ => ((y - 100, 0), Down),
            },
        }
    }
}

impl Day<'_> for Day22 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let (map, directions) = input.split_once("\n\n").unwrap();
        Self {
            map: map
                .lines()
                .map(|l| {
                    l.bytes()
                        .map(|c| match c {
                            b' ' => None,
                            b'.' => Some(false),
                            b'#' => Some(true),
                            _ => unreachable!(),
                        })
                        .collect()
                })
                .collect(),
            instructions: directions
                .lines()
                .next()
                .unwrap()
                .split_inclusive(|c| c == 'L' || c == 'R')
                .fold(vec![], |mut r, part| {
                    let last = part.chars().last().unwrap();
                    if last.is_ascii_alphabetic() {
                        r.push(Move(part[..part.len() - 1].parse().unwrap()));
                        r.push(match last {
                            'L' => TurnLeft,
                            'R' => TurnRight,
                            _ => unreachable!(),
                        });
                    } else {
                        r.push(Move(part.parse().unwrap()));
                    }
                    r
                }),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.navigate(Day22::wrap_flat)
    }

    fn part_2(&self) -> Self::T2 {
        self.navigate(Day22::wrap_cube)
    }
}
