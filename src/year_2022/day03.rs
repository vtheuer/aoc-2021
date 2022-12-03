use std::io::BufRead;
use std::ops::BitAnd;

use crate::day::Day;
use crate::util::SortableByKey;

pub struct Day03 {
    rucksacks: Vec<Vec<u8>>,
}

fn parse(b: &[u8]) -> u64 {
    b.iter().fold(0, |sack, c| sack | (1u64 << c))
}

impl Day<'_> for Day03 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            rucksacks: input
                .lines()
                .map(|l| {
                    l.bytes()
                        .map(|c| {
                            if c.is_ascii_lowercase() {
                                c - b'a'
                            } else {
                                26 + c - b'A'
                            }
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.rucksacks
            .iter()
            .map(|sack| (parse(&sack[..sack.len() / 2]) & parse(&sack[sack.len() / 2..])).trailing_zeros() as usize + 1)
            .sum()
    }

    fn part_2(&self) -> Self::T2 {
        self.rucksacks
            .chunks(3)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|sack| parse(sack))
                    .reduce(u64::bitand)
                    .unwrap()
                    .trailing_zeros() as usize
                    + 1
            })
            .sum()
    }
}
