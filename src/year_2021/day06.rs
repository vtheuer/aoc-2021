use std::cmp::min;

use fnv::{FnvHashMap, FnvHashSet};

use crate::day::Day;

pub struct Day06 {
    fishes_by_day: [usize; 9],
}

impl Day06 {
    fn simulate(&self, days: usize) -> usize {
        (0..days)
            .fold(self.fishes_by_day.clone(), |fishes_by_day, _| {
                fishes_by_day
                    .iter()
                    .enumerate()
                    .fold([0; 9], |mut new_fishes_by_day, (i, n)| {
                        if i == 0 {
                            new_fishes_by_day[6] += n;
                            new_fishes_by_day[8] += n;
                        } else {
                            new_fishes_by_day[i - 1] += n;
                        }
                        new_fishes_by_day
                    })
            })
            .into_iter()
            .sum()
    }
}

impl Day<'_> for Day06 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            fishes_by_day: input
                .lines()
                .next()
                .unwrap()
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .fold([0; 9], |mut fishes_by_day, n| {
                    fishes_by_day[n] += 1;
                    fishes_by_day
                }),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.simulate(80)
    }

    fn part_2(&self) -> Self::T2 {
        self.simulate(256)
    }
}