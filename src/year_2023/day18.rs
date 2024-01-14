use std::str::from_utf8;

use Direction::*;

use crate::day::Day;
use crate::util::direction::Direction;
use crate::util::direction::Direction::*;

pub struct Day18<'a> {
    instructions: Vec<(Direction, u8, &'a [u8])>,
}

fn count_dug(instructions: impl Iterator<Item = (Direction, isize)>) -> isize {
    instructions
        .fold((0, 0, 0), |(area, x, y), (direction, distance)| {
            let (nx, ny) = direction.apply_n((x, y), distance);
            // https://en.wikipedia.org/wiki/Shoelace_formula#Shoelace_formula
            (area + (x * ny) - (nx * y) + distance, nx, ny)
        })
        .0
        / 2
        + 1
}

impl<'a> Day<'a> for Day18<'a> {
    type T1 = isize;
    type T2 = isize;

    fn new(input: &'a str) -> Self {
        Self {
            instructions: input
                .lines()
                .map_while(|l| {
                    let mut parts = l.split(' ');
                    Some((
                        match parts.next()? {
                            "U" => Up,
                            "R" => Right,
                            "D" => Down,
                            "L" => Left,
                            _ => unreachable!(),
                        },
                        parts.next()?.parse().ok()?,
                        parts.next()?.trim_matches(|c| matches!(c, '#' | '(' | ')')).as_bytes(),
                    ))
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        count_dug(
            self.instructions
                .iter()
                .map(|&(direction, distance, _)| (direction, distance as isize)),
        )
    }

    fn part_2(&self) -> Self::T2 {
        count_dug(self.instructions.iter().map(|&(_, _, hex)| {
            (
                [Right, Down, Left, Up][(hex[5] - b'0') as usize],
                isize::from_str_radix(from_utf8(&hex[..5]).unwrap(), 16).unwrap(),
            )
        }))
    }
}
