use Direction::*;
use Rotation::*;

use crate::day::Day;

#[derive(Clone, Copy, Debug, std::cmp::PartialEq)]
enum Direction {
    East,
    South,
    West,
    North,
}

impl Direction {
    fn apply(&self, (x, y): (isize, isize), value: isize) -> (isize, isize) {
        match self {
            East => (x + value, y),
            South => (x, y + value),
            West => (x - value, y),
            North => (x, y - value),
        }
    }

    fn ordinal(&self) -> usize {
        match self {
            East => 0,
            South => 1,
            West => 2,
            North => 3,
        }
    }
}

#[derive(Debug, std::cmp::PartialEq)]
enum Rotation {
    Left,
    Right,
}

const DIRECTIONS: [Direction; 4] = [East, South, West, North];

impl Rotation {
    fn apply(&self, direction: Direction, value: isize) -> Direction {
        let n = direction.ordinal() as isize + if *self == Left { -1 } else { 1 } * value / 90;
        DIRECTIONS[(if n < 0 { 4 } else { 0 } + (n % 4)) as usize]
    }

    fn rotate(&self, angle: isize, (x, y): (isize, isize)) -> (isize, isize) {
        if angle == 0 {
            (x, y)
        } else {
            self.rotate(
                angle - 90,
                match self {
                    Left => (y, -x),
                    Right => (-y, x),
                },
            )
        }
    }
}

#[derive(Debug)]
enum Action {
    Direction(Direction),
    Rotation(Rotation),
    Forward,
}

pub struct Day12 {
    instructions: Vec<(Action, isize)>,
}

impl Day<'_> for Day12 {
    type T1 = isize;
    type T2 = isize;

    fn new(input: &str) -> Self {
        Day12 {
            instructions: input
                .lines()
                .map(|l| {
                    let (action, value) = l.split_at(1);
                    (
                        match action {
                            "E" => Action::Direction(East),
                            "S" => Action::Direction(South),
                            "W" => Action::Direction(West),
                            "N" => Action::Direction(North),
                            "L" => Action::Rotation(Left),
                            "R" => Action::Rotation(Right),
                            "F" => Action::Forward,
                            a => unreachable!("unexpected action {}", a),
                        },
                        value.parse().unwrap(),
                    )
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let (_, (x, y)) =
            self.instructions
                .iter()
                .fold(
                    (East, (0isize, 0isize)),
                    |(d, p), (action, value)| match action {
                        Action::Direction(direction) => (d, direction.apply(p, *value)),
                        Action::Rotation(rotation) => (rotation.apply(d, *value), p),
                        Action::Forward => (d, d.apply(p, *value)),
                    },
                );
        x.abs() + y.abs()
    }

    fn part_2(&self) -> Self::T2 {
        let (_, (x, y)) = self.instructions.iter().fold(
            ((10, -1), (0isize, 0isize)),
            |(w, p), (action, value)| match action {
                Action::Direction(direction) => (direction.apply(w, *value), p),
                Action::Rotation(rotation) => (rotation.rotate(*value, w), p),
                Action::Forward => (w, (p.0 + value * w.0, p.1 + value * w.1)),
            },
        );
        x.abs() + y.abs()
    }
}
