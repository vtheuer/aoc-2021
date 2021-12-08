use cmp::max;
use std::cmp;
use std::cmp::min;
use std::convert::identity;
use std::thread::sleep;
use std::time::Duration;

use crate::day::Day;
use crate::util::{split_pair, SortableByKey};

const UNIQUE_SETS: [u8; 4] = [2u8, 3u8, 4u8, 7u8];

pub struct Day08 {
    entries: Vec<(Vec<u8>, Vec<u8>)>,
}

fn parse_segments(left: &str) -> Vec<u8> {
    left.split(' ')
        .map(|x| {
            x.bytes().fold(0u8, |n, b| {
                n | match b {
                    b'a' => 0b0000001,
                    b'b' => 0b0000010,
                    b'c' => 0b0000100,
                    b'd' => 0b0001000,
                    b'e' => 0b0010000,
                    b'f' => 0b0100000,
                    b'g' => 0b1000000,
                    _ => unreachable!(),
                }
            })
        })
        .collect()
}

impl Day<'_> for Day08 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            entries: input
                .lines()
                .map(|l| {
                    let (left, right) = split_pair(l, " | ").unwrap();
                    (
                        parse_segments(left)
                            .iter()
                            .copied()
                            .filter(|n| n.count_ones() < 7)
                            .sorted_unstable_by_key(|n| n.count_ones())
                            .collect::<Vec<_>>(),
                        parse_segments(right),
                    )
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.entries
            .iter()
            .map(|(_, right)| {
                right
                    .iter()
                    .filter(|n| UNIQUE_SETS.contains(&(n.count_ones() as u8)))
                    .count()
            })
            .sum::<usize>()
    }

    fn part_2(&self) -> Self::T2 {
        0
    }
}
