use crate::day::Day;
use std::convert::identity;

pub struct Day06<'a> {
    input: &'a str,
}

impl<'a> Day06<'a> {
    fn find_sequence_of_unique_bytes(&self, len: usize) -> usize {
        self.input
            .as_bytes()
            .windows(len)
            .enumerate()
            .find(|&(_, w)| {
                w.iter()
                    .scan(0u32, |seen, b| {
                        let i = 1 << (b - b'a') as usize;
                        if *seen & i > 0 {
                            Some(false)
                        } else {
                            *seen |= i;
                            Some(true)
                        }
                    })
                    .all(identity)
            })
            .unwrap()
            .0
            + len
    }
}

impl<'a> Day<'a> for Day06<'a> {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &'a str) -> Self {
        Self { input }
    }

    fn part_1(&self) -> Self::T1 {
        self.find_sequence_of_unique_bytes(4)
    }

    fn part_2(&self) -> Self::T2 {
        self.find_sequence_of_unique_bytes(14)
    }
}
