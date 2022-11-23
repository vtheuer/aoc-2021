use fnv::{FnvHashMap, FnvHashSet};
use std::convert::identity;

use crate::day::Day;

pub struct Day05<'a> {
    strings: Vec<&'a str>,
}

fn index(w: &[u8]) -> u16 {
    (w[0] - b'a') as u16 * 26 + (w[1] - b'a') as u16
}

impl<'a> Day<'a> for Day05<'a> {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &'a str) -> Self {
        Self {
            strings: input.lines().collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let vowels = b"aeiou";
        let forbidden_pairs: [&[u8]; 4] = [b"ab", b"cd", b"pq", b"xy"];
        self.strings
            .iter()
            .filter(|&&s| {
                s.bytes().filter(|b| vowels.contains(b)).count() >= 3
                    && s.as_bytes().windows(2).any(|w| w[0] == w[1])
                    && !s.as_bytes().windows(2).any(|pair| forbidden_pairs.contains(&pair))
            })
            .count()
    }

    fn part_2(&self) -> Self::T2 {
        self.strings
            .iter()
            .filter(|&&s| {
                s.as_bytes().windows(3).any(|w| w[0] == w[2])
                    && s.as_bytes()
                        .windows(2)
                        .enumerate()
                        .map(|(i, w)| (i as isize, ((w[0] as u16) << 8) | w[1] as u16))
                        .scan(FnvHashMap::default(), |seen, (i, w)| {
                            Some(*seen.entry(w).or_insert(i) < i - 1)
                        })
                        .any(identity)
            })
            .count()
    }
}
