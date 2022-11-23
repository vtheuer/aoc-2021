use std::convert::identity;

use regex::bytes::SubCaptureMatches;
use regex::{Captures, Regex};

use Action::On;

use crate::day::Day;
use crate::year_2015::day06::Action::{Off, Toggle};

#[derive(Debug, Copy, Clone)]
enum Action {
    On,
    Off,
    Toggle,
}

type Instruction = (Action, (usize, usize), (usize, usize));

pub struct Day06 {
    instructions: Vec<Instruction>,
}

impl Day<'_> for Day06 {
    type T1 = usize;
    type T2 = isize;

    fn new(input: &str) -> Self {
        let re = Regex::new("(turn (?:on|off)|toggle) (\\d+),(\\d+) through (\\d+),(\\d+)").unwrap();
        Self {
            instructions: input
                .lines()
                .map(|l| {
                    let captures = re.captures(l)?;
                    let mut it = captures.iter().skip(1);
                    Some((
                        match it.next()??.as_str() {
                            "turn on" => On,
                            "turn off" => Off,
                            "toggle" => Toggle,
                            s => unreachable!(s),
                        },
                        (
                            it.next()??.as_str().parse::<usize>().ok()?,
                            it.next()??.as_str().parse::<usize>().ok()?,
                        ),
                        (
                            it.next()??.as_str().parse::<usize>().ok()?,
                            it.next()??.as_str().parse::<usize>().ok()?,
                        ),
                    ))
                })
                .map(|i| i.unwrap())
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let mut grid = [[false; 1000]; 1000];
        for &(action, (fx, fy), (tx, ty)) in self.instructions.iter() {
            for row in grid.iter_mut().take(ty + 1).skip(fy) {
                for cell in row.iter_mut().take(tx + 1).skip(fx) {
                    *cell = match action {
                        On => true,
                        Off => false,
                        Toggle => !*cell,
                    }
                }
            }
        }
        grid.iter().map(|row| row.iter().filter(|&&b| b).count()).sum()
    }

    fn part_2(&self) -> Self::T2 {
        let mut grid = [[0isize; 1000]; 1000];
        for &(action, (fx, fy), (tx, ty)) in self.instructions.iter() {
            for row in grid.iter_mut().take(ty + 1).skip(fy) {
                for cell in row.iter_mut().take(tx + 1).skip(fx) {
                    *cell += match action {
                        On => 1,
                        Off => -1,
                        Toggle => 2,
                    }
                }
            }
        }
        grid.iter().map(|row| row.iter().copied().sum::<isize>()).sum()
    }
}
