use Direction::Forward;

use crate::day::Day;
use crate::year_2021::day02::Direction::{Down, Up};

#[derive(Debug, Copy, Clone)]
enum Direction {
    Forward,
    Down,
    Up,
}

pub struct Day02 {
    instructions: Vec<(Direction, usize)>,
}

impl Day<'_> for Day02 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            instructions: input
                .lines()
                .map(|l| {
                    let mut i = l.split(' ');
                    Some((
                        match i.next()? {
                            "forward" => Forward,
                            "down" => Down,
                            "up" => Up,
                            _ => unreachable!(),
                        },
                        i.next()?.parse().ok()?,
                    ))
                })
                .map(Option::unwrap)
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let (x, y) = self
            .instructions
            .iter()
            .fold((0, 0), |(x, y), &(direction, n)| match direction {
                Forward => (x + n, y),
                Down => (x, y + n),
                Up => (x, y - n),
            });
        x * y
    }

    fn part_2(&self) -> Self::T2 {
        let (x, y, _) = self
            .instructions
            .iter()
            .fold((0, 0, 0), |(x, y, aim), &(direction, n)| match direction {
                Forward => (x + n, y + aim * n, aim),
                Down => (x, y, aim + n),
                Up => (x, y, aim - n),
            });
        x * y
    }
}
