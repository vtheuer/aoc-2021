use cmp::max;
use std::cmp;
use std::cmp::min;
use std::convert::identity;

use crate::day::Day;

pub struct Day07 {
    crabs: Vec<isize>,
}

impl Day07 {
    fn count_fuel_with(&self, target: isize, count_fuel: fn(isize) -> isize) -> isize {
        self.crabs
            .iter()
            .map(|&n| count_fuel((n - target).abs()))
            .sum::<isize>()
    }

    fn get_fuel_to_align(&self, count_fuel: fn(isize) -> isize) -> isize {
        let (mut left, mut right) = self
            .crabs
            .iter()
            .fold((isize::MAX, 0), |(left, right), &n| (min(left, n), max(right, n)));
        let mut left_fuel = 0;
        let mut right_fuel = 0;

        while left < right - 1 {
            left_fuel = self.count_fuel_with(left, count_fuel);
            right_fuel = self.count_fuel_with(right, count_fuel);
            if left_fuel > right_fuel {
                left += (right - left) / 2;
            } else {
                right -= (right - left) / 2;
            }
        }

        min(left_fuel, right_fuel)
    }
}

impl Day<'_> for Day07 {
    type T1 = isize;
    type T2 = isize;

    fn new(input: &str) -> Self {
        Self {
            crabs: input
                .lines()
                .next()
                .unwrap()
                .split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.get_fuel_to_align(identity)
    }

    fn part_2(&self) -> Self::T2 {
        // https://fr.wikipedia.org/wiki/1_%2B_2_%2B_3_%2B_4_%2B_%E2%8B%AF
        self.get_fuel_to_align(|distance| distance * (distance + 1) / 2)
    }
}
